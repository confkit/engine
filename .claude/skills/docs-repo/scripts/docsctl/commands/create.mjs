import path from "path";
import { DEFAULT_TODO_OWNER, DOCS_DIRECTORIES } from "../core/constants.mjs";
import { writeText } from "../core/fs.mjs";
import { formatIssueDocument, formatTodoDocument } from "../core/formatter.mjs";
import { resolveEnglishSlug } from "../core/id.mjs";
import {
  createIssueBucketDocument,
  createIssueItem,
  createTodoDocument,
  aggregateIssueStatus,
  refreshTodoDocument,
} from "../core/model.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";
import { loadStores, nextIssueBucketPath } from "../core/store.mjs";
import { hasJsonInput, loadJsonInput, buildTodoDocumentFromInput } from "../core/todo-manifest.mjs";

export function runCreate(entity, args) {
  if (hasJsonInput(args)) {
    return printCommandError(args, `Use ${entity} batch-create for JSON manifest input.`, {
      entity,
      action: "create",
    });
  }
  if (entity === "todo") return createTodo(args);
  if (entity === "issue") return createIssue(args);
  throw new Error(`unsupported entity for create: ${entity}`);
}

export function runBatchCreate(entity, args) {
  if (!hasJsonInput(args)) {
    return printCommandError(args, `Missing required option: --file or --stdin for ${entity} batch-create`, {
      entity,
      action: "batch-create",
    });
  }
  if (entity === "todo") return createTodoBatch(args);
  if (entity === "issue") return createIssueBatch(args);
  throw new Error(`unsupported entity for batch-create: ${entity}`);
}

function createTodo(args) {
  try {
    const document = createSingleTodoDocument(args);
    return writeTodoDocument(args, document, "create");
  } catch (error) {
    return printCommandError(args, error.message, { entity: "todo", action: "create" });
  }
}

function createTodoBatch(args) {
  try {
    const document = buildTodoDocumentFromInput(loadJsonInput(args, "todo batch-create"), {
      title: args.title,
      slug: args.slug,
      tags: normalizeList(args.tag, ["todo"]),
      source: args.source || "none",
      scope: args.scope || inferScope(args.source),
      owner: args.owner || DEFAULT_TODO_OWNER,
      acceptance: args.acceptance || "待补充",
      groupTitle: args.group || "G1. 初始拆解",
      paths: normalizeList(args.path),
      dependsOn: args["depends-on"] || "none",
    });
    return writeTodoDocument(args, document, "batch-create");
  } catch (error) {
    return printCommandError(args, error.message, { entity: "todo", action: "batch-create" });
  }
}

function writeTodoDocument(args, document, action) {
  const stores = loadStores(args.root);
  const filePath = path.join(stores.rootDir, DOCS_DIRECTORIES.todo, `${document.title}.md`);
  document.filePath = path.relative(process.cwd(), filePath);
  document.ref = `${document.filePath}#L1`;

  const errors = validateTodoDocument(document);
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "todo", action });
  }

  if (!args["dry-run"]) {
    writeText(filePath, formatTodoDocument(document));
  }

  const createdItems = document.groups.flatMap((group) => group.items.map((item) => item.id));
  const verb = args["dry-run"]
    ? `dry-run todo ${action}`
    : action === "batch-create"
      ? "batch-created todo"
      : "created todo";
  return printActionResult(args, `${verb} | ${document.meta.id} | items=${createdItems.length} | ${document.filePath}`, {
    entity: "todo",
    action,
    dryRun: Boolean(args["dry-run"]),
    documentId: document.meta.id,
    ids: createdItems,
    filePath: document.filePath,
  });
}

function createSingleTodoDocument(args) {
  if (!args.title) {
    throw new Error("Missing required option: --title");
  }

  const slug = resolveCreateSlug(args, "todo");
  return refreshTodoDocument(createTodoDocument({
    slug,
    title: args.title,
    itemTitle: args["item-title"] || args.title,
    tags: normalizeList(args.tag, ["todo"]),
    source: args.source || "none",
    scope: args.scope || inferScope(args.source),
    owner: args.owner || DEFAULT_TODO_OWNER,
    acceptance: args.acceptance || "待补充",
    groupTitle: args.group || "G1. 初始拆解",
    paths: normalizeList(args.path),
    dependsOn: args["depends-on"] || "none",
  }));
}

