import { DEFAULT_TODO_OWNER } from "./constants.mjs";
import { hash6, normalizeEnglishSlug, resolveEnglishSlug } from "./id.mjs";
import { readStdinText, readText } from "./fs.mjs";
import { createTodoDocument, createTodoItem, refreshTodoDocument } from "./model.mjs";

const DEFAULT_GROUP_TITLE = "G1. 初始拆解";
const DEFAULT_FIX_GROUP_TITLE = "G1. 问题修复";

export function hasJsonInput(args) {
  return Boolean(args.file || args.stdin);
}

export function loadJsonInput(args, contextLabel) {
  if (args.file && args.stdin) {
    throw new Error(`${contextLabel} cannot use --file and --stdin at the same time.`);
  }

  if (!args.file && !args.stdin) {
    return null;
  }

  const raw = args.file ? readText(args.file) : readStdinText();
  if (!raw.trim()) {
    throw new Error(`${contextLabel} requires non-empty JSON input.`);
  }

  let parsed;
  try {
    parsed = JSON.parse(raw);
  } catch (error) {
    throw new Error(`${contextLabel} received invalid JSON: ${error.message}`);
  }

  if (!parsed || typeof parsed !== "object" || Array.isArray(parsed)) {
    throw new Error(`${contextLabel} JSON must be an object.`);
  }

  return parsed;
}

export function buildTodoDocumentFromInput(payload, defaults = {}) {
  const title = requireText(payload.title ?? defaults.title, "todo title");
  const slug = resolveTodoDocumentSlug(payload, title, defaults);
  const tags = normalizeList(payload.tags ?? payload.tag, normalizeList(defaults.tags, ["todo"]));
  const source = normalizeString(payload.source ?? defaults.source, "none");
  const scope = normalizeString(payload.scope ?? defaults.scope, inferScope(source));
  const verificationTitle = normalizeString(
    payload.verificationTitle ?? payload["verification-title"] ?? defaults.verificationTitle,
    "阶段性验证",
  );
  const verificationLines = normalizeVerificationLines(
    payload.verificationLines ?? payload["verification-lines"] ?? defaults.verificationLines,
  );
  const groups = buildGroupsFromPayload(payload, {
    docSlug: slug,
    groupTitle: defaults.groupTitle || DEFAULT_GROUP_TITLE,
    owner: defaults.owner || DEFAULT_TODO_OWNER,
    tags,
    source,
    acceptance: defaults.acceptance || "待补充",
    paths: normalizeList(defaults.paths),
    dependsOn: normalizeString(defaults.dependsOn, "none"),
  });

  return refreshTodoDocument(createTodoDocument({
    slug,
    title,
    tags,
    source,
    scope,
    groups,
    verificationTitle,
    verificationLines,
  }));
}

export function appendTodoFromInput(document, payload, defaults = {}) {
  const docSlug = extractTodoDocumentSlug(document.meta.id);
  const groups = buildGroupsFromPayload(payload, {
    docSlug,
    groupTitle: defaults.groupTitle || DEFAULT_GROUP_TITLE,
    owner: defaults.owner || DEFAULT_TODO_OWNER,
    tags: normalizeList(payload.tags ?? payload.tag, normalizeList(defaults.tags)),
    source: normalizeString(payload.source ?? defaults.source, document.meta.source || "none"),
    acceptance: defaults.acceptance || "待补充",
    paths: normalizeList(defaults.paths),
    dependsOn: normalizeString(defaults.dependsOn, "none"),
    position: payload.position ?? defaults.position,
    before: payload.before ?? defaults.before,
    after: payload.after ?? defaults.after,
    beforeGroup: payload.beforeGroup ?? payload["before-group"] ?? defaults.beforeGroup,
    afterGroup: payload.afterGroup ?? payload["after-group"] ?? defaults.afterGroup,
  });

  const addedItems = [];
  for (const incomingGroup of groups) {
    let group = document.groups.find((entry) => entry.title === incomingGroup.title);
    if (!group) {
      group = {
        title: incomingGroup.title,
        line: 0,
        items: [],
      };
      insertGroupIntoDocument(document.groups, group, incomingGroup.insert);
    }

    insertItemsIntoGroup(document.groups, group, incomingGroup.items, incomingGroup.insert);
    addedItems.push(...incomingGroup.items);
  }

  if (payload.verificationTitle || payload["verification-title"]) {
    document.verificationTitle = normalizeString(
      payload.verificationTitle ?? payload["verification-title"],
      document.verificationTitle || "阶段性验证",
    );
  }
  if (payload.verificationLines || payload["verification-lines"]) {
    document.verificationLines = normalizeVerificationLines(
      payload.verificationLines ?? payload["verification-lines"],
      document.verificationLines,
    );
  }

  refreshTodoDocument(document);
  return addedItems;
}

