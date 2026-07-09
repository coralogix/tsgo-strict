import { execSync } from 'node:child_process';
import { defineConfig } from 'vitepress';

const GITHUB = 'https://github.com/coralogix/tsgo-strict';
const BASE = process.env.DOCS_BASE ?? '/tsgo-strict/';
const SITE_URL = process.env.DOCS_SITE_URL ?? `https://coralogix.github.io${BASE}`;
const DESCRIPTION =
  'Strict TypeScript. One file at a time. A fast, Rust-powered strict-only type checker built on Microsoft\'s tsgo.';

// Releases are tag-authoritative (see .github/workflows/release.yml) — the
// in-repo package.json versions are intentionally frozen, so the docs version
// label must come from the git tag, not from a hardcoded string. The release
// workflow passes the resolved version via DOCS_VERSION; otherwise we derive it
// from the latest local `vX.Y.Z` tag so local builds stay in sync too.
const VERSION = resolveVersion();

function resolveVersion(): string {
  const raw = process.env.DOCS_VERSION?.trim() || latestGitTag();
  if (!raw) return 'latest';
  return raw.startsWith('v') ? raw : `v${raw}`;
}

function latestGitTag(): string | undefined {
  try {
    return execSync("git tag --list 'v*' --sort=-v:refname", {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'ignore'],
    })
      .split('\n')
      .map((t: string) => t.trim())
      .find((t: string) => /^v\d+\.\d+\.\d+$/.test(t));
  } catch {
    return undefined;
  }
}

export default defineConfig({
  title: 'tsgo-strict',
  description: DESCRIPTION,
  base: BASE,
  lang: 'en-US',
  cleanUrls: true,
  lastUpdated: true,
  sitemap: { hostname: SITE_URL },
  head: [
    ['link', { rel: 'icon', type: 'image/svg+xml', href: `${BASE}logo.svg` }],
    ['meta', { name: 'theme-color', content: '#02763a' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'tsgo-strict — Strict TypeScript, one file at a time' }],
    ['meta', { property: 'og:description', content: DESCRIPTION }],
    ['meta', { property: 'og:url', content: SITE_URL }],
    ['meta', { property: 'og:image', content: `${SITE_URL}og-image.svg` }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'tsgo-strict' }],
    ['meta', { name: 'twitter:description', content: DESCRIPTION }],
    ['meta', { name: 'twitter:image', content: `${SITE_URL}og-image.svg` }],
  ],
  themeConfig: {
    logo: { src: '/logo.svg', width: 24, height: 24 },
    siteTitle: 'tsgo-strict',

    nav: [
      { text: 'Guide', link: '/guide/introduction', activeMatch: '/guide/' },
      { text: 'Reference', link: '/reference/cli', activeMatch: '/reference/' },
      { text: 'Benchmarks', link: '/benchmarks' },
      {
        text: VERSION,
        items: [
          { text: 'Changelog', link: `${GITHUB}/releases` },
          { text: 'Contributing', link: '/contributing' },
          { text: 'npm package', link: 'https://www.npmjs.com/package/tsgo-strict' },
        ],
      },
    ],

    sidebar: {
      '/guide/': [
        {
          text: 'Introduction',
          items: [
            { text: 'What is tsgo-strict?', link: '/guide/introduction' },
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'How it works', link: '/guide/how-it-works' },
          ],
        },
        {
          text: 'Usage',
          items: [
            { text: 'Configuration', link: '/guide/configuration' },
            { text: 'Pragmas', link: '/guide/pragmas' },
            { text: 'Incremental migration', link: '/guide/incremental-migration' },
          ],
        },
      ],
      '/reference/': [
        {
          text: 'Reference',
          items: [
            { text: 'CLI', link: '/reference/cli' },
            { text: 'Programmatic API', link: '/reference/api' },
            { text: 'Exit codes', link: '/reference/exit-codes' },
          ],
        },
      ],
    },

    socialLinks: [{ icon: 'github', link: GITHUB }],

    search: {
      provider: 'local',
      options: {
        detailedView: true,
      },
    },

    footer: {
      message: `<a href="https://coralogix.com" target="_blank" rel="noopener"><img class="cx-logo cx-logo--light" src="${BASE}coralogix-horizontal-light.svg" alt="Coralogix" width="160" /><img class="cx-logo cx-logo--dark" src="${BASE}coralogix-horizontal-dark.svg" alt="Coralogix" width="160" /></a><br />Built and maintained by Coralogix. Released under the Apache 2.0 License.`,
      copyright: `Copyright © ${new Date().getFullYear()} Coralogix`,
    },

    editLink: {
      pattern: `${GITHUB}/edit/master/docs/:path`,
      text: 'Edit this page on GitHub',
    },

    outline: { level: [2, 3] },
  },
});
