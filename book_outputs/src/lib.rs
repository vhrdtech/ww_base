//! Golden outputs of the docs' examples, under `docs/generated/`. A test calls [check] with what a reader would see
//! when running an example; it fails when the file holds something else. With `WW_BOOK_OUTPUTS` set to `write`
//! (`just book-outputs`), [check] writes the file instead. Name such tests with `book_output` in their name, so that
//! `just book-outputs` finds them.

use std::fs;
use std::path::{Path, PathBuf};

/// Environment variable that makes [check] write its file: set to `write`.
const WRITE_ENV: &str = "WW_BOOK_OUTPUTS";

/// Where the outputs are, relative to the repository root.
pub const GENERATED_DIR: &str = "docs/generated";

/// Longest name of an output, in bytes.
const NAME_BYTES_MAX: usize = 128; // spirit-level: allow(exact-units-quantity) compared with str::len, a byte count

/// The repository root: the directory above this crate.
fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("book_outputs sits one level below the repository root")
        .to_path_buf()
}

/// Whether `name` is a relative path of plain segments: lowercase letters, digits, `_`, `-` and `.`.
fn valid_name(name: &str) -> bool {
    name.len() <= NAME_BYTES_MAX
        && name.split('/').all(|segment| {
            !segment.is_empty()
                && segment != "."
                && segment != ".."
                && segment.bytes().all(|b| {
                    b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')
                })
        })
}

/// Checks `docs/generated/<name>` against `text`, or writes it when `WW_BOOK_OUTPUTS` is `write`.
///
/// # Panics
///
/// When `name` is not a plain relative path, the file can't be read or written, or it holds something else than
/// `text`; the message names the first line that differs.
pub fn check(name: &str, text: &str) {
    assert!(valid_name(name), "`{name}` is not an output name");
    let file = repository_root().join(GENERATED_DIR).join(name);
    if std::env::var(WRITE_ENV).as_deref() == Ok("write") {
        let dir = file.parent().expect("a file has a directory");
        fs::create_dir_all(dir).expect("the generated directory can be made");
        fs::write(&file, text).expect("the output can be written");
        return;
    }
    let on_disk = match fs::read_to_string(&file) {
        Ok(on_disk) => on_disk,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => panic!("{}: {error}", file.display()),
    };
    if on_disk != text {
        let line = 1 + text
            .lines()
            .zip(on_disk.lines())
            .take_while(|(ours, theirs)| ours == theirs)
            .count();
        panic!(
            "{GENERATED_DIR}/{name} is stale from line {line}: run `just book-outputs`\n--- expected\n{text}--- on disk\n{on_disk}"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::valid_name;

    #[test]
    fn names_are_plain_relative_paths() {
        assert!(valid_name("ww_si/bytes.txt"));
        assert!(!valid_name("../bytes.txt"));
        assert!(!valid_name("/bytes.txt"));
        assert!(!valid_name("Bytes.txt"));
        assert!(!valid_name("a//b"));
    }
}
