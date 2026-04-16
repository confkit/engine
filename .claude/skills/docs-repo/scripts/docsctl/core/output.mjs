import { formatListLine, formatShow, formatValidationError } from "./formatter.mjs";

export function isJsonMode(args) {
  return args.json === true || args.json === "true";
}

function emit(args, payload, text) {
  if (args.silent) {
    return;
  }

  if (isJsonMode(args)) {
    console.log(JSON.stringify(payload, null, 2));
    return;
  }

  if (text) {
    console.log(text);
  }
}

export function printCommandError(args, message, extra = {}) {
  emit(
    args,
    {
      ok: false,
      error: {
        message,
        ...extra,
      },
    },
    message,
  );
  return 1;
}

export function printSchemaErrors(args, errors, extra = {}) {
  emit(
    args,
    {
      ok: false,
      ...extra,
      errorCount: errors.length,
      errors,
    },
    errors.map((error) => `${error.code} | ${error.message}`).join("\n"),
  );
  return 1;
}

export function printListResult(args, entity, items, filters = {}) {
  emit(
    args,
    {
      ok: true,
      entity,
      action: "list",
      count: items.length,
      filters,
      items,
    },
    items.length === 0 ? "No matching items." : items.map(formatListLine).join("\n"),
  );
  return items.length === 0 ? 1 : 0;
}

export function printShowResult(args, entity, item) {
  emit(
    args,
    {
      ok: true,
      entity,
      action: "show",
      item,
    },
    formatShow(item),
  );
  return 0;
}

export function printActionResult(args, message, data = {}) {
  emit(
    args,
    {
      ok: true,
      message,
      ...data,
    },
    message,
  );
  return 0;
}

export function printValidationResult(args, entity, errors) {
  if (errors.length === 0) {
    emit(
      args,
      {
        ok: true,
        entity,
        action: "validate",
        errorCount: 0,
        errors: [],
      },
      "Validation passed.",
    );
    return 0;
  }

  emit(
    args,
    {
      ok: false,
      entity,
      action: "validate",
      errorCount: errors.length,
      errors,
    },
    errors.map(formatValidationError).join("\n"),
  );
  return 1;
}
