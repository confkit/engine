import { aggregateIssueStatus } from "../core/model.mjs";

const ISSUE_STATUSES = new Set(["open", "planned", "doing", "closed", "dropped"]);
const ISSUE_KINDS = new Set(["bug", "optimization"]);
const ISSUE_PRIORITIES = new Set(["low", "medium", "high", "critical"]);
const TODO_DONE_STATES = new Set(["done", "dropped"]);
const ISSUE_DOCUMENT_ID = /^ISSUE-DOC-[a-z0-9-]+-[a-f0-9]{6}$/;
const ISSUE_ITEM_ID = {
  bug: /^BUG-[a-z0-9-]+-[a-f0-9]{6}$/,
  optimization: /^OPT-[a-z0-9-]+-[a-f0-9]{6}$/,
};

export function validateIssueDocument(document, todoIndex = new Map()) {
  const errors = [];
  const requiredMeta = ["type", "id", "title", "kind", "status", "tags"];

  for (const key of requiredMeta) {
    if (!document.meta[key]) {
      errors.push(error("issue.meta.missing", document.meta.id, document.ref, `missing meta field: ${key}`));
    }
  }

  if (document.meta.type && document.meta.type !== "issue") {
    errors.push(error("issue.meta.type", document.meta.id, document.ref, "type must be issue"));
  }
  if (document.meta.id && !ISSUE_DOCUMENT_ID.test(document.meta.id)) {
    errors.push(error("issue.meta.id", document.meta.id, document.ref, "id must match ISSUE-DOC-<topic>-<hash6>"));
  }
  if (document.meta.kind && !ISSUE_KINDS.has(document.meta.kind)) {
    errors.push(error("issue.meta.kind", document.meta.id, document.ref, `invalid meta kind: ${document.meta.kind}`));
  }
  if (document.meta.status && !ISSUE_STATUSES.has(document.meta.status)) {
    errors.push(error("issue.meta.status", document.meta.id, document.ref, `invalid meta status: ${document.meta.status}`));
  }

  for (const item of document.items) {
    if (!ISSUE_KINDS.has(item.kind)) {
      errors.push(error("issue.kind.invalid", item.id, item.ref, `invalid kind: ${item.kind || "none"}`));
    }
    if (document.meta.kind && item.kind && document.meta.kind !== item.kind) {
      errors.push(error("issue.kind.mismatch", item.id, item.ref, `item kind must match document kind: ${document.meta.kind}`));
    }
    const itemIdPattern = ISSUE_ITEM_ID[item.kind];
    if (itemIdPattern && !itemIdPattern.test(item.id || "")) {
      errors.push(error("issue.item.id", item.id, item.ref, `id must match ${item.kind === "bug" ? "BUG" : "OPT"}-<topic>-<hash6>`));
    }
    if (!ISSUE_STATUSES.has(item.status)) {
      errors.push(error("issue.status.invalid", item.id, item.ref, `invalid status: ${item.status || "none"}`));
    }
    if (!ISSUE_PRIORITIES.has(item.priority)) {
      errors.push(error("issue.priority.invalid", item.id, item.ref, `invalid priority: ${item.priority || "none"}`));
    }
    if (item.tags.length === 0) {
      errors.push(error("issue.tags.missing", item.id, item.ref, "missing tags"));
    }
    if (!item.todo) {
      errors.push(error("issue.todo.missing", item.id, item.ref, "missing todo field"));
    }
    if (!item.direction) {
      errors.push(error("issue.direction.missing", item.id, item.ref, "missing direction"));
    }
    if (!item.scope) {
      errors.push(error("issue.scope.missing", item.id, item.ref, "missing scope"));
    }
    if (!item.summary) {
      errors.push(error("issue.summary.missing", item.id, item.ref, "missing summary"));
    }
    if (["planned", "doing", "closed"].includes(item.status) && item.todo === "none") {
      errors.push(error("issue.todo.required", item.id, item.ref, `status=${item.status} requires todo!=none`));
    }
    if (["closed", "dropped"].includes(item.status) && item.todo !== "none") {
      const linkedTodo = todoIndex.get(item.todo);
      if (linkedTodo && !TODO_DONE_STATES.has(linkedTodo.status) && item.closeReason === "none") {
        errors.push(error("issue.close-reason.required", item.id, item.ref, `status=${item.status} requires close-reason when linked todo is unfinished`));
      }
    }
  }
  if (document.items.length > 0) {
    const aggregatedStatus = aggregateIssueStatus(document.items);
    if (document.meta.status && document.meta.status !== aggregatedStatus) {
      errors.push(error("issue.meta.status.aggregate", document.meta.id, document.ref, `meta status must match aggregate status: ${aggregatedStatus}`));
    }
  }

  return errors;
}

function error(code, id, ref, message) {
  return { code, id, ref, message };
}
