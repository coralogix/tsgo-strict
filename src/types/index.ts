export type Mode = 'exact' | 'fast';

export interface CliOptions {
  project: string;
  json: boolean;
  pretty?: boolean;
  tracePerformance: boolean;
  strictPlugin: string;
  mode: Mode;
  maxDiagnostics?: number;
  cwd: string;
  subsetInputs: string[];
}

export interface StrictPluginConfig {
  name: string;
  paths?: string[];
  excludePattern?: string;
}

export interface ProjectContext {
  cwd: string;
  projectPath: string;
  configDir: string;
  projectFiles: string[];
  compilerOptions: Record<string, unknown>;
  rawConfig: Record<string, unknown>;
  strictPluginConfig: StrictPluginConfig | null;
}

export interface Diagnostic {
  file?: string;
  line?: number;
  column?: number;
  code: number;
  category: 'error' | 'warning' | 'message';
  message: string;
  rawLine?: string;
}

export interface TsgoRunResult {
  diagnostics: Diagnostic[];
  stdout: string;
  stderr: string;
  exitCode: number;
  durationMs: number;
}

export interface EffectiveSelection {
  effectiveTargets: string[];
  strictCandidates: string[];
}

export interface JsonOutput {
  mode: Mode;
  errorCount: number;
  diagnostics: Diagnostic[];
  truncated: boolean;
}
