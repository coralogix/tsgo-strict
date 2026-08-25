import { execSync } from 'node:child_process';
import { defineConfig } from 'vitepress';

const GITHUB = 'https://github.com/coralogix/tsgo-strict';
// Tagged so OSS-driven traffic to coralogix.com is attributable per project.
const CORALOGIX_URL =
  'https://coralogix.com/?utm_source=tsgo-strict-docs&utm_medium=oss&utm_campaign=tsgo-strict';
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
    // Coralogix mark, matching galeforce-css. The tsgo-strict logo stays as the
    // navbar mark; the favicon is the company-level signal.
    ['link', { rel: 'icon', type: 'image/svg+xml', href: `${BASE}coralogix-mark.svg` }],
    ['meta', { name: 'theme-color', content: '#02763a' }],
    // Nunito Sans + Inconsolata are the Coralogix design system's families
    // (tailwind.theme.ts `fontFamily`). Served from Google Fonts rather than
    // vendored: the design system ships TTFs, which are several hundred kB
    // heavier than the woff2 the CDN negotiates. Without these the families
    // named in custom.css silently fall through to the stack defaults.
    ['link', { rel: 'preconnect', href: 'https://fonts.googleapis.com' }],
    ['link', { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' }],
    [
      'link',
      {
        rel: 'stylesheet',
        href:
          'https://fonts.googleapis.com/css2?family=Nunito+Sans:ital,opsz,wght@0,6..12,400..800;1,6..12,400..800' +
          '&family=Inconsolata:wght@400..700&display=swap',
      },
    ],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'tsgo-strict — Strict TypeScript, one file at a time' }],
    ['meta', { property: 'og:description', content: DESCRIPTION }],
    ['meta', { property: 'og:url', content: SITE_URL }],
    // PNG, not the SVG it was rendered from: no major social platform
    // (X, Slack, LinkedIn, Facebook, Discord) rasterises SVG for a link
    // preview, so an SVG here means every shared link shows a blank card.
    // Regenerate after editing og-image.svg:
    //   npx @resvg/resvg-js-cli docs/public/og-image.svg -o docs/public/og-image.png --width 1200
    ['meta', { property: 'og:image', content: `${SITE_URL}og-image.png` }],
    ['meta', { property: 'og:image:type', content: 'image/png' }],
    ['meta', { property: 'og:image:width', content: '1200' }],
    ['meta', { property: 'og:image:height', content: '630' }],
    ['meta', { property: 'og:image:alt', content: 'tsgo-strict — Strict TypeScript, one file at a time' }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: 'tsgo-strict' }],
    ['meta', { name: 'twitter:description', content: DESCRIPTION }],
    ['meta', { name: 'twitter:image', content: `${SITE_URL}og-image.png` }],
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
          { text: 'npm package', link: 'https://www.npmjs.com/package/@coralogix/tsgo-strict' },
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
      message: 'Released under the Apache License 2.0.',
      copyright: [
        'Built with 💚 by',
        `<a href="${CORALOGIX_URL}" target="_blank" rel="noopener">`,
        `<img src="${BASE}coralogix-mark.svg" alt="" width="14" height="14"` +
          ' style="display:inline-block;vertical-align:-2px">',
        'Coralogix</a>',
      ].join(' '),
    },

    editLink: {
      pattern: `${GITHUB}/edit/master/docs/:path`,
      text: 'Edit this page on GitHub',
    },

    outline: { level: [2, 3] },
  },
});
