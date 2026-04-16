import path from "path";
import { writeText } from "../core/fs.mjs";
import { formatIssueDocument } from "../core/formatter.mjs";
import { aggregateIssueStatus } from "../core/model.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import { buildTodoReferenceIndex, loadStores, findIssueDocumentByItemId, resolveTodoReference } from "../core/store.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { loadBatchItems, requireIssueItemId, runBatchWithPreview, toBatchCommandError } from "../core/batch-command.mjs";

export function runClose(args) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "issue", action: "close" });
  }

  const stores = loadStores(args.root);
  const document = findIssueDocumentByItemId(stores.issueDocuments, args.id);
  if (!document) {
    return printCommandError(args, "Issue not found.", { entity: "issue", action: "close", id: args.id });
  }

  const item = document.items.find((entry) => entry.id === args.id);
  if (item.todo === "none") {
    return printCommandError(args, "Issue close requires a linked todo.", {
      entity: "issue",
      action: "close",
      id: args.id,
    });
  }

  const linkedTodo = resolveTodoReference(stores, item.todo);
  const todoDone = linkedTodo && ["done", "dropped"].includes(linkedTodo.status);
  if (!todoDone && (!args.force || !args.reason)) {
    return printCommandError(args, "Closing an issue with unfinished todo requires --force and --reason.", {
      entity: "issue",
      action: "close",
      id: args.id,
      todo: item.todo,
    });
  }

  item.status = "closed";
  item.closeReason = todoDone ? "none" : args.reason;
  document.meta.status = aggregateIssueStatus(document.items);

  const errors = validateIssueDocument(document, buildTodoReferenceIndex(stores));
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "issue", action: "close", id: args.id });
  }

  writeText(path.resolve(process.cwd(), document.filePath), formatIssueDocument(document));
  return printActionResult(args, `closed issue | ${item.id} | todo=${item.todo}`, {
    entity: "issue",
    action: "close",
    id: item.id,
    todo: item.todo,
    filePath: document.filePath,
  });
}

export function runBatchClose(args) {
  let items;
  try {
    ({ items } = loadBatchItems(args, "issue batch-close"));
  } catch (error) {
    return toBatchCommandError(args, "batch-close", error);
  }

  return runBatchWithPreview({
    args,
    entity: "issue",
    action: "batch-close",
    items,
    mapItemToArgs: (item) => ({
      id: requireIssueItemId(item, "issue batch-close item"),
      force: item.force ?? args.force,
      reason: item.reason ?? args.reason,
    }),
    runSingle: runClose,
    summarize: {
      message: (dryRun) => `${dryRun ? "dry-run issue batch-close" : "batch-closed issue"} | items=${items.length}`,
      data: { ids: items.map((item) => item.id) },
    },
  });
}
