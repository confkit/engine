import path from "path";
import { writeText } from "../core/fs.mjs";
import { formatIssueDocument, formatTodoDocument } from "../core/formatter.mjs";
import { aggregateIssueStatus, refreshTodoDocument } from "../core/model.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import { buildTodoReferenceIndex, loadStores, findIssueDocumentByItemId, findTodoDocumentByItemId } from "../core/store.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";
import { loadBatchItems, requireIssueItemId, runBatchWithPreview, toBatchCommandError } from "../core/batch-command.mjs";

export function runUpdate(entity, args) {
  if (entity === "todo") return updateTodo(args);
  if (entity === "issue") return updateIssue(args);
  throw new Error(`unsupported entity for update: ${entity}`);
}

export function runBatchUpdate(entity, args) {
  if (entity !== "issue") {
    throw new Error(`unsupported entity for batch-update: ${entity}`);
  }

  let items;
  try {
    ({ items } = loadBatchItems(args, "issue batch-update"));
  } catch (error) {
    return toBatchCommandError(args, "batch-update", error);
  }

  return runBatchWithPreview({
    args,
    entity: "issue",
    action: "batch-update",
    items,
    mapItemToArgs: (item) => ({
      id: requireIssueItemId(item, "issue batch-update item"),
      status: item.status,
      title: item.title,
      priority: item.priority,
      tag: item.tags ?? item.tag,
      todo: item.todo,
      "close-reason": item["close-reason"] ?? item.closeReason,
      summary: item.summary,
      cause: item.cause,
      direction: item.direction,
      scope: item.scope,
    }),
    runSingle: updateIssue,
    summarize: {
      message: (dryRun) => `${dryRun ? "dry-run issue batch-update" : "batch-updated issue"} | items=${items.length}`,
      data: { ids: items.map((item) => item.id) },
    },
  });
}

function updateTodo(args) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "todo", action: "update" });
  }

  const stores = loadStores(args.root);
  const document = findTodoDocumentByItemId(stores.todoDocuments, args.id);
  if (!document) {
    return printCommandError(args, "Todo item not found.", { entity: "todo", action: "update", id: args.id });
  }

  const item = document.groups.flatMap((group) => group.items).find((entry) => entry.id === args.id);
  if (args.status) item.status = args.status;
  if (args.title) item.title = args.title;
  if (args.owner) item.owner = args.owner;
  if (args.tag) item.tags = normalizeList(args.tag);
  if (args.path) item.paths = normalizeList(args.path);
  if (args.source) item.source = args.source;
  if (args["depends-on"]) item.dependsOn = args["depends-on"];
  if (args.acceptance) item.acceptance = args.acceptance;

  refreshTodoDocument(document);
  const errors = validateTodoDocument(document);
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "todo", action: "update", id: args.id });
  }

  writeText(path.resolve(process.cwd(), document.filePath), formatTodoDocument(document));
  return printActionResult(args, `updated todo | ${item.id} | status=${item.status}`, {
    entity: "todo",
    action: "update",
    id: item.id,
    status: item.status,
    filePath: document.filePath,
  });
}

function updateIssue(args) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "issue", action: "update" });
  }

  const stores = loadStores(args.root);
  const document = findIssueDocumentByItemId(stores.issueDocuments, args.id);
  if (!document) {
    return printCommandError(args, "Issue item not found.", { entity: "issue", action: "update", id: args.id });
  }

  const item = document.items.find((entry) => entry.id === args.id);
  if (args.status) item.status = args.status;
  if (args.title) item.title = args.title;
  if (args.priority) item.priority = args.priority;
  if (args.tag) item.tags = normalizeList(args.tag);
  if (args.todo) item.todo = args.todo;
  if (args["close-reason"]) item.closeReason = args["close-reason"];
  if (args.summary) item.summary = args.summary;
  if (args.cause !== undefined) item.cause = args.cause;
  if (args.direction) item.direction = args.direction;
  if (args.scope) item.scope = args.scope;

  document.meta.status = aggregateIssueStatus(document.items);
  const errors = validateIssueDocument(document, buildTodoReferenceIndex(stores));
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "issue", action: "update", id: args.id });
  }

  writeText(path.resolve(process.cwd(), document.filePath), formatIssueDocument(document));
  return printActionResult(args, `updated issue | ${item.id} | status=${item.status}`, {
    entity: "issue",
    action: "update",
    id: item.id,
    status: item.status,
    filePath: document.filePath,
  });
}

function normalizeList(value) {
  return (Array.isArray(value) ? value : [value])
    .flatMap((entry) => String(entry).split(/[，,]/))
    .map((entry) => entry.trim())
    .filter(Boolean);
}
