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
    let mut buf = [0u8; HEAD_BYTES];
    let Ok(mut file) = File::open(path) else {
        return PragmaHint::None;
    };
    let read = match file.read(&mut buf) {
        Ok(n) => n,
        Err(_) => return PragmaHint::None,
    };
    classify_head(&buf[..read])
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
/// emulating the `\b` word boundary from the TS regex without allocating a Regex.
fn contains_pragma(haystack: &[u8], needle: &[u8]) -> bool {
    let mut i = 0;
    while i + needle.len() <= haystack.len() {
        if &haystack[i..i + needle.len()] == needle {
            let after = haystack.get(i + needle.len()).copied();
            match after {
                None => return true,
                Some(b) if !is_word_byte(b) => return true,
                _ => {}
            }
        }
        i += 1;
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
        assert_eq!(classify_head(b"// @ts-strict\nconst x = 1;"), PragmaHint::Strict);
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
        assert_eq!(classify_head(b"// @ts-strictx\nconst x = 1;"), PragmaHint::None);
    }

    #[test]
    fn empty_is_none() {
        assert_eq!(classify_head(b""), PragmaHint::None);
    }
}
