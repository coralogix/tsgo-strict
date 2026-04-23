import { defineConfig } from 'vitepress';

const GITHUB = 'https://github.com/coralogix/internal-tsgo-strict';
const BASE = process.env.DOCS_BASE ?? '/';
const SITE_URL = process.env.DOCS_SITE_URL ?? `https://refactored-disco-1qv15pr.pages.github.io${BASE}`;
const DESCRIPTION =
  'Strict TypeScript. One file at a time. A fast, Rust-powered strict-only type checker built on Microsoft\'s tsgo.';

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
    ['meta', { name: 'theme-color', content: '#3178c6' }],
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
        text: 'v0.1',
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
      message: 'Released under the Apache 2.0 License.',
      copyright: `Copyright © ${new Date().getFullYear()} Coralogix`,
    },

    editLink: {
      pattern: `${GITHUB}/edit/master/docs/:path`,
      text: 'Edit this page on GitHub',
    },

    outline: { level: [2, 3] },
  },
});