function createIssue(args) {
  if (!args.kind || !["bug", "optimization"].includes(args.kind)) {
    return printCommandError(args, "Missing or invalid option: --kind bug|optimization", {
      entity: "issue",
      action: "create",
    });
  }
  if (!args.title) {
    return printCommandError(args, "Missing required option: --title", { entity: "issue", action: "create" });
  }

  let slug;
  try {
    slug = resolveCreateSlug(args, "issue");
  } catch (error) {
    return printCommandError(args, error.message, { entity: "issue", action: "create" });
  }

  const stores = loadStores(args.root);
  const filePath = nextIssueBucketPath(stores.rootDir, args.kind, stores.issueDocuments);
  const workspaceFilePath = path.relative(process.cwd(), filePath);
  const document = stores.issueDocuments.find((entry) => entry.filePath === workspaceFilePath)
    || createIssueBucketDocument(filePath, args.kind);

  const item = createIssueItem({
    kind: args.kind,
    slug,
    title: args.title,
    priority: args.priority || "medium",
    tags: normalizeList(args.tag, [args.kind]),
    summary: args.summary || args.title,
    cause: args.cause || "",
    direction: args.direction || "待补充",
    scope: args.scope || "待补充",
  });

  document.filePath = workspaceFilePath;
  document.ref = `${workspaceFilePath}#L1`;
  document.items.push(item);
  document.meta.status = aggregateIssueStatus(document.items);

  const errors = validateIssueDocument(document, stores.todoIndex);
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "issue", action: "create" });
  }

  writeText(filePath, formatIssueDocument(document));
  return printActionResult(args, `created issue | ${item.id} | ${workspaceFilePath}`, {
    entity: "issue",
    action: "create",
    id: item.id,
    filePath: workspaceFilePath,
  });
}

function createIssueBatch(args) {
  let payload;
  try {
    payload = loadJsonInput(args, "issue batch-create");
  } catch (error) {
    return printCommandError(args, error.message, { entity: "issue", action: "batch-create" });
  }

  const itemEntries = Array.isArray(payload.items)
    ? payload.items
    : Array.isArray(payload.issues)
      ? payload.issues
      : null;
  if (!itemEntries || itemEntries.length === 0) {
    return printCommandError(args, "issue batch-create requires items[] or issues[].", {
      entity: "issue",
      action: "batch-create",
    });
  }

  const stores = loadStores(args.root);
  const workingDocuments = stores.issueDocuments.map((document) => ({
    ...document,
    meta: { ...document.meta },
    items: document.items.map((item) => ({ ...item, tags: [...item.tags] })),
  }));
  const touchedDocuments = new Map();
  const createdIds = [];

  try {
    for (const [index, rawItem] of itemEntries.entries()) {
      if (!rawItem || typeof rawItem !== "object" || Array.isArray(rawItem)) {
        throw new Error(`issue batch-create item #${index + 1} must be an object.`);
      }

      const kind = rawItem.kind || payload.kind;
      if (!["bug", "optimization"].includes(kind)) {
        throw new Error(`issue batch-create item #${index + 1} requires kind bug|optimization.`);
      }
      if (!rawItem.title) {
        throw new Error(`issue batch-create item #${index + 1} requires title.`);
      }

      const slug = resolveCreateSlug({
        slug: rawItem.slug,
        title: rawItem.title,
      }, "issue");

      const filePath = nextIssueBucketPath(stores.rootDir, kind, workingDocuments);
      const workspaceFilePath = path.relative(process.cwd(), filePath);
      let document = workingDocuments.find((entry) => entry.filePath === workspaceFilePath);
      if (!document) {
        document = createIssueBucketDocument(filePath, kind);
        document.filePath = workspaceFilePath;
        document.ref = `${workspaceFilePath}#L1`;
        workingDocuments.push(document);
      }

      const item = createIssueItem({
        kind,
        slug,
        title: rawItem.title,
        priority: rawItem.priority || payload.priority || "medium",
        tags: normalizeList(rawItem.tags ?? rawItem.tag, normalizeList(payload.tags ?? payload.tag, [kind])),
        summary: rawItem.summary || rawItem.title,
        cause: rawItem.cause || "",
        direction: rawItem.direction || payload.direction || "待补充",
        scope: rawItem.scope || payload.scope || "待补充",
      });

      document.items.push(item);
      document.meta.status = aggregateIssueStatus(document.items);
      touchedDocuments.set(document.filePath, document);
      createdIds.push(item.id);
    }
  } catch (error) {
    return printCommandError(args, error.message, { entity: "issue", action: "batch-create" });
  }

  const errors = [];
  for (const document of touchedDocuments.values()) {
    errors.push(...validateIssueDocument(document, stores.todoIndex));
  }
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, { entity: "issue", action: "batch-create" });
  }

  if (!args["dry-run"]) {
    for (const document of touchedDocuments.values()) {
      writeText(path.resolve(process.cwd(), document.filePath), formatIssueDocument(document));
    }
  }

  return printActionResult(args, `${args["dry-run"] ? "dry-run issue batch-create" : "batch-created issue"} | items=${createdIds.length}`, {
    entity: "issue",
    action: "batch-create",
    dryRun: Boolean(args["dry-run"]),
    ids: createdIds,
    filePaths: Array.from(touchedDocuments.values()).map((document) => document.filePath),
  });
}

function normalizeList(value, fallback = []) {
  const raw = value === undefined ? [] : Array.isArray(value) ? value : [value];
  const result = raw
    .flatMap((entry) => String(entry).split(/[，,]/))
    .map((entry) => entry.trim())
    .filter(Boolean);
  return result.length > 0 ? result : fallback;
}

function inferScope(source) {
  return source && source !== "none" ? "fix" : "feature";
}

function resolveCreateSlug(args, entity) {
  return resolveEnglishSlug({ slug: args.slug, title: args.title, entity });
}
