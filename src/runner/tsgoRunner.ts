import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import { spawn } from 'node:child_process';
import type { Diagnostic, TsgoRunResult } from '../types/index.js';

const STRICT_FAMILY_FLAGS = [
  'strict',
  'strictBindCallApply',
  'strictBuiltinIteratorReturn',
  'strictFunctionTypes',
  'strictNullChecks',
  'strictPropertyInitialization',
  'useUnknownInCatchVariables',
  'noImplicitAny',
  'noImplicitThis',
  'noImplicitOverride',
  'noPropertyAccessFromIndexSignature',
  'noUncheckedIndexedAccess',
  'noUncheckedSideEffectImports',
  'exactOptionalPropertyTypes'
] as const;

interface RunInput {
  cwd: string;
  projectPath: string;
  rawConfig: Record<string, unknown>;
  files: string[];
  strictEnabled: boolean;
  pretty?: boolean;
}

export async function runTsgo(input: RunInput): Promise<TsgoRunResult> {
  const tempConfigPath = writeTempConfig(input);
  const started = Date.now();

  try {
    const args = [
      '--noEmit',
      '--pretty',
      input.pretty === false ? 'false' : 'true',
      '-p',
      tempConfigPath
    ];
    const binary = resolveTsgoBinary(input.cwd);

    const { stdout, stderr, exitCode } = await spawnCollect(binary, args, input.cwd);
    const diagnostics = parseDiagnostics(stdout, stderr, input.cwd);

    return {
      diagnostics,
      stdout,
      stderr,
      exitCode,
      durationMs: Date.now() - started
    };
  } finally {
    safeUnlink(tempConfigPath);
  }
}

function writeTempConfig(input: RunInput): string {
  const tempParent = path.join(input.cwd, '.tsgo-strict-tmp');
  fs.mkdirSync(tempParent, { recursive: true });
  const tempDir = fs.mkdtempSync(path.join(tempParent, 'run-'));
  const configPath = path.join(tempDir, input.strictEnabled ? 'strict.json' : 'baseline.json');

  const compilerOptions = {
    ...(extractCompilerOptions(input.rawConfig) ?? {}),
    noEmit: true
  } as Record<string, unknown>;

  for (const flag of STRICT_FAMILY_FLAGS) {
    compilerOptions[flag] = input.strictEnabled;
  }

  const tempConfig = {
    extends: input.projectPath,
    compilerOptions,
    files: input.files.map((file) => path.relative(tempDir, file).split(path.sep).join('/'))
  };

  fs.writeFileSync(configPath, `${JSON.stringify(tempConfig, null, 2)}\n`, 'utf8');
  return configPath;
}

function extractCompilerOptions(
  rawConfig: Record<string, unknown>
): Record<string, unknown> | undefined {
  const compilerOptions = rawConfig.compilerOptions;
  if (!compilerOptions || typeof compilerOptions !== 'object') {
    return undefined;
  }
  return compilerOptions as Record<string, unknown>;
}

function resolveTsgoBinary(cwd: string): string {
  const override = process.env.TSGO_BINARY;
  if (override && override.trim()) {
    return override.trim();
  }

  const localBin = path.join(
    cwd,
    'node_modules',
    '.bin',
    process.platform === 'win32' ? 'tsgo.cmd' : 'tsgo'
  );
  if (fs.existsSync(localBin)) {
    return localBin;
  }

  const require = createRequire(path.join(cwd, 'package.json'));
  try {
    const resolved = resolveBinaryFromPackage(require, 'tsgo');
    if (resolved) {
      return resolved;
    }
  } catch {
    // Fall back to additional package lookup and PATH.
  }

  try {
    const resolved = resolveBinaryFromPackage(require, '@typescript/native-preview');
    if (resolved) {
      return resolved;
    }
  } catch {
    // Fall back to PATH lookup.
  }

  return 'tsgo';
}

