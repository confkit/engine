import path from "path";
import { DEFAULT_TODO_OWNER, DOCS_DIRECTORIES } from "../core/constants.mjs";
import { writeText } from "../core/fs.mjs";
import { formatIssueDocument, formatTodoDocument } from "../core/formatter.mjs";
import { extractSlugFromId, normalizeEnglishSlug } from "../core/id.mjs";
import { aggregateIssueStatus, createTodoDocument, refreshTodoDocument } from "../core/model.mjs";
import { printActionResult, printCommandError, printSchemaErrors } from "../core/output.mjs";
import { buildTodoReferenceIndex, loadStores, findIssueDocumentByItemId } from "../core/store.mjs";
import { validateIssueDocument } from "../schemas/issue.mjs";
import { validateTodoDocument } from "../schemas/todo.mjs";
import {
  buildTodoDocumentFromInput,
  defaultFixGroupTitle,
  hasJsonInput,
  loadJsonInput,
} from "../core/todo-manifest.mjs";

export function runPlan(args) {
  if (hasJsonInput(args)) {
    return printCommandError(args, "Use issue batch-plan for JSON manifest input.", {
      entity: "issue",
      action: "plan",
    });
  }
  return planIssue(args, {
    action: "plan",
    buildDocument: ({ issueItem, slug }) =>
      refreshTodoDocument(createTodoDocument({
        slug,
        title: args["doc-title"] || issueItem.title,
        itemTitle: args.title || issueItem.title,
        tags: issueItem.tags,
        source: issueItem.id,
        scope: "fix",
        owner: args.owner || DEFAULT_TODO_OWNER,
        acceptance: args.acceptance || issueItem.direction || "待补充",
        groupTitle: args.group || defaultFixGroupTitle(),
        paths: [],
        dependsOn: "none",
      })),
  });
}

export function runBatchPlan(args) {
  if (!hasJsonInput(args)) {
    return printCommandError(args, "Missing required option: --file or --stdin for issue batch-plan", {
      entity: "issue",
      action: "batch-plan",
    });
  }
  return planIssue(args, {
    action: "batch-plan",
    buildDocument: ({ issueItem, slug }) =>
      buildTodoDocumentFromInput(loadJsonInput(args, "issue batch-plan"), {
        title: args["doc-title"] || issueItem.title,
        slug,
        tags: issueItem.tags,
        source: issueItem.id,
        scope: "fix",
        owner: args.owner || DEFAULT_TODO_OWNER,
        acceptance: args.acceptance || issueItem.direction || "待补充",
        groupTitle: args.group || defaultFixGroupTitle(),
        paths: [],
        dependsOn: "none",
      }),
  });
}

function planIssue(args, config) {
  if (!args.id) {
    return printCommandError(args, "Missing required option: --id", { entity: "issue", action: config.action });
  }

  const stores = loadStores(args.root);
  const issueDocument = findIssueDocumentByItemId(stores.issueDocuments, args.id);
  if (!issueDocument) {
    return printCommandError(args, "Issue not found.", { entity: "issue", action: config.action, id: args.id });
  }

  const issueItem = issueDocument.items.find((entry) => entry.id === args.id);
  if (issueItem.todo !== "none") {
    return printCommandError(args, `Issue already linked to todo: ${issueItem.todo}`, {
      entity: "issue",
      action: config.action,
      id: args.id,
      todo: issueItem.todo,
    });
  }

  let slug;
  try {
    slug = args.slug ? normalizeEnglishSlug(args.slug, "--slug") : extractSlugFromId(issueItem.id);
  } catch (error) {
    return printCommandError(args, error.message, { entity: "issue", action: config.action, id: args.id });
  }

  let todoDocument;
  try {
    todoDocument = config.buildDocument({ issueItem, slug });
  } catch (error) {
    return printCommandError(args, error.message, { entity: "issue", action: config.action, id: args.id });
  }

  const todoPath = path.join(stores.rootDir, DOCS_DIRECTORIES.todo, `${todoDocument.title}.md`);
  todoDocument.filePath = path.relative(process.cwd(), todoPath);
  todoDocument.ref = `${todoDocument.filePath}#L1`;

  issueItem.todo = todoDocument.meta.id;
  issueItem.status = "planned";
  issueItem.closeReason = "none";
  issueDocument.meta.status = aggregateIssueStatus(issueDocument.items);

  const todoErrors = validateTodoDocument(todoDocument);
  const todoLookup = buildTodoReferenceIndex({
    ...stores,
    todoDocuments: [...stores.todoDocuments, todoDocument],
    todoItems: [...stores.todoItems, ...todoDocument.groups.flatMap((group) => group.items)],
  });
  const issueErrors = validateIssueDocument(issueDocument, todoLookup);
  const errors = [...todoErrors, ...issueErrors];
  if (errors.length > 0) {
    return printSchemaErrors(args, errors, {
      entity: "issue",
      action: config.action,
      id: args.id,
      todo: todoDocument.meta.id,
    });
  }

  if (!args["dry-run"]) {
    writeText(todoPath, formatTodoDocument(todoDocument));
    writeText(path.resolve(process.cwd(), issueDocument.filePath), formatIssueDocument(issueDocument));
  }
  const verb = args["dry-run"]
    ? `dry-run issue ${config.action}`
    : config.action === "batch-plan"
      ? "batch-planned issue"
      : "planned issue";
  return printActionResult(args, `${verb} | ${issueItem.id} | todo=${todoDocument.meta.id}`, {
    entity: "issue",
    action: config.action,
    dryRun: Boolean(args["dry-run"]),
    id: issueItem.id,
    todo: todoDocument.meta.id,
    ids: todoDocument.groups.flatMap((group) => group.items.map((item) => item.id)),
    issueFilePath: issueDocument.filePath,
    todoFilePath: todoDocument.filePath,
  });
}
