---
layout: home

title: tsgo-strict
titleTemplate: Strict TypeScript. One file at a time.

hero:
  name: tsgo-strict
  text: Strict TypeScript, one file at a time.
  tagline: "A fast, Rust-powered strict-only type checker built on Microsoft's tsgo — so you can adopt `strict: true` incrementally, without drowning the build."
  image:
    src: /logo.svg
    alt: tsgo-strict
  actions:
    - theme: brand
      text: Get started →
      link: /guide/getting-started
    - theme: alt
      text: Why tsgo-strict?
      link: /guide/introduction
    - theme: alt
      text: View on GitHub
      link: https://github.com/ashley-hunter/tsgo-strict-plugin

features:
  - icon: 🎯
    title: Strict, scoped precisely
    details: Flip <code>"strict": true</code> only for the files, directories, or globs you've opted in — everything else stays on your normal tsconfig.
  - icon: 🧭
    title: Pragma-driven overrides
    details: Drop a <code>// @ts-strict</code> or <code>// @ts-strict-ignore</code> at the top of a file and it wins over your plugin config. Migrate at your own pace.
  - icon: ⚡
    title: Rust-fast, tsgo-powered
    details: Config parsing in under a millisecond and parallel pragma scanning via rayon. <strong>2.48× faster</strong> end-to-end than the TS implementation.
  - icon: 🔌
    title: Zero-config plugin block
    details: Reuses <code>typescript-strict-plugin</code>'s config shape in <code>compilerOptions.plugins</code>. No extra config file, no custom runner.
  - icon: 🛠️
    title: CLI + programmatic API
    details: Use it from your scripts, your CI, or a Node process via the <code>run()</code> API that returns structured diagnostics and per-phase timings.
  - icon: 📦
    title: Prebuilt native binaries
    details: Ships as an npm launcher with per-platform packages — Linux (gnu/musl, x64/arm64), macOS (x64/arm64), Windows x64. No build step on install.
---

<div class="tss-stats">
  <div class="tss-stat">
    <div class="tss-stat-value">2.48×</div>
    <div class="tss-stat-label">faster full-project runs vs the TS port</div>
  </div>
  <div class="tss-stat">
    <div class="tss-stat-value">&lt;1 ms</div>
    <div class="tss-stat-label">tsconfig parsing (vs ~100 ms from Node startup)</div>
  </div>
  <div class="tss-stat">
    <div class="tss-stat-value">strict</div>
    <div class="tss-stat-label">flipped on only where you want — same override as the original plugin</div>
  </div>
  <div class="tss-stat">
    <div class="tss-stat-value">6</div>
    <div class="tss-stat-label">prebuilt native targets, one npm install</div>
  </div>
</div>

## Install

```bash
npm install --save-dev tsgo-strict @typescript/native-preview
# or
pnpm add -D tsgo-strict @typescript/native-preview
```

## Quickstart

Add a plugin block to your `tsconfig.json`:

```jsonc
{
  "compilerOptions": {
    "plugins": [
      {
        "name": "typescript-strict-plugin",
        "paths": ["./src/strict", "./src/shared/**/*.ts"],
        "excludePattern": "\\.test\\.ts$"
      }
    ]
  }
}
```

Then run the checker:

```bash
npx tsgo-strict
```

Files under `paths` are checked under strict mode. Everything else stays on
your normal tsconfig — and its diagnostics are filtered out of the output.
Drop a `// @ts-strict` or `// @ts-strict-ignore` at the top of any file to
override the config for that single file.

Ready to go deeper? Head to [Getting Started](/guide/getting-started) for a
full walkthrough, or [How it works](/guide/how-it-works) to see the pipeline.