function resolveBinaryFromPackage(require: NodeRequire, packageName: string): string | undefined {
  const packageJsonPath = require.resolve(`${packageName}/package.json`);
  const packageDir = path.dirname(packageJsonPath);
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8')) as {
    bin?: string | Record<string, string>;
  };
  const binField = packageJson.bin;
  if (typeof binField === 'string') {
    return path.resolve(packageDir, binField);
  }
  if (binField && typeof binField === 'object') {
    const entry = binField.tsgo ?? Object.values(binField)[0];
    if (entry) {
      return path.resolve(packageDir, entry);
    }
  }
  return undefined;
}

function spawnCollect(
  binary: string,
  args: string[],
  cwd: string
): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  return new Promise((resolve, reject) => {
    const child = spawn(binary, args, {
      cwd,
      env: process.env,
      stdio: ['ignore', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    child.stdout.setEncoding('utf8');
    child.stderr.setEncoding('utf8');

    child.stdout.on('data', (chunk) => {
      stdout += chunk;
    });

    child.stderr.on('data', (chunk) => {
      stderr += chunk;
    });

    child.on('error', (error) => {
      reject(error);
    });

    child.on('close', (code) => {
      resolve({
        stdout,
        stderr,
        exitCode: code ?? 1
      });
    });
  });
}

function parseDiagnostics(stdout: string, stderr: string, cwd: string): Diagnostic[] {
  const lines = `${stdout}\n${stderr}`.split(/\r?\n/);
  const diagnostics: Diagnostic[] = [];
  let current: Diagnostic | null = null;

  for (const rawLine of lines) {
    const line = stripAnsi(rawLine).trimEnd();
    if (!line) {
      continue;
    }

    const parsed = parseDiagnosticLine(line, cwd);
    if (parsed) {
      if (current) {
        diagnostics.push(current);
      }
      current = parsed;
      continue;
    }

    if (current) {
      current.message = `${current.message}\n${line.trim()}`;
    }
  }

  if (current) {
    diagnostics.push(current);
  }

  return diagnostics;
}

function stripAnsi(value: string): string {
  let out = '';
  let i = 0;
  while (i < value.length) {
    const ch = value.charCodeAt(i);
    // ANSI CSI sequence: ESC [ ... m
    if (ch === 27 && value.charCodeAt(i + 1) === 91) {
      i += 2;
      while (i < value.length && value[i] !== 'm') {
        i += 1;
      }
      if (i < value.length) {
        i += 1;
      }
      continue;
    }
    out += value[i];
    i += 1;
  }
  return out;
}

function parseDiagnosticLine(line: string, cwd: string): Diagnostic | null {
  let match = /^(.*)\((\d+),(\d+)\):\s(error|warning)\sTS(\d+):\s(.*)$/.exec(line);
  if (match) {
    const [, file, l, c, category, code, message] = match;
    return {
      file: path.resolve(cwd, file),
      line: Number(l),
      column: Number(c),
      category: category as 'error' | 'warning',
      code: Number(code),
      message,
      rawLine: line
    };
  }

  match = /^(.*):(\d+):(\d+)\s-\s(error|warning)\sTS(\d+):\s(.*)$/.exec(line);
  if (match) {
    const [, file, l, c, category, code, message] = match;
    return {
      file: path.resolve(cwd, file),
      line: Number(l),
      column: Number(c),
      category: category as 'error' | 'warning',
      code: Number(code),
      message,
      rawLine: line
    };
  }

  match = /^(error|warning)\sTS(\d+):\s(.*)$/.exec(line);
  if (match) {
    const [, category, code, message] = match;
    return {
      category: category as 'error' | 'warning',
      code: Number(code),
      message,
      rawLine: line
    };
  }

  return null;
}

function safeUnlink(configPath: string): void {
  const dir = path.dirname(configPath);
  try {
    fs.unlinkSync(configPath);
  } catch {
    // ignore
  }
  try {
    fs.rmdirSync(dir);
  } catch {
    // ignore
  }
}
