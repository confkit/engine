export function splitCsv(value) {
  if (!value || value === "none") {
    return [];
  }

  return String(value)
    .split(/[，,]/)
    .map((item) => item.trim())
    .filter(Boolean);
}

export function truncateTitle(value, maxLength = 28) {
  const text = String(value || "").trim();
  if (text.length <= maxLength) {
    return text;
  }
  return `${text.slice(0, maxLength - 3)}...`;
}

export function formatListLine(item) {
  const parts = item.entity === "todo" || item.entity === "issue"
    ? [item.kind || item.entity, item.status || "-", item.id, truncateTitle(item.title)]
    : [item.kind || item.entity, item.id, truncateTitle(item.title)];

  if (item.entity === "todo" && item.owner) {
    parts.push(`owner=${item.owner}`);
  }
  if (item.entity === "issue") {
    if (item.priority) {
      parts.push(`priority=${item.priority}`);
    }
    if (item.todo) {
      parts.push(`todo=${item.todo}`);
    }
  }

  return parts.join(" | ");
}

function pushShowField(lines, label, value, { empty = "-" } = {}) {
  if (value === undefined || value === null || value === "") {
    lines.push(`${label}: ${empty}`);
    return;
  }

  const text = String(value);
  if (!text.includes("\n")) {
    lines.push(`${label}: ${text}`);
    return;
  }

  lines.push(`${label}: |`);
  for (const line of text.split("\n")) {
    lines.push(`  ${line}`);
  }
}

export function formatShow(item) {
  const lines = [
    `entity: ${item.entity}`,
    `id: ${item.id}`,
    `title: ${item.title}`,
    `ref: ${item.ref}`,
  ];

  if (item.entity === "todo") {
    lines.splice(3, 0, `status: ${item.status || "-"}`);
    if (item.owner) lines.push(`owner: ${item.owner}`);
    if (item.group) lines.push(`group: ${item.group}`);
    if (item.tags.length > 0) lines.push(`tags: ${item.tags.join(", ")}`);
    if (item.paths.length > 0) lines.push(`paths: ${item.paths.join(", ")}`);
    lines.push(`source: ${item.source || "none"}`);
    lines.push(`depends-on: ${item.dependsOn || "none"}`);
    pushShowField(lines, "acceptance", item.acceptance);
    return lines.join("\n");
  }

  if (item.entity === "requirement") {
    if (item.tags.length > 0) lines.push(`tags: ${item.tags.join(", ")}`);
    lines.push(`priority: ${item.priority || "-"}`);
    pushShowField(lines, "scenario", item.scenario);
    pushShowField(lines, "acceptance", item.acceptance);
    pushShowField(lines, "constraints", item.constraints);
    return lines.join("\n");
  }

  if (item.entity === "architecture") {
    if (item.tags.length > 0) lines.push(`tags: ${item.tags.join(", ")}`);
    pushShowField(lines, "module", item.module);
    pushShowField(lines, "decision", item.decision);
    pushShowField(lines, "constraints", item.constraints);
    pushShowField(lines, "trade-offs", item.tradeOffs);
    return lines.join("\n");
  }

  if (item.entity === "testing") {
    if (item.tags.length > 0) lines.push(`tags: ${item.tags.join(", ")}`);
    lines.push(`priority: ${item.priority || "-"}`);
    pushShowField(lines, "preconditions", item.preconditions);
    pushShowField(lines, "steps", item.steps);
    pushShowField(lines, "expected", item.expected);
    return lines.join("\n");
  }

  lines.splice(3, 0, `status: ${item.status || "-"}`);
  lines.push(`kind: ${item.kind}`);
  lines.push(`priority: ${item.priority || "-"}`);
  if (item.tags.length > 0) lines.push(`tags: ${item.tags.join(", ")}`);
  lines.push(`todo: ${item.todo || "none"}`);
  lines.push(`close-reason: ${item.closeReason || "none"}`);
  pushShowField(lines, "summary", item.summary);
  pushShowField(lines, "cause", item.cause);
  pushShowField(lines, "direction", item.direction);
  pushShowField(lines, "scope", item.scope);
  return lines.join("\n");
}

export function formatValidationError(error) {
  return `${error.code} | ${error.id || "-"} | ${error.ref} | ${error.message}`;
}

function joinCsv(values) {
  const list = Array.isArray(values) ? values : splitCsv(values);
  return list.length > 0 ? list.join(", ") : "";
}

function pushMeta(lines, meta, keys) {
  for (const key of keys) {
    if (meta[key] !== undefined) {
      lines.push(`${key}: ${meta[key]}`);
    }
  }
}

function pushBulletValue(lines, label, value, { force = false } = {}) {
  if (value === undefined || value === null || value === "") {
    if (!force) {
      return;
    }
    lines.push(`- ${label}:`);
    return;
  }

  if (Array.isArray(value)) {
    const text = joinCsv(value);
    if (!text && !force) {
      return;
    }
    lines.push(`- ${label}: ${text}`);
    return;
  }

  const text = String(value);
  if (!text.includes("\n")) {
    lines.push(`- ${label}: ${text}`);
    return;
  }

  lines.push(`- ${label}: |`);
  for (const line of text.split("\n")) {
    lines.push(`  ${line}`);
  }
}

export function formatTodoDocument(document) {
  const lines = [`# ${document.meta.title || document.title || document.meta.id}`];
  pushMeta(lines, document.meta, ["type", "id", "title", "status", "tags", "source", "scope"]);
  lines.push("");

  for (const group of document.groups) {
    lines.push(`## ${group.title}`);
    lines.push("");
    for (const item of group.items) {
      lines.push(`### ${item.id} [${item.owner}] ${item.title}`);
      pushBulletValue(lines, "status", item.status, { force: true });
      pushBulletValue(lines, "tags", item.tags, { force: true });
      if (item.paths && item.paths.length > 0) {
        pushBulletValue(lines, "paths", item.paths);
      }
      pushBulletValue(lines, "source", item.source || "none", { force: true });
      pushBulletValue(lines, "depends-on", item.dependsOn || "none", { force: true });
      pushBulletValue(lines, "acceptance", item.acceptance, { force: true });
      lines.push("");
    }
  }

  lines.push(`## ${document.verificationTitle || "阶段性验证"}`);
  for (const line of document.verificationLines || ["- 待补充"]) {
    lines.push(line);
  }

  return `${lines.join("\n").replace(/\n{3,}/g, "\n\n")}\n`;
}

export function formatIssueDocument(document) {
  const lines = [`# ${document.meta.title || document.title || document.meta.id}`];
  pushMeta(lines, document.meta, ["type", "id", "title", "kind", "status", "tags"]);
  lines.push("");

  for (const item of document.items) {
    lines.push(`## ${item.id} ${item.title}`);
    pushBulletValue(lines, "kind", item.kind, { force: true });
    pushBulletValue(lines, "status", item.status, { force: true });
    pushBulletValue(lines, "priority", item.priority, { force: true });
    pushBulletValue(lines, "tags", item.tags, { force: true });
    pushBulletValue(lines, "todo", item.todo || "none", { force: true });
    pushBulletValue(lines, "close-reason", item.closeReason || "none", { force: true });
    pushBulletValue(lines, "现象 / 问题", item.summary, { force: true });
    if (item.cause) {
      pushBulletValue(lines, "原因", item.cause);
    }
    pushBulletValue(lines, "处理方向", item.direction, { force: true });
    pushBulletValue(lines, "影响范围", item.scope, { force: true });
    lines.push("");
  }

  return `${lines.join("\n").replace(/\n{3,}/g, "\n\n")}\n`;
}
