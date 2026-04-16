import { resolveDocsRoot } from "../core/fs.mjs";
import {
  loadIssueDocuments,
  loadTodoDocuments,
} from "../core/parser.mjs";
import { printListResult } from "../core/output.mjs";
import { filterIssueItems, filterTodoItems, limitItems } from "../core/query.mjs";

export function runList(entity, args) {
  const rootDir = resolveDocsRoot(args.root);
  if (entity === "todo") {
    const items = loadTodoDocuments(rootDir).flatMap((document) => document.groups.flatMap((group) => group.items));
    const result = limitItems(filterTodoItems(items, args), args.limit);
    return printListResult(args, entity, result, {
      status: args.status || null,
      owner: args.owner || null,
      tag: args.tag || null,
      text: args.text || null,
      limit: args.limit || null,
    });
  }

  if (entity === "issue") {
    const items = loadIssueDocuments(rootDir).flatMap((document) => document.items);
    const result = limitItems(filterIssueItems(items, args), args.limit);
    return printListResult(args, entity, result, {
      kind: args.kind || null,
      status: args.status || null,
      tag: args.tag || null,
      text: args.text || null,
      limit: args.limit || null,
    });
  }

  throw new Error(`unsupported entity for list: ${entity}`);
}
