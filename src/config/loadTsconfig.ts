import fs from 'node:fs';
import path from 'node:path';
import { createRequire } from 'node:module';
import ts from 'typescript';
import type { ProjectContext, StrictPluginConfig } from '../types/index.js';

interface TsConfigJson {
  extends?: string;
  compilerOptions?: {
    plugins?: unknown[];
  };
}

export function loadProjectContext(
  cwd: string,
  projectArg: string,
  pluginName: string
): ProjectContext {
  const projectPath = path.resolve(cwd, projectArg);
  const configDir = path.dirname(projectPath);

  const parsed = ts.getParsedCommandLineOfConfigFile(
    projectPath,
    {},
    {
      ...ts.sys,
      onUnRecoverableConfigFileDiagnostic: (diagnostic) => {
        throw new Error(ts.formatDiagnostic(diagnostic, formatHost(cwd)));
      }
    }
  );

  if (!parsed) {
    throw new Error(`Unable to parse tsconfig at ${projectPath}`);
  }

  const diagnostics = parsed.errors ?? [];
  if (diagnostics.length > 0) {
    const message = diagnostics.map((d) => ts.formatDiagnostic(d, formatHost(cwd))).join('\n');
    throw new Error(message);
  }

  const rawConfig = readJsonConfig(projectPath);
  const mergedPluginConfig = resolvePluginConfig(projectPath, pluginName);

  return {
    cwd,
    projectPath,
    configDir,
    projectFiles: parsed.fileNames.map((file) => path.resolve(file)),
    compilerOptions: { ...(parsed.options as unknown as Record<string, unknown>) },
    rawConfig,
    strictPluginConfig: mergedPluginConfig
  };
}

function resolvePluginConfig(projectPath: string, pluginName: string): StrictPluginConfig | null {
  const chain = loadExtendsChain(projectPath);
  let matched: StrictPluginConfig | null = null;

  for (const cfg of chain) {
    const plugins = cfg.compilerOptions?.plugins;
    if (!Array.isArray(plugins)) {
      continue;
    }
    for (const plugin of plugins) {
      if (!plugin || typeof plugin !== 'object') {
        continue;
      }
      const record = plugin as Record<string, unknown>;
      if (record.name !== pluginName) {
        continue;
      }
      matched = {
        name: pluginName,
        paths: toStringArray(record.paths),
        excludePattern:
          typeof record.excludePattern === 'string' ? record.excludePattern : undefined
      };
    }
  }

  return matched;
}

function loadExtendsChain(projectPath: string): TsConfigJson[] {
  const visited = new Set<string>();
  const chain: TsConfigJson[] = [];

  let current: string | undefined = projectPath;
  while (current) {
    const resolved = path.resolve(current);
    if (visited.has(resolved)) {
      break;
    }
    visited.add(resolved);

    const config = readJsonConfig(resolved) as TsConfigJson;
    chain.unshift(config);

    const ext = config.extends;
    if (!ext) {
      break;
    }

    current = resolveExtendsPath(path.dirname(resolved), ext);
  }

  return chain;
}

function resolveExtendsPath(baseDir: string, ext: string): string {
  if (ext.startsWith('.') || ext.startsWith('/')) {
    return ensureJsonExtension(path.resolve(baseDir, ext));
  }

  try {
    return require.resolve(ext, { paths: [baseDir] });
  } catch {
    return ensureJsonExtension(path.resolve(baseDir, ext));
  }
}

function ensureJsonExtension(filePath: string): string {
  if (path.extname(filePath)) {
    return filePath;
  }
  return `${filePath}.json`;
}

function readJsonConfig(configPath: string): Record<string, unknown> {
  const result = ts.readConfigFile(configPath, (filePath) => fs.readFileSync(filePath, 'utf8'));
  if (result.error) {
    throw new Error(ts.formatDiagnostic(result.error, formatHost(path.dirname(configPath))));
  }
  return result.config as Record<string, unknown>;
}

function formatHost(cwd: string): ts.FormatDiagnosticsHost {
  return {
    getCanonicalFileName: (fileName) => fileName,
    getCurrentDirectory: () => cwd,
    getNewLine: () => '\n'
  };
}

function toStringArray(value: unknown): string[] | undefined {
  if (!Array.isArray(value)) {
    return undefined;
  }
  return value.filter((entry): entry is string => typeof entry === 'string');
}
const require = createRequire(import.meta.url);
