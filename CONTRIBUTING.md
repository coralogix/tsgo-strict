# Contributing

Thanks for contributing to `tsgo-strict`.

## Development setup

1. Install Node.js 20+.
2. Install dependencies: `pnpm install`.
3. Run checks locally:
   - `pnpm typecheck`
   - `pnpm lint`
   - `pnpm format`
   - `pnpm test`
   - `pnpm build`

## Pull requests

1. Keep PRs focused and small.
2. Add or update tests for behavior changes.
3. Ensure `pnpm prepush:check` passes before opening/updating a PR.
4. Update `README.md` when user-facing behavior changes.

## Commit quality

- Prefer descriptive commit messages that explain intent and impact.
- Keep generated files out of commits unless required for release artifacts.

## Reporting bugs and requesting features

Use GitHub Issues and include:

- Expected behavior
- Actual behavior
- Minimal reproduction (if possible)
- Environment details (Node version, OS, command used)
