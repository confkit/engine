import path from "path";
import { DOCS_DIRECTORIES, ISSUE_BUCKETS } from "./constants.mjs";
import { TODO_OWNER_CAPTURE } from "./constants.mjs";
import { readText, toWorkspacePath, walkMarkdownFiles } from "./fs.mjs";
import { splitCsv } from "./formatter.mjs";

function parseMetaLine(line) {
  const match = line.match(/^([a-z-]+):\s*(.*)$/);
  if (!match) {
    return null;
  }
  return {
    key: match[1].trim(),
    value: match[2].trim(),
  };
}

function parseBulletField(line) {
  const match = line.match(/^-\s*([^:：]+)\s*[:：]\s*(.*)$/);
  if (!match) {
    return null;
  }
  return {
    key: match[1].trim(),
    value: match[2].trim(),
  };
}

function parseDocumentMeta(lines) {
  const meta = {};
  let title = "";
  let index = 0;
  let seenTitle = false;

  while (index < lines.length) {
    const line = lines[index].trim();
    if (!line) {
      if (!seenTitle) {
        index += 1;
        continue;
      }
      index += 1;
      continue;
    }

    if (line.startsWith("# ")) {
      title = line.slice(2).trim();
      seenTitle = true;
      index += 1;
      continue;
    }

    const metaLine = parseMetaLine(line);
    if (!metaLine) {
      break;
    }
    meta[metaLine.key] = metaLine.value;
    index += 1;
  }

  return { title, meta, startIndex: index };
}

function trimTrailingEmptyLines(lines) {
  const result = [...lines];
  while (result.length > 0 && !result[result.length - 1].trim()) {
    result.pop();
  }
  return result;
}

function parseItemFields(lines, startIndex, shouldStop) {
  const fields = {};
  let cursor = startIndex;

  while (cursor < lines.length) {
    const line = lines[cursor];
    if (shouldStop(line)) {
      break;
    }

    const field = parseBulletField(line);
    if (!field) {
      cursor += 1;
      continue;
    }

    if (field.value === "|") {
      const blockLines = [];
      cursor += 1;
      while (cursor < lines.length) {
        const nextLine = lines[cursor];
        if (shouldStop(nextLine) || parseBulletField(nextLine)) {
          break;
        }
        if (!nextLine.trim()) {
          blockLines.push("");
          cursor += 1;
          continue;
        }
        if (!nextLine.startsWith("  ")) {
          break;
        }
        blockLines.push(nextLine.slice(2));
        cursor += 1;
      }
      fields[field.key] = trimTrailingEmptyLines(blockLines).join("\n").trimEnd();
      continue;
    }

    fields[field.key] = field.value;
    cursor += 1;
  }

  return { fields, cursor };
}

export function parseTodoFile(filePath) {
  const todoHeadingPattern = new RegExp(
    `^###\\s+(?<id>[A-Z]+-[a-z0-9-]+-[a-f0-9]{6})\\s+\\[(?<owner>${TODO_OWNER_CAPTURE})\\]\\s+(?<title>.+)$`,
  );
  const lines = readText(filePath).split(/\r?\n/);
  const { title, meta, startIndex } = parseDocumentMeta(lines);
  const groups = [];
  let currentGroup = null;
  let verificationTitle = "阶段性验证";
  let verificationLines = [];

  for (let index = startIndex; index < lines.length; index += 1) {
    const line = lines[index];
    if (/^##\s+/.test(line)) {
      const headingTitle = line.replace(/^##\s+/, "").trim();
      if (["阶段性验证", "验收用例"].includes(headingTitle)) {
        verificationTitle = headingTitle;
        verificationLines = lines.slice(index + 1).filter((entry) => entry.trim());
        break;
      }
      currentGroup = {
        title: headingTitle,
        line: index + 1,
        items: [],
      };
      groups.push(currentGroup);
      continue;
    }

    const itemMatch = line.match(todoHeadingPattern);
    if (!itemMatch || !currentGroup) {
      continue;
    }

    const { fields, cursor } = parseItemFields(
      lines,
      index + 1,
      (line) => /^##\s+/.test(line) || /^###\s+/.test(line),
    );

    currentGroup.items.push({
      entity: "todo",
      kind: "todo",
      id: itemMatch.groups.id,
      owner: itemMatch.groups.owner,
      title: itemMatch.groups.title.trim(),
      status: fields.status || "",
      tags: splitCsv(fields.tags),
      paths: splitCsv(fields.paths),
      source: fields.source || "none",
      dependsOn: fields["depends-on"] || "none",
      acceptance: fields.acceptance || "",
      group: currentGroup.title,
      filePath: toWorkspacePath(filePath),
      line: index + 1,
      ref: `${toWorkspacePath(filePath)}#L${index + 1}`,
      rawFields: fields,
    });

    index = cursor - 1;
  }

  return {
    entity: "todo",
    filePath: toWorkspacePath(filePath),
    ref: `${toWorkspacePath(filePath)}#L1`,
    title,
    meta,
    groups,
    verificationTitle,
    verificationLines,
  };
}

export function parseIssueFile(filePath) {
  const lines = readText(filePath).split(/\r?\n/);
  const { title, meta, startIndex } = parseDocumentMeta(lines);
  const items = [];

  for (let index = startIndex; index < lines.length; index += 1) {
    const line = lines[index];
    const itemMatch = line.match(/^##\s+(?<id>[A-Z]+-[a-z0-9-]+-[a-f0-9]{6})\s+(?<title>.+)$/);
    if (!itemMatch) {
      continue;
    }

    const { fields, cursor } = parseItemFields(lines, index + 1, (line) => /^##\s+/.test(line));

    items.push({
      entity: "issue",
      kind: fields.kind || deriveIssueKind(filePath, itemMatch.groups.id),
      id: itemMatch.groups.id,
      title: itemMatch.groups.title.trim(),
      status: fields.status || "",
      priority: fields.priority || "",
      tags: splitCsv(fields.tags),
      todo: fields.todo || "none",
      closeReason: fields["close-reason"] || "none",
      summary: fields["现象 / 问题"] || fields.summary || "",
      cause: fields["原因"] || fields.cause || "",
      direction: fields["处理方向"] || fields.direction || "",
      scope: fields["影响范围"] || fields.scope || "",
      filePath: toWorkspacePath(filePath),
      line: index + 1,
      ref: `${toWorkspacePath(filePath)}#L${index + 1}`,
      rawFields: fields,
    });

    index = cursor - 1;
  }

  return {
    entity: "issue",
    filePath: toWorkspacePath(filePath),
    ref: `${toWorkspacePath(filePath)}#L1`,
    title,
    meta,
    items,
  };
}

function deriveIssueKind(filePath, id) {
  if (filePath.includes(`${path.sep}${ISSUE_BUCKETS.bug}${path.sep}`) || id.startsWith("BUG-")) {
    return "bug";
  }
  if (filePath.includes(`${path.sep}${ISSUE_BUCKETS.optimization}${path.sep}`) || id.startsWith("OPT-")) {
    return "optimization";
  }
  return "issue";
}

export function loadTodoDocuments(rootDir) {
  return walkMarkdownFiles(path.join(rootDir, DOCS_DIRECTORIES.todo)).map(parseTodoFile);
}

export function loadIssueDocuments(rootDir) {
  return walkMarkdownFiles(path.join(rootDir, DOCS_DIRECTORIES.issues)).map(parseIssueFile);
}
