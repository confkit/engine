import path from "path";
import { copyDir, makeTempDir, resolveDocsRoot } from "./fs.mjs";
import { loadJsonInput } from "./todo-manifest.mjs";
import { printActionResult, printCommandError } from "./output.mjs";

export function loadBatchItems(args, contextLabel) {
  const payload = loadJsonInput(args, contextLabel);
  const items = Array.isArray(payload.items)
    ? payload.items
    : Array.isArray(payload.issues)
      ? payload.issues
      : null;
  if (!items || items.length === 0) {
    throw new Error(`${contextLabel} requires items[] or issues[].`);
  }
  return { payload, items };
}

export function runBatchWithPreview({
  args,
  entity,
  action,
  items,
  mapItemToArgs,
  runSingle,
  summarize,
}) {
  const actualRoot = resolveDocsRoot(args.root);
  const previewRoot = path.join(makeTempDir(`docsctl-${action}-`), ".docs");
  copyDir(actualRoot, previewRoot);

  for (const [index, item] of items.entries()) {
    const operationArgs = mapItemToArgs(item, index, previewRoot);
    const previewResult = runSingle({
      ...operationArgs,
      root: previewRoot,
      silent: true,
      json: false,
    });
    if (previewResult !== 0) {
      runSingle({
        ...operationArgs,
        root: previewRoot,
        silent: false,
        json: args.json,
      });
      return 1;
    }
  }

  if (!args["dry-run"]) {
    for (const [index, item] of items.entries()) {
      const operationArgs = mapItemToArgs(item, index, actualRoot);
      const writeResult = runSingle({
        ...operationArgs,
        root: actualRoot,
        silent: true,
        json: false,
      });
      if (writeResult !== 0) {
        runSingle({
          ...operationArgs,
          root: actualRoot,
          silent: false,
          json: args.json,
        });
        return 1;
      }
    }
  }

  return printActionResult(args, summarize.message(args["dry-run"]), {
    entity,
    action,
    dryRun: Boolean(args["dry-run"]),
    ...summarize.data,
  });
}

export function requireIssueItemId(item, contextLabel) {
  const id = String(item?.id || "").trim();
  if (!id) {
    throw new Error(`${contextLabel} requires id.`);
  }
  return id;
}

export function toBatchCommandError(args, action, error) {
  return printCommandError(args, error.message, { entity: "issue", action });
}
