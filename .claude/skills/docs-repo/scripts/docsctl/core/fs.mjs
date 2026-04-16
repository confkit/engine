import fs from "fs";
import os from "os";
import path from "path";
import { DEFAULT_DOCS_ROOT } from "./constants.mjs";

export function resolveDocsRoot(rootOption) {
  return path.resolve(process.cwd(), rootOption || DEFAULT_DOCS_ROOT);
}

export function exists(filePath) {
  return fs.existsSync(filePath);
}

export function readText(filePath) {
  return fs.readFileSync(filePath, "utf8");
}

export function readStdinText() {
  return fs.readFileSync(0, "utf8");
}

export function ensureDir(dirPath) {
  fs.mkdirSync(dirPath, { recursive: true });
}

export function writeText(filePath, content) {
  ensureDir(path.dirname(filePath));
  fs.writeFileSync(filePath, content, "utf8");
}

export function copyDir(sourceDir, targetDir) {
  if (!exists(sourceDir)) {
    ensureDir(targetDir);
    return;
  }
  fs.cpSync(sourceDir, targetDir, { recursive: true });
}

export function makeTempDir(prefix = "docsctl-") {
  return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

export function removeFile(filePath) {
  if (exists(filePath)) {
    fs.unlinkSync(filePath);
  }
}

export function walkMarkdownFiles(dir) {
  if (!exists(dir)) {
    return [];
  }

  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const files = [];
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...walkMarkdownFiles(fullPath));
      continue;
    }
    if (entry.isFile() && entry.name.endsWith(".md")) {
      files.push(fullPath);
    }
  }

  return files.sort();
}

export function toWorkspacePath(filePath) {
  const relativePath = path.relative(process.cwd(), filePath);
  return relativePath || filePath;
}
