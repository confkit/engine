import path from "path";
import { DEFAULT_TODO_OWNER } from "../core/constants.mjs";
import { writeText } from "../core/fs.mjs";
import { formatTodoDocument } from "../core/formatter.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import { findTodoDocumentByDocumentId, loadStores } from "../core/store.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";
import { appendTodoFromInput, buildSingleTodoAppendPayload, hasJsonInput, loadJsonInput } from "../core/todo-manifest.mjs";

export function runAppend(entity, args) {
  if (entity !== "todo") {
    throw new Error(`unsupported entity for append: ${entity}`);
  }
  if (hasJsonInput(args)) {
    return printCommandError(args, "Use todo batch-append for JSON manifest input.", {
      entity: "todo",
      action: "append",
    });
  }
  return appendTodo(args, {
    action: "append",
    payloadLoader: () => buildSingleTodoAppendPayload(args),
  });
}

export function runBatchAppend(entity, args) {
  if (entity !== "todo") {
    throw new Error(`unsupported entity for batch-append: ${entity}`);
  }
  if (!hasJsonInput(args)) {
    return printCommandError(args, "Missing required option: --file or --stdin for todo batch-append", {
      entity: "todo",
      action: "batch-append",
    });
  }
  return appendTodo(args, {
    action: "batch-append",
    payloadLoader: () => loadJsonInput(args, "todo batch-append"),
  });
}

function appendTodo(args, config) {
  if (!args["doc-id"]) {
    return printCommandError(args, "Missing required option: --doc-id", { entity: "todo", action: config.action });
  }

  const stores = loadStores(args.root);
  const document = findTodoDocumentByDocumentId(stores.todoDocuments, args["doc-id"]);
  if (!document) {
    return printCommandError(args, "Todo document not found.", {
      entity: "todo",
      action: config.action,
      documentId: args["doc-id"],
    });
  }

  let payload;
  try {
    payload = config.payloadLoader();
  } catch (error) {
    return printCommandError(args, error.message, { entity: "todo", action: config.action, documentId: args["doc-id"] });
  }

  let addedItems;
  try {
    addedItems = appendTodoFromInput(document, payload, {
      groupTitle: args.group || "G1. 初始拆解",
      owner: args.owner || DEFAULT_TODO_OWNER,
      tags: args.tag,
      source: args.source,
      acceptance: args.acceptance,
      paths: args.path,
      dependsOn: args["depends-on"],
      position: args.position,
      before: args.before,
      after: args.after,
      beforeGroup: args["before-group"],
      afterGroup: args["after-group"],
    });
  } catch (error) {
    return printCommandError(args, error.message, { entity: "todo", action: config.action, documentId: args["doc-id"] });
  }

  const errors = validateTodoDocument(document);
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, {
      entity: "todo",
      action: config.action,
      documentId: args["doc-id"],
    });
  }

  if (!args["dry-run"]) {
    writeText(path.resolve(process.cwd(), document.filePath), formatTodoDocument(document));
  }
  const verb = args["dry-run"]
    ? `dry-run todo ${config.action}`
    : config.action === "batch-append"
      ? "batch-appended todo"
      : "appended todo";
  return printActionResult(args, `${verb} | ${document.meta.id} | items=${addedItems.length}`, {
    entity: "todo",
    action: config.action,
    dryRun: Boolean(args["dry-run"]),
    documentId: document.meta.id,
    ids: addedItems.map((item) => item.id),
    filePath: document.filePath,
  });
}
