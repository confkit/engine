import path from "path";
import { DEFAULT_TODO_OWNER } from "./constants.mjs";
import { createDocumentId, createFileStem, createId, normalizeEnglishSlug } from "./id.mjs";

export function aggregateTodoStatus(groups) {
  const items = groups.flatMap((group) => group.items);
  if (items.some((item) => item.status === "doing")) return "doing";
  if (items.some((item) => item.status === "blocked")) return "blocked";
  if (items.some((item) => item.status === "todo")) return "todo";
  if (items.length > 0 && items.every((item) => item.status === "dropped")) return "dropped";
  return "done";
}

export function aggregateIssueStatus(items) {
  if (items.some((item) => item.status === "doing")) return "doing";
  if (items.some((item) => item.status === "planned")) return "planned";
  if (items.some((item) => item.status === "open")) return "open";
  if (items.length > 0 && items.every((item) => item.status === "dropped")) return "dropped";
  return "closed";
}

export function createTodoItem({
  slug,
  title,
  owner,
  tags,
  paths,
  source,
  dependsOn,
  acceptance,
  status = "todo",
  group = "",
}) {
  return {
    entity: "todo",
    kind: "todo",
    id: createId("TODO", slug),
    owner: owner || DEFAULT_TODO_OWNER,
    title,
    status,
    tags,
    paths: paths || [],
    source: source || "none",
    dependsOn: dependsOn || "none",
    acceptance: acceptance || "待补充",
    group,
    filePath: "",
    line: 0,
    ref: "",
    rawFields: {},
  };
}

export function createTodoDocument({
  slug,
  title,
  itemTitle,
  tags,
  source,
  scope,
  owner,
  acceptance,
  groupTitle,
  paths,
  dependsOn,
  groups,
  verificationTitle,
  verificationLines,
}) {
  const normalizedSlug = normalizeEnglishSlug(slug, "slug");
  const documentId = createDocumentId("TODO", normalizedSlug);
  const normalizedGroups = groups && groups.length > 0
    ? groups
    : [
        {
          title: groupTitle || "G1. 初始拆解",
          line: 0,
          items: [
            createTodoItem({
              slug: normalizedSlug,
              title: itemTitle || title,
              owner: owner || DEFAULT_TODO_OWNER,
              tags,
              paths: paths || [],
              source: source || "none",
              dependsOn: dependsOn || "none",
              acceptance: acceptance || "待补充",
              group: groupTitle || "G1. 初始拆解",
            }),
          ],
        },
      ];

  return {
    entity: "todo",
    title: createFileStem(normalizedSlug, "todo-doc-heading"),
    meta: {
      type: "todo",
      id: documentId,
      title: title || itemTitle,
      status: "todo",
      tags: tags.join(", "),
      source: source || "none",
      scope: scope || "feature",
    },
    groups: normalizedGroups,
    verificationTitle: verificationTitle || "阶段性验证",
    verificationLines: verificationLines || ["- 待补充"],
    filePath: "",
    ref: "",
  };
}

export function refreshTodoDocument(document) {
  const tags = new Set();
  for (const group of document.groups) {
    for (const item of group.items) {
      for (const tag of item.tags || []) {
        if (tag) {
          tags.add(tag);
        }
      }
    }
  }
  document.meta.status = aggregateTodoStatus(document.groups);
  document.meta.tags = Array.from(tags).join(", ");
  return document;
}

export function createIssueBucketDocument(filePath, kind) {
  const bucketName = path.basename(filePath, ".md");
  return {
    entity: "issue",
    title: bucketName,
    meta: {
      type: "issue",
      id: createDocumentId("ISSUE", `${kind}-${bucketName}`),
      title: `${kind} tracking ${bucketName}`,
      kind,
      status: "open",
      tags: kind,
    },
    items: [],
    filePath: "",
    ref: "",
  };
}

export function createIssueItem({ kind, slug, title, priority, tags, summary, cause, direction, scope }) {
  return {
    entity: "issue",
    kind,
    id: createId(kind === "bug" ? "BUG" : "OPT", slug),
    title,
    status: "open",
    priority: priority || "medium",
    tags,
    todo: "none",
    closeReason: "none",
    summary: summary || title,
    cause: cause || "",
    direction: direction || "待补充",
    scope: scope || "待补充",
    filePath: "",
    line: 0,
    ref: "",
    rawFields: {},
  };
}
