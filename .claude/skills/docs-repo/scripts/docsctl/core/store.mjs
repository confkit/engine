import path from "path";
import {
  DEFAULT_DOCS_ROOT,
  DOCS_DIRECTORIES,
  ISSUE_BUCKETS,
  ISSUE_BUCKET_ITEM_LIMIT,
} from "./constants.mjs";
import {
  loadIssueDocuments,
  loadTodoDocuments,
} from "./parser.mjs";
import { resolveDocsRoot, toWorkspacePath, walkMarkdownFiles } from "./fs.mjs";

export function loadStores(rootOption) {
  const rootDir = resolveDocsRoot(rootOption);
  const todoDocuments = loadTodoDocuments(rootDir);
  const issueDocuments = loadIssueDocuments(rootDir);

  const todoItems = todoDocuments.flatMap((document) =>
    document.groups.flatMap((group) =>
      group.items.map((item) => ({ ...item, documentRef: document.filePath })),
    ),
  );
  const issueItems = issueDocuments.flatMap((document) =>
    document.items.map((item) => ({ ...item, documentRef: document.filePath })),
  );

  return {
    rootDir,
    todoDocuments,
    issueDocuments,
    todoItems,
    issueItems,
    todoDocumentIndex: new Map(todoDocuments.map((document) => [document.meta.id, document])),
    todoIndex: new Map(todoItems.map((item) => [item.id, item])),
    issueIndex: new Map(issueItems.map((item) => [item.id, item])),
  };
}

export function findTodoDocumentByItemId(documents, itemId) {
  return documents.find((document) =>
    document.groups.some((group) => group.items.some((item) => item.id === itemId)),
  );
}

export function findIssueDocumentByItemId(documents, itemId) {
  return documents.find((document) => document.items.some((item) => item.id === itemId));
}

export function findTodoDocumentByDocumentId(documents, documentId) {
  return documents.find((document) => document.meta.id === documentId);
}

export function resolveTodoReference(stores, referenceId) {
  const todoItem = stores.todoIndex.get(referenceId);
  if (todoItem) {
    return {
      type: "item",
      item: todoItem,
      document: findTodoDocumentByItemId(stores.todoDocuments, referenceId),
      status: todoItem.status,
      id: todoItem.id,
    };
  }

  const todoDocument = stores.todoDocumentIndex.get(referenceId);
  if (todoDocument) {
    return {
      type: "document",
      document: todoDocument,
      status: todoDocument.meta.status,
      id: todoDocument.meta.id,
    };
  }

  return null;
}

export function buildTodoReferenceIndex(stores) {
  const index = new Map();
  for (const item of stores.todoItems) {
    index.set(item.id, item);
  }
  for (const document of stores.todoDocuments) {
    index.set(document.meta.id, {
      id: document.meta.id,
      status: document.meta.status,
      ref: document.ref,
    });
  }
  return index;
}

export function nextIssueBucketPath(rootDir, kind, issueDocuments) {
  const prefix = ISSUE_BUCKETS[kind];
  const directory = path.join(rootDir, DOCS_DIRECTORIES.issues, prefix);
  const candidates = issueDocuments
    .filter((document) => document.meta.kind === kind)
    .sort((left, right) => left.filePath.localeCompare(right.filePath));

  for (const document of candidates) {
    if (document.items.length < ISSUE_BUCKET_ITEM_LIMIT) {
      return path.resolve(process.cwd(), document.filePath);
    }
  }

  const files = walkMarkdownFiles(directory).map((filePath) => path.basename(filePath, ".md"));
  const alphabet = "abcdefghijklmnopqrstuvwxyz".split("");
  const nextSuffix = alphabet.find((letter) => !files.includes(`${prefix}-${letter}`)) || "z";
  return path.join(directory, `${prefix}-${nextSuffix}.md`);
}

export function absoluteWorkspacePath(relativePath) {
  return path.resolve(process.cwd(), relativePath);
}

export function makeBucketHeading(filePath) {
  return path.basename(filePath, ".md");
}

export function toAbsolutePath(rootDir, relativeDocPath) {
  return path.isAbsolute(relativeDocPath)
    ? relativeDocPath
    : path.join(rootDir, path.relative(DEFAULT_DOCS_ROOT, relativeDocPath));
}

export function toDocWorkspacePath(filePath) {
  return toWorkspacePath(filePath);
}
