// Take a look at the license at the top of the repository in the LICENSE file.

// We want to ensure that all subclasses have:
//
// ```
// if !crate::rt::is_initialized() {
//     panic!("GTK has to be initialized first");
// }
// ```

use std::fs::{read_dir, read_to_string};
use std::path::Path;

fn check_file(f: &Path) -> usize {
    let s = read_to_string(f).expect("cannot read file");
    let mut checking_type: Option<String> = None;
    let mut found_is_initialized_check = false;

    for line in s.lines() {
        if checking_type.is_none() {
            if !line.starts_with("unsafe impl") || !line.contains(" IsSubclassable<") {
                continue;
            }
            if let Some(for_) = line.split(" for ").nth(1).and_then(|s| s.split(' ').next()) {
                checking_type = Some(for_.to_owned());
            }
            continue;
        }
        if line == "}" {
            // Analysis complete!
            break;
        }
        let trimmed = line.trim();
        if trimmed.contains("is_initialized()") {
            found_is_initialized_check = true;
        }
    }
    if !found_is_initialized_check {
        if let Some(for_) = checking_type {
            if for_ == "Application" {
                return 0;
            }
            eprintln!(
                "[{}] Missing `is_initialized()` check in subclass implementation (`{}`)",
                f.display(),
                for_
            );
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[test]
fn check_subclass_panic() {
    let mut errors = 0;

    for entry in read_dir("src/subclass").unwrap() {
        let entry = entry.expect("invalid entry");
        let path = entry.path();
        if path.is_file() {
            errors += check_file(&path);
        }
    }
    assert!(errors == 0);
}
