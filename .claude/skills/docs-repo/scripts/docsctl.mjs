#!/usr/bin/env node

import { parseCliArgs } from "./docsctl/core/args.mjs";
import { runAppend, runBatchAppend } from "./docsctl/commands/append.mjs";
import { runClose, runBatchClose } from "./docsctl/commands/close.mjs";
import { runCreate, runBatchCreate } from "./docsctl/commands/create.mjs";
import { runDelete, runBatchDelete } from "./docsctl/commands/delete.mjs";
import { runExamples } from "./docsctl/commands/examples.mjs";
import { runList } from "./docsctl/commands/list.mjs";
import { runPlan, runBatchPlan } from "./docsctl/commands/plan.mjs";
import { runShow } from "./docsctl/commands/show.mjs";
import { runUpdate, runBatchUpdate } from "./docsctl/commands/update.mjs";
import { runValidate } from "./docsctl/commands/validate.mjs";
import { DEFAULT_DOCS_ROOT } from "./docsctl/core/constants.mjs";
import { isJsonMode } from "./docsctl/core/output.mjs";

function printHelp() {
  console.log(`Usage:
  node scripts/docsctl.mjs <entity> <action> [options]
  node scripts/docsctl.mjs validate [--entity todo|issue|all] [--root ${DEFAULT_DOCS_ROOT}]
  node scripts/docsctl.mjs examples [todo-batch-create|todo-batch-append|todo-batch-append-anchor|todo-batch-append-group-anchor|issue-batch-create|issue-batch-plan|issue-batch-update|issue-batch-close|issue-batch-delete]

Entities:
  todo           Manage todo items
  issue          Manage issue items
  validate       Validate docs structure
  examples       Print reusable JSON examples

Actions:
  list           List matching items
  show           Show one item by --id
  create         Create one docs item
  batch-create   Create docs items from JSON manifest
  append         Append one todo item into an existing todo document
  batch-append   Append todo items from JSON manifest
  update         Update a docs item
  batch-update   Update issue items from JSON manifest
  delete         Delete a docs item
  batch-delete   Delete issue items from JSON manifest
  plan           Create a minimal todo from issue
  batch-plan     Create a full todo plan from JSON manifest
  close          Close issue
  batch-close    Close issue items from JSON manifest

Common options:
  --root <path>      Docs root directory. Default: ${DEFAULT_DOCS_ROOT}
  --id <value>       Stable item ID for show/update/delete/plan/close
  --doc-id <value>   Todo document ID for append
  --slug <value>     English slug for ID/file naming. Use a-z, 0-9, and -
  --status <value>   Todo/issue only: filter by status or set status on update
  --tag <value>      Filter by tag. Repeatable or comma-separated
  --text <value>     Text match across common fields
  --limit <n>        Limit listed items
  --json             Print machine-friendly JSON output
  --file <path>      Read structured JSON input from file
  --stdin            Read structured JSON input from stdin
  --dry-run          Preview generated changes without writing files
  --write <path>     examples only: write JSON example to file
  --position <v>     todo append position: start | end
  --before <value>   todo append anchor item ID, insert before it
  --after <value>    todo append anchor item ID, insert after it
  --before-group <v> New todo group anchor title, insert before it
  --after-group <v>  New todo group anchor title, insert after it
  --title <value>    Title for create or update
  --reason <value>   Reason for forced close/delete
  --priority <v>     Priority for issue
  --constraints <v>  Constraints text

Todo options:
  --owner <code>     Filter todo items by owner code, e.g. FE, BE, APP, API, INFRA, CORE
  --group <value>    Group title for todo create/plan
  --acceptance <v>   Acceptance text for todo
  --source <value>   Source issue/doc ID for todo
  --path <value>     Todo affected path. Repeatable or comma-separated

Issue options:
  --kind <value>      bug | optimization | all
  --todo <value>      Linked todo item/document ID or none
  --close-reason <v>  Close reason for closed/dropped issue
  --summary <v>       Issue summary
  --direction <v>     Issue direction
  --scope <v>         Issue scope
  --cause <v>         Issue cause
  --entity <value>    validate only: todo | issue | all

Notes:
  Use '- <field>: |' with indented lines when a field needs multi-line content.
  Slug must be English. If --title contains non-English text, pass --slug explicitly.

Examples:
  node scripts/docsctl.mjs todo create --title "登录需求" --slug login-requirement
  node scripts/docsctl.mjs todo batch-create --file todo.json
  node scripts/docsctl.mjs todo batch-create --file todo.json --dry-run
  node scripts/docsctl.mjs todo append --doc-id TODO-DOC-auth-a1b2c3 --title "补充回归" --owner CORE
  node scripts/docsctl.mjs todo batch-append --doc-id TODO-DOC-auth-a1b2c3 --file append.json
  node scripts/docsctl.mjs examples todo-batch-append-anchor --write append-anchor.json
  node scripts/docsctl.mjs issue create --kind bug --title "登录态丢失" --slug login-state-lost
  node scripts/docsctl.mjs issue batch-create --file issues.json --dry-run
  node scripts/docsctl.mjs issue batch-plan --id BUG-auth-a1b2c3 --file plan.json
  node scripts/docsctl.mjs issue batch-update --file issue-update.json --dry-run
  node scripts/docsctl.mjs issue batch-close --file issue-close.json
  node scripts/docsctl.mjs issue batch-delete --file issue-delete.json --dry-run
  node scripts/docsctl.mjs examples issue-batch-plan
  node scripts/docsctl.mjs examples issue-batch-create --write issue-batch-create.json
  node scripts/docsctl.mjs validate --entity issue`);
}

