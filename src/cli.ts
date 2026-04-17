#!/usr/bin/env node
import path from 'node:path';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import { loadProjectContext } from './config/loadTsconfig.js';
import { findStrictCandidates } from './config/strictFileSelection.js';
import { resolveSubsetInputs } from './files/resolveSubset.js';
import { runTsgo } from './runner/tsgoRunner.js';
import { diffDiagnostics } from './diag/diff.js';
import { formatTextOutput } from './output/formatText.js';
import { formatJsonOutput } from './output/formatJson.js';
import { Timer } from './perf/timing.js';
import type { CliOptions, Diagnostic, Mode } from './types/index.js';

async function main(): Promise<void> {
  const argv = await yargs(hideBin(process.argv))
    .scriptName('tsgo-strict')
    .usage('$0 [fileOrGlob ...]')
    .option('project', {
      alias: 'p',
      type: 'string',
      default: 'tsconfig.json',
      describe: 'Path to tsconfig'
    })
    .option('json', {
      type: 'boolean',
      default: false,
      describe: 'Emit JSON diagnostics'
    })
    .option('pretty', {
      type: 'boolean',
      describe: 'Pretty diagnostic output from backend checker'
    })
    .option('trace-performance', {
      type: 'boolean',
      default: false,
      describe: 'Emit timing breakdown'
    })
    .option('strict-plugin', {
      type: 'string',
      default: 'typescript-strict-plugin',
      describe: 'Strict plugin name to inspect in compilerOptions.plugins'
    })
    .option('mode', {
      choices: ['exact', 'fast'] as const,
      default: 'exact',
      describe: 'Diagnostic mode'
    })
    .option('max-diagnostics', {
      type: 'number',
      describe: 'Maximum number of diagnostics to print'
    })
    .option('cwd', {
      type: 'string',
      default: process.cwd(),
      describe: 'Working directory'
    })
    .strictOptions()
    .help()
    .version()
    .parse();

  const options: CliOptions = {
    project: argv.project,
    json: argv.json,
    pretty: argv.pretty,
    tracePerformance: argv.tracePerformance,
    strictPlugin: argv.strictPlugin,
    mode: argv.mode as Mode,
    maxDiagnostics: argv.maxDiagnostics,
    cwd: path.resolve(argv.cwd),
    subsetInputs: argv._.map(String)
  };

  const timer = new Timer();
  timer.start('config-load');
  const context = loadProjectContext(options.cwd, options.project, options.strictPlugin);
  timer.end('config-load');

  timer.start('file-resolution');
  const subsetFiles = resolveSubsetInputs(options.subsetInputs, options.cwd);
  const projectScopeFiles = resolveProjectScope(context.projectFiles, subsetFiles);
  const strictCandidates = findStrictCandidates(
    projectScopeFiles,
    context.strictPluginConfig,
    context.configDir
  );
  const effectiveTargets = resolveEffectiveTargets(strictCandidates, subsetFiles);
  timer.end('file-resolution');

  if (effectiveTargets.length === 0) {
    emitNoDiagnostics(options, timer);
    process.exit(0);
  }

  let diagnostics: Diagnostic[];

  if (options.mode === 'fast') {
    timer.start('strict-run');
    const strictResult = await runTsgo({
      cwd: options.cwd,
      projectPath: context.projectPath,
      rawConfig: context.rawConfig,
      files: effectiveTargets,
      strictEnabled: true,
      pretty: options.pretty
    });
    timer.end('strict-run');

    diagnostics = filterToTargets(strictResult.diagnostics, effectiveTargets);
  } else {
    const parallel = process.env['TSGO_STRICT_PARALLEL'] !== '0';
    if (parallel) {
      timer.start('baseline-run');
      timer.start('strict-run');
      const baselinePromise = runTsgo({
        cwd: options.cwd,
        projectPath: context.projectPath,
        rawConfig: context.rawConfig,
        files: effectiveTargets,
        strictEnabled: false,
        pretty: options.pretty
      });
      const strictPromise = runTsgo({
        cwd: options.cwd,
        projectPath: context.projectPath,
        rawConfig: context.rawConfig,
        files: effectiveTargets,
        strictEnabled: true,
        pretty: options.pretty
      });
      const [baseline, strict] = await Promise.all([baselinePromise, strictPromise]);
      timer.end('baseline-run');
      timer.end('strict-run');

      timer.start('diff');
      diagnostics = diffDiagnostics(
        filterToTargets(strict.diagnostics, effectiveTargets),
        filterToTargets(baseline.diagnostics, effectiveTargets)
      );
      timer.end('diff');
    } else {
      timer.start('baseline-run');
      const baseline = await runTsgo({
        cwd: options.cwd,
        projectPath: context.projectPath,
        rawConfig: context.rawConfig,
        files: effectiveTargets,
        strictEnabled: false,
        pretty: options.pretty
      });
      timer.end('baseline-run');
      timer.start('strict-run');
      const strict = await runTsgo({
        cwd: options.cwd,
        projectPath: context.projectPath,
        rawConfig: context.rawConfig,
        files: effectiveTargets,
        strictEnabled: true,
        pretty: options.pretty
      });
      timer.end('strict-run');

      timer.start('diff');
      diagnostics = diffDiagnostics(
        filterToTargets(strict.diagnostics, effectiveTargets),
        filterToTargets(baseline.diagnostics, effectiveTargets)
      );
      timer.end('diff');
    }
  }

  timer.start('formatting');
  const errorDiagnostics = diagnostics.filter((d) => d.category === 'error');

  if (options.json) {
    const { text } = formatJsonOutput(errorDiagnostics, options.mode, options.maxDiagnostics);
    process.stdout.write(`${text}\n`);
  } else {
    const { text } = formatTextOutput(errorDiagnostics, options.cwd, options.maxDiagnostics);
    process.stdout.write(`${text}\n`);
  }
  timer.end('formatting');

  if (options.tracePerformance) {
    printTimings(timer);
  }

  process.exit(errorDiagnostics.length > 0 ? 1 : 0);
}