export function buildSingleTodoAppendPayload(args) {
  if (!args.title) {
    throw new Error("Missing required option: --title");
  }

  return {
    group: args.group || DEFAULT_GROUP_TITLE,
    position: args.position,
    before: args.before,
    after: args.after,
    beforeGroup: args["before-group"],
    afterGroup: args["after-group"],
    items: [
      {
        title: args.title,
        slug: args.slug,
        owner: args.owner || DEFAULT_TODO_OWNER,
        tags: args.tag,
        paths: args.path,
        source: args.source,
        acceptance: args.acceptance || "待补充",
        dependsOn: args["depends-on"] || "none",
        status: args.status || "todo",
      },
    ],
  };
}

export function defaultFixGroupTitle() {
  return DEFAULT_FIX_GROUP_TITLE;
}

function buildGroupsFromPayload(payload, defaults) {
  const rawGroups = Array.isArray(payload.groups) && payload.groups.length > 0
    ? payload.groups
    : Array.isArray(payload.items) && payload.items.length > 0
      ? [{ title: payload.group || defaults.groupTitle || DEFAULT_GROUP_TITLE, items: payload.items }]
      : null;

  if (!rawGroups) {
    throw new Error("Todo payload must provide groups[] or items[].");
  }

  return rawGroups.map((group, groupIndex) => {
    if (!group || typeof group !== "object" || Array.isArray(group)) {
      throw new Error(`Todo group #${groupIndex + 1} must be an object.`);
    }

    const groupTitle = requireText(group.title ?? `G${groupIndex + 1}. 分组`, `group title #${groupIndex + 1}`);
    if (!Array.isArray(group.items) || group.items.length === 0) {
      throw new Error(`Todo group "${groupTitle}" must contain at least one item.`);
    }

    return {
      title: groupTitle,
      line: 0,
      insert: resolveGroupInsert(group, defaults, groupTitle),
      items: group.items.map((item, itemIndex) => buildTodoItemFromPayload(item, {
        docSlug: defaults.docSlug,
        groupTitle,
        groupIndex,
        itemIndex,
        owner: defaults.owner,
        tags: normalizeList(group.tags ?? group.tag, defaults.tags),
        source: normalizeString(group.source, defaults.source),
        acceptance: group.acceptance ?? defaults.acceptance,
        paths: normalizeList(group.paths ?? group.path, defaults.paths),
        dependsOn: normalizeString(group.dependsOn ?? group["depends-on"], defaults.dependsOn),
      })),
    };
  });
}

function resolveGroupInsert(group, defaults, groupTitle) {
  const before = normalizeOptionalString(group.before ?? defaults.before);
  const after = normalizeOptionalString(group.after ?? defaults.after);
  const beforeGroup = normalizeOptionalString(group.beforeGroup ?? group["before-group"] ?? defaults.beforeGroup);
  const afterGroup = normalizeOptionalString(group.afterGroup ?? group["after-group"] ?? defaults.afterGroup);
  const position = normalizePosition(group.position ?? defaults.position, "end");

  if (before && after) {
    throw new Error(`Todo group "${groupTitle}" cannot use before and after at the same time.`);
  }
  if (beforeGroup && afterGroup) {
    throw new Error(`Todo group "${groupTitle}" cannot use beforeGroup and afterGroup at the same time.`);
  }

  return {
    before,
    after,
    beforeGroup,
    afterGroup,
    position,
  };
}

function buildTodoItemFromPayload(item, defaults) {
  if (!item || typeof item !== "object" || Array.isArray(item)) {
    throw new Error(`Todo item in group "${defaults.groupTitle}" must be an object.`);
  }

  const title = requireText(item.title, `todo item title in "${defaults.groupTitle}"`);
  const itemSlug = resolveTodoItemSlug(item.slug, title, defaults);
  return createTodoItem({
    slug: itemSlug,
    title,
    owner: normalizeString(item.owner, defaults.owner || DEFAULT_TODO_OWNER),
    tags: normalizeList(item.tags ?? item.tag, defaults.tags.length > 0 ? defaults.tags : ["todo"]),
    paths: normalizeList(item.paths ?? item.path, defaults.paths),
    source: normalizeString(item.source, defaults.source || "none"),
    dependsOn: normalizeString(item.dependsOn ?? item["depends-on"], defaults.dependsOn || "none"),
    acceptance: normalizeString(item.acceptance, defaults.acceptance || "待补充"),
    status: normalizeString(item.status, "todo"),
    group: defaults.groupTitle,
  });
}