function main() {
  const args = parseCliArgs(process.argv.slice(2));
  const [entity, action] = args._;

  if (!entity || entity === "help" || args.help) {
    printHelp();
    process.exitCode = 0;
    return;
  }

  try {
    if (entity === "validate") {
      process.exitCode = runValidate(args);
      return;
    }

    if (entity === "examples") {
      process.exitCode = runExamples(args);
      return;
    }

    if (!["todo", "issue"].includes(entity)) {
      throw new Error(`unsupported entity: ${entity}`);
    }

    if (action === "list") {
      process.exitCode = runList(entity, args);
      return;
    }

    if (action === "show") {
      process.exitCode = runShow(entity, args);
      return;
    }

    if (action === "create") {
      process.exitCode = runCreate(entity, args);
      return;
    }

    if (action === "batch-create") {
      process.exitCode = runBatchCreate(entity, args);
      return;
    }

    if (action === "append") {
      process.exitCode = runAppend(entity, args);
      return;
    }

    if (action === "batch-append") {
      process.exitCode = runBatchAppend(entity, args);
      return;
    }

    if (action === "update") {
      process.exitCode = runUpdate(entity, args);
      return;
    }

    if (action === "batch-update") {
      process.exitCode = runBatchUpdate(entity, args);
      return;
    }

    if (action === "delete") {
      process.exitCode = runDelete(entity, args);
      return;
    }

    if (action === "batch-delete") {
      process.exitCode = runBatchDelete(entity, args);
      return;
    }

    if (entity === "issue" && action === "plan") {
      process.exitCode = runPlan(args);
      return;
    }

    if (entity === "issue" && action === "batch-plan") {
      process.exitCode = runBatchPlan(args);
      return;
    }

    if (entity === "issue" && action === "close") {
      process.exitCode = runClose(args);
      return;
    }

    if (entity === "issue" && action === "batch-close") {
      process.exitCode = runBatchClose(args);
      return;
    }

    throw new Error(`unsupported action: ${action || "none"}`);
  } catch (error) {
    if (isJsonMode(args)) {
      console.log(JSON.stringify({ ok: false, error: { message: error.message } }, null, 2));
    } else {
      console.log(error.message);
    }
    process.exitCode = 1;
  }
}

main();
