use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const HEAD_BYTES: usize = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PragmaHint {
    /// No strict pragma found.
    None,
    /// `@ts-strict` present: force include, overrides paths/excludePattern.
    Strict,
    /// `@ts-strict-ignore` present: force exclude, short-circuits everything.
    Ignore,
}

/// Read the first `HEAD_BYTES` of `path` and classify the strict pragma.
///
/// Uses substring search (not regex) on the raw bytes. Matches the
/// `STRICT_COMMENT` / `STRICT_IGNORE_COMMENT` regexes in the TS impl
/// (`@ts-strict\b`, `@ts-strict-ignore\b`) precisely enough for our inputs:
/// TypeScript source files, ASCII pragmas. Silently returns `None` on any I/O
/// error, matching the TS `readHead` fallback behavior.
pub fn detect_pragma(path: &Path) -> PragmaHint {
    let Ok(mut file) = File::open(path) else {
        return PragmaHint::None;
    };
    // `read` is allowed to return fewer bytes than requested, so loop into a
    // stack buffer until the buffer is full or we hit EOF. Keeps the hot path
    // allocation-free while still catching pragmas that straddle a block
    // boundary below HEAD_BYTES.
    let mut buf = [0u8; HEAD_BYTES];
    let mut filled = 0;
    while filled < buf.len() {
        match file.read(&mut buf[filled..]) {
            Ok(0) => break,
            Ok(n) => filled += n,
            Err(_) => return PragmaHint::None,
        }
    }
    classify_head(&buf[..filled])
}

pub fn classify_head(head: &[u8]) -> PragmaHint {
    // Walk each line and apply the same shape TypeScript's
    // `STRICT_COMMENT` / `STRICT_IGNORE_COMMENT` regexes require:
    //     /^\s*\/\/\s*@ts-strict(-ignore)?\b/
    // i.e. optional leading whitespace, exactly `//`, optional whitespace,
    // then the pragma followed by a word boundary. Bare substring matching
    // (the previous implementation) over-triggered on commented-out pragmas,
    // string literals, and inline trailing comments — all of which the real
    // TS toolchain treats as non-pragmas.
    //
    // Ignore wins over Strict when both appear on different lines.
    let mut saw_strict = false;
    for line in head.split(|&b| b == b'\n') {
        match classify_line(line) {
            PragmaHint::Ignore => return PragmaHint::Ignore,
            PragmaHint::Strict => saw_strict = true,
            PragmaHint::None => {}
        }
    }
    if saw_strict {
        PragmaHint::Strict
    } else {
        PragmaHint::None
    }
}

/// Classify a single line against the TS pragma regex.
fn classify_line(line: &[u8]) -> PragmaHint {
    let rest = trim_ascii_whitespace_start(line);
    // Must start with exactly `//`. Note: `///` (triple-slash reference
    // directives) is not a valid pragma carrier either — after stripping
    // the first `//` the next char is `/`, which does not match `\s*@`.
    let Some(after_slashes) = rest.strip_prefix(b"//") else {
        return PragmaHint::None;
    };
    let after_slashes = trim_ascii_whitespace_start(after_slashes);

    // Check longest needle first so `@ts-strict-ignore` wins over `@ts-strict`.
    if starts_with_word(after_slashes, b"@ts-strict-ignore") {
        return PragmaHint::Ignore;
    }
    if starts_with_word(after_slashes, b"@ts-strict") {
        return PragmaHint::Strict;
    }
    PragmaHint::None
}

/// `\b`-anchored prefix check: does `haystack` start with `needle` followed
/// by a non-word byte (or end-of-input)?
fn starts_with_word(haystack: &[u8], needle: &[u8]) -> bool {
    let Some(tail) = haystack.strip_prefix(needle) else {
        return false;
    };
    match tail.first().copied() {
        None => true,
        Some(b) => !is_word_byte(b),
    }
}

fn trim_ascii_whitespace_start(buf: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < buf.len() && matches!(buf[i], b' ' | b'\t' | b'\r') {
        i += 1;
    }
    &buf[i..]
}

fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_strict() {
        assert_eq!(
            classify_head(b"// @ts-strict\nconst x = 1;"),
            PragmaHint::Strict
        );
    }

    #[test]
    fn detects_ignore() {
        assert_eq!(
            classify_head(b"// @ts-strict-ignore\nconst x = 1;"),
            PragmaHint::Ignore
        );
    }

    #[test]
    fn ignore_wins_over_strict() {
        assert_eq!(
            classify_head(b"// @ts-strict-ignore\n// @ts-strict\nconst x = 1;"),
            PragmaHint::Ignore
        );
    }

    #[test]
    fn word_boundary_required() {
        assert_eq!(
            classify_head(b"// @ts-strictx\nconst x = 1;"),
            PragmaHint::None
        );
    }

    #[test]
    fn empty_is_none() {
        assert_eq!(classify_head(b""), PragmaHint::None);
    }

    // ─── Line-oriented pragma matching (mirrors TS's STRICT_COMMENT regex) ────
    // The TS compiler and the original `typescript-strict-plugin` require the
    // pragma to appear on a line that starts with `//` (optionally indented).
    // A bare substring scan matches cases that the real TS toolchain does not,
    // turning "no-op" tokens (double-commented pragmas, strings, block
    // comments) into spurious PragmaHint::Ignore results.

    #[test]
    fn double_commented_ignore_is_not_a_pragma() {
        // `// // @ts-strict-ignore` — the outer `//` comments out an attempted
        // pragma. The old plugin regex /^\s*\/\/\s*@ts-strict-ignore\b/ does
        // NOT match because after the leading `//\s*` comes another `//`, not
        // the pragma token. tsgo-strict must match that behaviour.
        assert_eq!(
            classify_head(b"// // @ts-strict-ignore\nconst x = 1;"),
            PragmaHint::None
        );
        assert_eq!(
            classify_head(b"// // @ts-strict\nconst x = 1;"),
            PragmaHint::None
        );
    }

    #[test]
    fn block_comment_pragma_is_not_matched() {
        // Block comments are not valid pragma carriers per TS's regex.
        assert_eq!(
            classify_head(b"/* @ts-strict-ignore */\nconst x = 1;"),
            PragmaHint::None
        );
        assert_eq!(
            classify_head(b"/* @ts-strict */\nconst x = 1;"),
            PragmaHint::None
        );
    }

    #[test]
    fn pragma_inside_string_literal_is_not_matched() {
        assert_eq!(
            classify_head(b"const msg = \"@ts-strict-ignore\";\n"),
            PragmaHint::None
        );
    }

    #[test]
    fn leading_whitespace_allowed_before_comment() {
        assert_eq!(
            classify_head(b"    // @ts-strict-ignore\nconst x = 1;"),
            PragmaHint::Ignore
        );
        assert_eq!(
            classify_head(b"\t// @ts-strict-ignore\nconst x = 1;"),
            PragmaHint::Ignore
        );
    }

    #[test]
    fn no_space_between_slashes_and_pragma() {
        // `//@ts-strict-ignore` (no space) is still valid per TS regex
        // `^\s*\/\/\s*@ts-strict-ignore\b` — the `\s*` between `//` and the
        // pragma matches zero characters.
        assert_eq!(
            classify_head(b"//@ts-strict-ignore\nconst x = 1;"),
            PragmaHint::Ignore
        );
    }

    #[test]
    fn pragma_on_later_line_matches() {
        // The pragma doesn't need to be the first line, as long as it's a
        // valid `//` line somewhere in the head block.
        assert_eq!(
            classify_head(b"import { x } from 'y';\n// @ts-strict-ignore\nconst x = 1;"),
            PragmaHint::Ignore
        );
    }

    #[test]
    fn pragma_after_non_comment_code_on_same_line_not_matched() {
        // `const x = 1; // @ts-strict-ignore` — the `//` is not at line start,
        // so the old plugin regex (anchored with ^) would not match. This
        // prevents inline trailing comments on arbitrary lines from marking a
        // file as strict-ignored.
        assert_eq!(
            classify_head(b"const x = 1; // @ts-strict-ignore\n"),
            PragmaHint::None
        );
    }
}
