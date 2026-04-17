import fs from 'node:fs';
import path from 'node:path';
import fg from 'fast-glob';

const TS_EXTENSIONS = ['.ts', '.tsx', '.cts', '.mts'];

export function resolveSubsetInputs(inputs: string[], cwd: string): string[] {
  if (inputs.length === 0) {
    return [];
  }

  const explicitFiles: string[] = [];
  const patterns: string[] = [];

  for (const input of inputs) {
    if (isGlob(input)) {
      patterns.push(input);
      continue;
    }

    const abs = path.resolve(cwd, input);
    if (!fs.existsSync(abs)) {
      patterns.push(input);
      continue;
    }

    const stat = fs.statSync(abs);
    if (stat.isDirectory()) {
      patterns.push(`${input.replace(/\\/g, '/')}/**/*.{ts,tsx,cts,mts}`);
      continue;
    }

    if (isTsFile(abs)) {
      explicitFiles.push(abs);
    }
  }

  const globbed = patterns.length
    ? fg.sync(patterns, {
        cwd,
        absolute: true,
        onlyFiles: true,
        dot: true,
        unique: true,
        ignore: ['**/node_modules/**', '**/.git/**']
      })
    : [];

  const merged = new Set<string>();
  for (const file of [...explicitFiles, ...globbed]) {
    if (isTsFile(file)) {
      merged.add(path.resolve(file));
    }
  }

  return Array.from(merged);
}

function isGlob(input: string): boolean {
  return /[*?[\]{}()!]/.test(input);
}

function isTsFile(filePath: string): boolean {
  return TS_EXTENSIONS.includes(path.extname(filePath).toLowerCase());
}