function resolveTodoDocumentSlug(payload, title, defaults) {
  const providedSlug = payload.slug ?? defaults.slug;
  if (providedSlug !== undefined) {
    return normalizeEnglishSlug(providedSlug, "todo slug");
  }
  return resolveEnglishSlug({ title, entity: "todo" });
}

function resolveTodoItemSlug(slug, title, defaults) {
  if (slug !== undefined) {
    return normalizeEnglishSlug(slug, "todo item slug");
  }

  const plainTitle = String(title || "");
  if (!/[^\x00-\x7F]/.test(plainTitle)) {
    try {
      return normalizeEnglishSlug(plainTitle, "todo item title");
    } catch {
      // Fall through to a doc-scoped hash slug when the title is not a clean slug.
    }
  }

  return `${defaults.docSlug}-${hash6(`${defaults.groupTitle}:${title}:${defaults.groupIndex}:${defaults.itemIndex}`)}`;
}

function extractTodoDocumentSlug(documentId) {
  const match = String(documentId || "").match(/^TODO-DOC-(?<slug>[a-z0-9-]+)-[a-f0-9]{6}$/);
  if (!match?.groups?.slug) {
    throw new Error(`Cannot extract todo document slug from ID: ${documentId}`);
  }
  return match.groups.slug;
}

function insertGroupIntoDocument(groups, group, insert) {
  const beforeIndex = findGroupIndex(groups, insert.beforeGroup);
  if (beforeIndex >= 0) {
    groups.splice(beforeIndex, 0, group);
    return;
  }

  const afterIndex = findGroupIndex(groups, insert.afterGroup);
  if (afterIndex >= 0) {
    groups.splice(afterIndex + 1, 0, group);
    return;
  }

  if (insert.position === "start") {
    groups.unshift(group);
    return;
  }

  groups.push(group);
}

function insertItemsIntoGroup(groups, group, items, insert) {
  for (const item of items) {
    item.group = group.title;
  }

  const anchorId = insert.before || insert.after;
  if (anchorId) {
    const anchorIndex = group.items.findIndex((item) => item.id === anchorId);
    if (anchorIndex === -1) {
      const ownerGroup = groups.find((entry) => entry.items.some((item) => item.id === anchorId));
      if (ownerGroup) {
        throw new Error(`Todo anchor ${anchorId} belongs to group "${ownerGroup.title}", not "${group.title}".`);
      }
      throw new Error(`Todo anchor not found in group "${group.title}": ${anchorId}`);
    }
    const insertIndex = insert.before ? anchorIndex : anchorIndex + 1;
    group.items.splice(insertIndex, 0, ...items);
    return;
  }

  if (insert.position === "start") {
    group.items.unshift(...items);
    return;
  }

  group.items.push(...items);
}

function findGroupIndex(groups, title) {
  if (!title) {
    return -1;
  }

  const index = groups.findIndex((entry) => entry.title === title);
  if (index === -1) {
    throw new Error(`Todo group anchor not found: ${title}`);
  }
  return index;
}

function normalizeVerificationLines(value, fallback = ["- 待补充"]) {
  if (value === undefined) {
    return fallback;
  }

  if (Array.isArray(value)) {
    const lines = value.map((line) => String(line));
    return lines.length > 0 ? lines : fallback;
  }

  const text = String(value).trim();
  if (!text) {
    return fallback;
  }
  return text.split(/\r?\n/);
}

function normalizeList(value, fallback = []) {
  if (value === undefined || value === null || value === "") {
    return [...fallback];
  }

  const raw = Array.isArray(value) ? value : [value];
  const result = raw
    .flatMap((entry) => String(entry).split(/[，,]/))
    .map((entry) => entry.trim())
    .filter(Boolean);

  return result.length > 0 ? result : [...fallback];
}

function normalizeOptionalString(value) {
  if (value === undefined || value === null || value === "") {
    return undefined;
  }
  return String(value).trim() || undefined;
}

function normalizePosition(value, fallback = "end") {
  const resolved = normalizeOptionalString(value) || fallback;
  if (!["start", "end"].includes(resolved)) {
    throw new Error(`Unsupported position: ${resolved}. Use start or end.`);
  }
  return resolved;
}

function normalizeString(value, fallback = "") {
  if (value === undefined || value === null || value === "") {
    return fallback;
  }
  return String(value);
}

function requireText(value, fieldLabel) {
  const text = String(value || "").trim();
  if (!text) {
    throw new Error(`Missing required field: ${fieldLabel}.`);
  }
  return text;
}

function inferScope(source) {
  return source && source !== "none" ? "fix" : "feature";
}
