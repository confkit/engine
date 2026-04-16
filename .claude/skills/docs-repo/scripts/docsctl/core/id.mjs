import crypto from "crypto";

const ENGLISH_SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

export function hash6(seed) {
  return crypto.createHash("sha1").update(seed).digest("hex").slice(0, 6);
}

export function normalizeEnglishSlug(value, fieldName = "slug") {
  const slug = String(value || "")
    .trim()
    .toLowerCase()
    .replace(/[\s_]+/g, "-")
    .replace(/-{2,}/g, "-")
    .replace(/^-+|-+$/g, "");

  if (!slug) {
    throw new Error(`${fieldName} must provide an English slug using only a-z, 0-9, and -.`);
  }
  if (!ENGLISH_SLUG.test(slug)) {
    throw new Error(`${fieldName} must use an English slug with only a-z, 0-9, and -.`);
  }
  return slug;
}

export function resolveEnglishSlug({ slug, title, entity }) {
  if (slug !== undefined) {
    return normalizeEnglishSlug(slug, "--slug");
  }
  if (/[^\x00-\x7F]/.test(String(title || ""))) {
    throw new Error(`Missing required option: --slug for ${entity} when --title contains non-English text.`);
  }
  return normalizeEnglishSlug(title, "--title");
}

export function createId(kind, slugValue) {
  const slug = normalizeEnglishSlug(slugValue, "slug");
  const suffix = hash6(`${kind}:${slug}:${Date.now()}:${Math.random()}`);
  return `${kind}-${slug}-${suffix}`;
}

export function createDocumentId(kind, slugValue) {
  const slug = normalizeEnglishSlug(slugValue, "slug");
  const suffix = hash6(`doc:${kind}:${slug}:${Date.now()}:${Math.random()}`);
  return `${kind}-DOC-${slug}-${suffix}`;
}

export function createFileStem(slugValue, namespace = "file") {
  const slug = normalizeEnglishSlug(slugValue, "slug");
  return `${slug}-${hash6(`${namespace}:${slug}:${Date.now()}:${Math.random()}`)}`;
}

export function extractSlugFromId(id) {
  const match = String(id || "").match(/^[A-Z]+-(?<slug>[a-z0-9-]+)-[a-f0-9]{6}$/);
  if (!match?.groups?.slug) {
    throw new Error(`Cannot extract slug from ID: ${id}`);
  }
  return match.groups.slug;
}
