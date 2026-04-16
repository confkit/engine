export const DEFAULT_DOCS_ROOT = ".docs";
export const DEFAULT_TODO_OWNER = "CORE";
export const TODO_OWNER_CAPTURE = "[A-Z][A-Z0-9-]{0,15}";
export const TODO_OWNER_PATTERN = new RegExp(`^${TODO_OWNER_CAPTURE}$`);

export const DOCS_DIRECTORIES = {
  requirements: "requirements",
  architectures: "architectures",
  todo: "todo",
  testing: "testing",
  issues: "issues",
};

export const ISSUE_BUCKETS = {
  bug: "bugs",
  optimization: "optimizations",
};

export const ISSUE_BUCKET_ITEM_LIMIT = 50;
