import { TODO_OWNER_PATTERN } from "../core/constants.mjs";
import { aggregateTodoStatus } from "../core/model.mjs";

const TODO_STATUSES = new Set(["todo", "doing", "blocked", "done", "dropped"]);
const TODO_DOCUMENT_ID = /^TODO-DOC-[a-z0-9-]+-[a-f0-9]{6}$/;
const TODO_ITEM_ID = /^TODO-[a-z0-9-]+-[a-f0-9]{6}$/;

export function validateTodoDocument(document) {
  const errors = [];
  const requiredMeta = ["type", "id", "title", "status", "tags"];

  for (const key of requiredMeta) {
    if (!document.meta[key]) {
      errors.push(error("todo.meta.missing", document.meta.id, document.ref, `missing meta field: ${key}`));
    }
  }

  if (document.meta.type && document.meta.type !== "todo") {
    errors.push(error("todo.meta.type", document.meta.id, document.ref, "type must be todo"));
  }
  if (document.meta.id && !TODO_DOCUMENT_ID.test(document.meta.id)) {
    errors.push(error("todo.meta.id", document.meta.id, document.ref, "id must match TODO-DOC-<topic>-<hash6>"));
  }
  if (document.meta.status && !TODO_STATUSES.has(document.meta.status)) {
    errors.push(error("todo.meta.status", document.meta.id, document.ref, `invalid meta status: ${document.meta.status}`));
  }

  if (document.groups.length === 0) {
    errors.push(error("todo.group.missing", document.meta.id, document.ref, "todo document requires at least one ## group"));
  }
  if (!["阶段性验证", "验收用例"].includes(document.verificationTitle)) {
    errors.push(error("todo.verification.title", document.meta.id, document.ref, "verification section must be 阶段性验证 or 验收用例"));
  }
  if (!document.verificationLines || document.verificationLines.length === 0) {
    errors.push(error("todo.verification.missing", document.meta.id, document.ref, "todo document requires verification bullet lines"));
  } else if (!isValidVerificationLines(document.verificationLines)) {
    errors.push(error("todo.verification.format", document.meta.id, document.ref, "verification lines must use bullet list with optional indented continuation"));
  }

  for (const group of document.groups) {
    if (group.items.length === 0) {
      errors.push(error("todo.group.empty", document.meta.id, `${document.filePath}#L${group.line}`, "group must contain at least one todo item"));
    }

    for (const item of group.items) {
      if (!TODO_ITEM_ID.test(item.id || "")) {
        errors.push(error("todo.item.id", item.id, item.ref, "id must match TODO-<topic>-<hash6>"));
      }
      if (!TODO_OWNER_PATTERN.test(item.owner || "")) {
        errors.push(error(
          "todo.owner.invalid",
          item.id,
          item.ref,
          `invalid owner: ${item.owner || "none"}; use uppercase responsibility code such as FE, BE, APP, API, INFRA, CORE`,
        ));
      }
      if (!TODO_STATUSES.has(item.status)) {
        errors.push(error("todo.status.invalid", item.id, item.ref, `invalid status: ${item.status || "none"}`));
      }
      if (!item.acceptance) {
        errors.push(error("todo.acceptance.missing", item.id, item.ref, "missing acceptance"));
      }
      if (item.tags.length === 0) {
        errors.push(error("todo.tags.missing", item.id, item.ref, "missing tags"));
      }
    }
  }
  if (document.groups.length > 0) {
    const aggregatedStatus = aggregateTodoStatus(document.groups);
    if (document.meta.status && document.meta.status !== aggregatedStatus) {
      errors.push(error("todo.meta.status.aggregate", document.meta.id, document.ref, `meta status must match aggregate status: ${aggregatedStatus}`));
    }
  }

  return errors;
}

function error(code, id, ref, message) {
  return { code, id, ref, message };
}

function isValidVerificationLines(lines) {
  let sawBullet = false;
  for (const line of lines) {
    if (!line.trim()) {
      continue;
    }
    if (/^\s*-\s+/.test(line)) {
      sawBullet = true;
      continue;
    }
    if (/^\s{2,}\S/.test(line)) {
      continue;
    }
    return false;
  }
  return sawBullet;
}
