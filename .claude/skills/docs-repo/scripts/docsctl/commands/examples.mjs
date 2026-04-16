import { writeText } from "../core/fs.mjs";
import { isJsonMode } from "../core/output.mjs";

const EXAMPLES = {
  "todo-batch-create": {
    description: "Create a todo document with multiple groups and items.",
    payload: {
      title: "login state fix",
      slug: "login-state-fix",
      groups: [
        {
          title: "G1. root cause",
          items: [
            {
              title: "trace state source",
              owner: "CORE",
              acceptance: "source is clear",
            },
          ],
        },
      ],
      verificationLines: ["- cover main path"],
    },
  },
  "todo-batch-append": {
    description: "Append multiple todo items into an existing todo document.",
    payload: {
      group: "G2. regression",
      position: "start",
      items: [
        {
          title: "add regression checks",
          owner: "CORE",
          acceptance: "main regression paths are covered",
        },
      ],
    },
  },
  "todo-batch-append-anchor": {
    description: "Insert todo items around an existing todo item anchor.",
    payload: {
      group: "G1. initial breakdown",
      after: "TODO-auth-plan-f33ce0",
      items: [
        {
          title: "verify refresh flow",
          slug: "verify-refresh-flow",
          owner: "CORE",
          acceptance: "refresh path is covered",
        },
        {
          title: "verify token expiry flow",
          slug: "verify-token-expiry-flow",
          owner: "CORE",
          acceptance: "expiry path is covered",
        },
      ],
    },
  },
  "todo-batch-append-group-anchor": {
    description: "Create a new todo group and place it around another group.",
    payload: {
      groups: [
        {
          title: "G2. regression",
          beforeGroup: "G3. rollout",
          items: [
            {
              title: "add regression checks",
              slug: "add-regression-checks",
              owner: "CORE",
              acceptance: "main regression paths are covered",
            },
          ],
        },
      ],
    },
  },
  "issue-batch-create": {
    description: "Create multiple issues into issue buckets.",
    payload: {
      kind: "bug",
      items: [
        {
          title: "login state lost",
          slug: "login-state-lost",
          priority: "high",
          summary: "session is lost after refresh",
          direction: "restore session flow",
          scope: "web auth flow",
        },
      ],
    },
  },
  "issue-batch-plan": {
    description: "Plan an issue into a full todo document.",
    payload: {
      groups: [
        {
          title: "G1. reproduce and locate",
          items: [
            {
              title: "confirm reproduce steps",
              owner: "CORE",
              acceptance: "issue can be reproduced",
            },
          ],
        },
      ],
    },
  },
  "issue-batch-update": {
    description: "Update multiple issue fields in one batch.",
    payload: {
      items: [
        {
          id: "BUG-login-state-lost-abcdef",
          priority: "critical",
          direction: "restore session flow",
        },
      ],
    },
  },
  "issue-batch-close": {
    description: "Close multiple issues in one batch.",
    payload: {
      items: [
        {
          id: "BUG-login-state-lost-abcdef",
          force: true,
          reason: "validated manually",
        },
      ],
    },
  },
  "issue-batch-delete": {
    description: "Delete multiple issues in one batch.",
    payload: {
      items: [
        {
          id: "BUG-login-state-lost-abcdef",
          force: true,
          reason: "duplicate issue",
        },
      ],
    },
  },
};

export function runExamples(args) {
  const key = args._[1];
  if (!key) {
    return emit(args, {
      ok: true,
      action: "examples",
      names: Object.keys(EXAMPLES),
    }, Object.keys(EXAMPLES).join("\n"));
  }

  const example = EXAMPLES[key];
  if (!example) {
    return emit(args, {
      ok: false,
      action: "examples",
      error: { message: `Unknown example: ${key}` },
    }, `Unknown example: ${key}`);
  }

  if (args.write) {
    writeText(args.write, `${JSON.stringify(example.payload, null, 2)}\n`);
    return emit(args, {
      ok: true,
      action: "examples",
      name: key,
      wrote: args.write,
      ...example,
    }, args.write);
  }

  return emit(args, {
    ok: true,
    action: "examples",
    name: key,
    ...example,
  }, JSON.stringify(example.payload, null, 2));
}

function emit(args, payload, text) {
  if (isJsonMode(args)) {
    console.log(JSON.stringify(payload, null, 2));
  } else {
    console.log(text);
  }
  return payload.ok === false ? 1 : 0;
}
