import fs from 'node:fs';
import path from 'node:path';
import { minimatch } from 'minimatch';
import type { StrictPluginConfig } from '../types/index.js';

const STRICT_COMMENT = /@ts-strict\b/;
const STRICT_IGNORE_COMMENT = /@ts-strict-ignore\b/;

export function findStrictCandidates(
  projectFiles: string[],
  pluginConfig: StrictPluginConfig | null,
  configDir: string
): string[] {
  const unique = Array.from(new Set(projectFiles.map((file) => path.resolve(file))));

  return unique.filter((filePath) => isStrictFile(filePath, pluginConfig, configDir));
}

function isStrictFile(
  filePath: string,
  pluginConfig: StrictPluginConfig | null,
  configDir: string
): boolean {
  const sourceHead = readHead(filePath, 4096);
  const hasStrict = STRICT_COMMENT.test(sourceHead);
  const hasIgnore = STRICT_IGNORE_COMMENT.test(sourceHead);

  if (hasIgnore) {
    return false;
  }

  if (!pluginConfig) {
    return true;
  }

  const relative = normalizeRelative(filePath, configDir);

  const inPaths =
    !pluginConfig.paths ||
    pluginConfig.paths.length === 0 ||
    pluginConfig.paths.some((pattern) =>
      minimatch(relative, normalizePattern(pattern), { dot: true })
    );

  const excludedByRegex = pluginConfig.excludePattern
    ? new RegExp(pluginConfig.excludePattern).test(relative)
    : false;

  if (hasStrict) {
    return true;
  }

  return inPaths && !excludedByRegex;
}

function readHead(filePath: string, bytes: number): string {
  try {
    const handle = fs.openSync(filePath, 'r');
    const buffer = Buffer.alloc(bytes);
    const read = fs.readSync(handle, buffer, 0, bytes, 0);
    fs.closeSync(handle);
    return buffer.toString('utf8', 0, read);
  } catch {
    return '';
  }
}

function normalizeRelative(filePath: string, baseDir: string): string {
  const relative = path.relative(baseDir, filePath);
  return relative.split(path.sep).join('/');
}

function normalizePattern(pattern: string): string {
  const cleaned = pattern.split(path.sep).join('/');
  if (cleaned.endsWith('/**')) {
    return cleaned;
  }
  if (cleaned.endsWith('/*')) {
    return cleaned;
  }
  return cleaned;
}
