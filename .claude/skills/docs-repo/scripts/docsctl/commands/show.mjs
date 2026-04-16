import { resolveDocsRoot } from "../core/fs.mjs";
import {
  loadIssueDocuments,
  loadTodoDocuments,
} from "../core/parser.mjs";
import { printCommandError, printShowResult } from "../core/output.mjs";

export function runShow(entity, args) {
  const rootDir = resolveDocsRoot(args.root);
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity, action: "show" });
  }

  const item = entity === "todo"
    ? findTodoItem(rootDir, args.id)
    : findIssueItem(rootDir, args.id);

  if (!item) {
    return printCommandError(args, "Item not found.", { entity, action: "show", id: args.id });
  }

  return printShowResult(args, entity, item);
}

function findTodoItem(rootDir, id) {
  return loadTodoDocuments(rootDir)
    .flatMap((document) => document.groups.flatMap((group) => group.items))
    .find((item) => item.id === id);
}

function findIssueItem(rootDir, id) {
  return loadIssueDocuments(rootDir)
    .flatMap((document) => document.items)
    .find((item) => item.id === id);
}
