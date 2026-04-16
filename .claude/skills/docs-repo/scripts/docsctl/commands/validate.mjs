import { resolveDocsRoot } from "../core/fs.mjs";
import { printValidationResult } from "../core/output.mjs";
import {
  loadIssueDocuments,
  loadTodoDocuments,
} from "../core/parser.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";

export function runValidate(args) {
  const rootDir = resolveDocsRoot(args.root);
  const entity = args.entity || "all";
  const todoDocuments = loadTodoDocuments(rootDir);
  const todoItems = todoDocuments.flatMap((document) => document.groups.flatMap((group) => group.items));
  const todoIndex = new Map(todoItems.map((item) => [item.id, item]));

  const errors = [];

  if (entity === "all" || entity === "todo") {
    errors.push(...todoDocuments.flatMap(validateTodoDocument));
  }

  if (entity === "all" || entity === "issue") {
    const issueDocuments = loadIssueDocuments(rootDir);
    errors.push(...issueDocuments.flatMap((document) => validateIssueDocument(document, todoIndex)));
  }

  return printValidationResult(args, entity, errors);
}
