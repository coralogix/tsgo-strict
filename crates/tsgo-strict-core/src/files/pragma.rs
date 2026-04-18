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
    // @ts-strict-ignore first: if present, Ignore wins regardless of Strict.
    if contains_pragma(head, b"@ts-strict-ignore") {
        return PragmaHint::Ignore;
    }
    if contains_pragma(head, b"@ts-strict") {
        return PragmaHint::Strict;
    }
    PragmaHint::None
}

/// Find `needle` in `haystack` and require a non-word character (or EOF) after it,
/// emulating the `\b` word boundary from the TS regex. Uses SIMD-accelerated
/// `memmem` instead of a naive byte loop.
fn contains_pragma(haystack: &[u8], needle: &[u8]) -> bool {
    for pos in memchr::memmem::find_iter(haystack, needle) {
        match haystack.get(pos + needle.len()).copied() {
            None => return true,
            Some(b) if !is_word_byte(b) => return true,
            _ => {}
        }
    }
    false
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
}
