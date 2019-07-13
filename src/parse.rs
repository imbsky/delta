use std::path::Path;

/// Given input like
/// "diff --git a/src/main.rs b/src/main.rs"
/// Return "rs", i.e. a single file extension consistent with both files.
pub fn get_file_extension_from_diff_line(line: &str) -> Option<&str> {
    match get_file_extensions_from_diff_line(line) {
        (Some(ext1), Some(ext2)) => {
            if ext1 == ext2 {
                Some(ext1)
            } else {
                // Unexpected: old and new files have different extensions.
                None
            }
        }
        (Some(ext1), None) => Some(ext1),
        (None, Some(ext2)) => Some(ext2),
        (None, None) => None,
    }
}

// TODO: Don't parse the line twice (once for change description and once for extensions).
pub fn get_file_change_description_from_diff_line(line: &str) -> String {
    match get_file_paths_from_diff_line(line) {
        (Some(file_1), Some(file_2)) if file_1 == file_2 => format!("{}", file_1),
        (Some(file), Some("/dev/null")) => format!("deleted: {}", file),
        (Some("/dev/null"), Some(file)) => format!("added: {}", file),
        (Some(file_1), Some(file_2)) => format!("renamed: {} ⟶  {}", file_1, file_2),
        _ => format!("?"),
    }
}

/// Given input like
/// "@@ -74,15 +74,14 @@ pub fn delta("
/// Return " pub fn delta("
pub fn parse_hunk_metadata(line: &str) -> (String, String) {
    let mut iter = line.split("@@").skip(1);
    let line_number = iter
        .next()
        .and_then(|s| {
            s.split("+")
                .skip(1)
                .next()
                .and_then(|s| s.split(",").next())
        })
        .unwrap_or("")
        .to_string();
    let code_fragment = iter.next().unwrap_or("").to_string();
    (code_fragment, line_number)
}

fn get_file_paths_from_diff_line(line: &str) -> (Option<&str>, Option<&str>) {
    let mut iter = line.split(" ");
    iter.next(); // diff
    iter.next(); // --git
    (
        iter.next().and_then(|s| Some(&s[2..])),
        iter.next().and_then(|s| Some(&s[2..])),
    )
}

/// Given input like "diff --git a/src/main.rs b/src/main.rs"
/// return ("rs", "rs").
fn get_file_extensions_from_diff_line(line: &str) -> (Option<&str>, Option<&str>) {
    let mut iter = line.split(" ");
    iter.next(); // diff
    iter.next(); // --git
    (
        iter.next().and_then(|s| get_extension(&s[2..])),
        iter.next().and_then(|s| get_extension(&s[2..])),
    )
}

/// Attempt to parse input as a file path and return extension as a &str.
fn get_extension(s: &str) -> Option<&str> {
    let path = Path::new(s);
    path.extension()
        .and_then(|e| e.to_str())
        // E.g. 'Makefile' is the file name and also the extension
        .or_else(|| path.file_name().and_then(|s| s.to_str()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_extension_from_diff_line() {
        assert_eq!(
            get_file_extension_from_diff_line("diff --git a/src/main.rs b/src/main.rs"),
            Some("rs")
        );
    }

    #[test]
    fn test_get_file_change_description_from_diff_line() {
        assert_eq!(
            get_file_change_description_from_diff_line("diff --git a/src/main.rs b/src/main.rs"),
            "src/main.rs"
        );
    }

    #[test]
    fn test_parse_hunk_metadata() {
        assert_eq!(
            parse_hunk_metadata("@@ -74,15 +75,14 @@ pub fn delta(\n"),
            (" pub fn delta(\n".to_string(), "75".to_string())
        );
    }
}