function resolveEffectiveTargets(strictCandidates: string[], subsetFiles: string[]): string[] {
  if (subsetFiles.length === 0) {
    return strictCandidates;
  }

  const subsetSet = new Set(subsetFiles.map(normalize));
  return strictCandidates.filter((file) => subsetSet.has(normalize(file)));
}

function resolveProjectScope(projectFiles: string[], subsetFiles: string[]): string[] {
  if (subsetFiles.length === 0) {
    return projectFiles;
  }

  const subsetSet = new Set(subsetFiles.map(normalize));
  return projectFiles.filter((file) => subsetSet.has(normalize(file)));
}

function filterToTargets(diagnostics: Diagnostic[], targets: string[]): Diagnostic[] {
  const targetSet = new Set(targets.map(normalize));

  return diagnostics.filter((diag) => {
    if (!diag.file) {
      return true;
    }
    return targetSet.has(normalize(diag.file));
  });
}

function normalize(file: string): string {
  return path.resolve(file).replace(/\\/g, '/').toLowerCase();
}

function emitNoDiagnostics(options: CliOptions, timer: Timer): void {
  if (options.json) {
    process.stdout.write(
      `${JSON.stringify(
        {
          mode: options.mode,
          errorCount: 0,
          diagnostics: [],
          truncated: false
        },
        null,
        2
      )}\n`
    );
  } else {
    process.stdout.write('Found 0 strict errors.\n');
  }

  if (options.tracePerformance) {
    printTimings(timer);
  }
}

function printTimings(timer: Timer): void {
  const entries = timer.entries();
  if (entries.length === 0) {
    return;
  }

  process.stderr.write('Performance timings (ms):\n');
  for (const entry of entries) {
    process.stderr.write(`  ${entry.label}: ${entry.durationMs}\n`);
  }
}

main().catch((error) => {
  process.stderr.write(
    `tsgo-strict error: ${error instanceof Error ? error.message : String(error)}\n`
  );
  process.exit(2);
});
