import path from "path";
import { removeFile, writeText } from "../core/fs.mjs";
import { formatIssueDocument, formatTodoDocument } from "../core/formatter.mjs";
import { aggregateIssueStatus, refreshTodoDocument } from "../core/model.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import {
  loadStores,
  findIssueDocumentByItemId,
  findTodoDocumentByItemId,
  resolveTodoReference,
} from "../core/store.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";
import { loadBatchItems, requireIssueItemId, runBatchWithPreview, toBatchCommandError } from "../core/batch-command.mjs";

export function runDelete(entity, args) {
  if (entity === "todo") return deleteTodo(args);
  if (entity === "issue") return deleteIssue(args);
  throw new Error(`unsupported entity for delete: ${entity}`);
}

export function runBatchDelete(entity, args) {
  if (entity !== "issue") {
    throw new Error(`unsupported entity for batch-delete: ${entity}`);
  }

  let items;
  try {
    ({ items } = loadBatchItems(args, "issue batch-delete"));
  } catch (error) {
    return toBatchCommandError(args, "batch-delete", error);
  }

  return runBatchWithPreview({
    args,
    entity: "issue",
    action: "batch-delete",
    items,
    mapItemToArgs: (item) => ({
      id: requireIssueItemId(item, "issue batch-delete item"),
      force: item.force ?? args.force,
      reason: item.reason ?? args.reason,
    }),
    runSingle: deleteIssue,
    summarize: {
      message: (dryRun) => `${dryRun ? "dry-run issue batch-delete" : "batch-deleted issue"} | items=${items.length}`,
      data: { ids: items.map((item) => item.id) },
    },
  });
}

function deleteTodo(args) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "todo", action: "delete" });
  }

  const stores = loadStores(args.root);
  const document = findTodoDocumentByItemId(stores.todoDocuments, args.id);
  if (!document) {
    return printCommandError(args, "Todo item not found.", { entity: "todo", action: "delete", id: args.id });
  }

  const item = document.groups.flatMap((group) => group.items).find((entry) => entry.id === args.id);
  const remainingItemCount = document.groups.flatMap((group) => group.items).length - 1;
  if (item.source !== "none" && (!args.force || !args.reason)) {
    return printCommandError(args, "Deleting a linked todo requires --force and --reason.", {
      entity: "todo",
      action: "delete",
      id: args.id,
      source: item.source,
    });
  }

  if (item.source !== "none") {
    const issueDocument = findIssueDocumentByItemId(stores.issueDocuments, item.source);
    if (issueDocument) {
      const issueItem = issueDocument.items.find((entry) => entry.id === item.source);
      const linkedByDocument = issueItem.todo === document.meta.id;
      const linkedByItem = issueItem.todo === item.id;
      if (linkedByItem || (linkedByDocument && remainingItemCount <= 0)) {
        issueItem.todo = "none";
        if (["planned", "doing", "closed"].includes(issueItem.status)) {
          issueItem.status = "open";
        }
        issueItem.closeReason = "none";
        issueDocument.meta.status = aggregateIssueStatus(issueDocument.items);
        const issueErrors = validateIssueDocument(issueDocument, new Map());
        if (issueErrors.length === 0) {
          writeText(path.resolve(process.cwd(), issueDocument.filePath), formatIssueDocument(issueDocument));
        }
      }
    }
  }

  for (const group of document.groups) {
    group.items = group.items.filter((entry) => entry.id !== args.id);
  }
  document.groups = document.groups.filter((group) => group.items.length > 0);

  const absoluteFilePath = path.resolve(process.cwd(), document.filePath);
  if (document.groups.length === 0) {
    removeFile(absoluteFilePath);
    return printActionResult(args, `deleted todo | ${args.id} | removed document`, {
      entity: "todo",
      action: "delete",
      id: args.id,
      filePath: document.filePath,
      removedDocument: true,
    });
  }

  refreshTodoDocument(document);
  const errors = validateTodoDocument(document);
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "todo", action: "delete", id: args.id });
  }

  writeText(absoluteFilePath, formatTodoDocument(document));
  return printActionResult(args, `deleted todo | ${args.id}`, {
    entity: "todo",
    action: "delete",
    id: args.id,
    filePath: document.filePath,
    removedDocument: false,
  });
}

function deleteIssue(args) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "issue", action: "delete" });
  }

  const stores = loadStores(args.root);
  const document = findIssueDocumentByItemId(stores.issueDocuments, args.id);
  if (!document) {
    return printCommandError(args, "Issue not found.", { entity: "issue", action: "delete", id: args.id });
  }

  const item = document.items.find((entry) => entry.id === args.id);
  if (item.todo !== "none") {
    const linkedTodo = resolveTodoReference(stores, item.todo);
    if (linkedTodo && !["done", "dropped"].includes(linkedTodo.status) && (!args.force || !args.reason)) {
      return printCommandError(args, "Deleting an issue with unfinished todo requires --force and --reason.", {
        entity: "issue",
        action: "delete",
        id: args.id,
        todo: item.todo,
      });
    }
    if (linkedTodo) {
      const todoResult = linkedTodo.type === "document"
        ? deleteTodoDocumentByReference(stores, linkedTodo.document, args)
        : deleteTodo({
            ...args,
            id: item.todo,
            root: args.root,
            force: true,
            reason: args.reason || "cascade delete from issue",
            silent: true,
          });
      if (todoResult !== 0) {
        return todoResult;
      }
    }
  }

  document.items = document.items.filter((entry) => entry.id !== args.id);
  const absoluteFilePath = path.resolve(process.cwd(), document.filePath);
  if (document.items.length === 0) {
    removeFile(absoluteFilePath);
    return printActionResult(args, `deleted issue | ${args.id} | removed document`, {
      entity: "issue",
      action: "delete",
      id: args.id,
      filePath: document.filePath,
      removedDocument: true,
    });
  }

  document.meta.status = aggregateIssueStatus(document.items);
  writeText(absoluteFilePath, formatIssueDocument(document));
  return printActionResult(args, `deleted issue | ${args.id}`, {
    entity: "issue",
    action: "delete",
    id: args.id,
    filePath: document.filePath,
    removedDocument: false,
  });
}

function deleteTodoDocumentByReference(stores, document, args) {
  for (const group of document.groups) {
    for (const item of group.items) {
      if (item.source === "none") {
        continue;
      }
      const issueDocument = findIssueDocumentByItemId(stores.issueDocuments, item.source);
      if (!issueDocument) {
        continue;
      }
      const issueItem = issueDocument.items.find((entry) => entry.id === item.source);
      if (!issueItem || issueItem.todo !== document.meta.id) {
        continue;
      }
      issueItem.todo = "none";
      if (["planned", "doing", "closed"].includes(issueItem.status)) {
        issueItem.status = "open";
      }
      issueItem.closeReason = "none";
      issueDocument.meta.status = aggregateIssueStatus(issueDocument.items);
      const issueErrors = validateIssueDocument(issueDocument, new Map());
      if (issueErrors.length === 0) {
        writeText(path.resolve(process.cwd(), issueDocument.filePath), formatIssueDocument(issueDocument));
      }
    }
  }

  removeFile(path.resolve(process.cwd(), document.filePath));
  return 0;
}
