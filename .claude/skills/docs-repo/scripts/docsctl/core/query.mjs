import { toList } from "./args.mjs";
import { splitCsv } from "./formatter.mjs";

function normalize(value) {
  return String(value || "").trim().toLowerCase();
}

function matchesText(item, text) {
  if (!text) {
    return true;
  }
  const haystack = [
    item.id,
    item.title,
    item.status,
    item.owner,
    item.kind,
    item.priority,
    item.source,
    item.dependsOn,
    item.todo,
    item.summary,
    item.direction,
    item.scope,
    ...item.tags,
    ...item.paths,
  ]
    .join(" ")
    .toLowerCase();

  return haystack.includes(text.toLowerCase());
}

function matchesTags(item, args) {
  const tags = toList(args.tag).flatMap((value) => splitCsv(value));
  if (tags.length === 0) {
    return true;
  }
  const itemTags = new Set(item.tags.map(normalize));
  return tags.every((tag) => itemTags.has(normalize(tag)));
}

export function filterTodoItems(items, args) {
  return items
    .filter((item) => !args.status || normalize(item.status) === normalize(args.status))
    .filter((item) => !args.owner || normalize(item.owner) === normalize(args.owner))
    .filter((item) => matchesTags(item, args))
    .filter((item) => matchesText(item, args.text));
}

export function filterIssueItems(items, args) {
  return items
    .filter((item) => !args.kind || args.kind === "all" || normalize(item.kind) === normalize(args.kind))
    .filter((item) => !args.status || normalize(item.status) === normalize(args.status))
    .filter((item) => matchesTags(item, args))
    .filter((item) => matchesText(item, args.text));
}

export function limitItems(items, limitOption) {
  const limit = Number(limitOption || items.length);
  if (!Number.isFinite(limit) || limit <= 0) {
    return [];
  }
  return items.slice(0, limit);
}

export function filterGenericItems(items, args) {
  return items
    .filter((item) => matchesTags(item, args))
    .filter((item) => matchesText(item, args.text));
}
