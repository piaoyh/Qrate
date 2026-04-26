// Copyright 2026 PARK Youngho.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed
// except according to those terms.
///////////////////////////////////////////////////////////////////////////////


use std::path::Path;

// pub(crate) fn check_path(path: String, extention: &str) -> String
/// Checks if the given path has the specified extension.
/// If not, it appends the extension.
///
/// # Arguments
/// * `path` - The original file path.
/// * `extention` - The desired file extension (e.g., "txt", "docx").
///
/// # Returns
/// `String` - The path with the correct extension.
///
/// # Examples
/// ```
/// use qrate::check_path;
///
/// let path1 = "document.docx".to_string();
/// let checked_path1 = check_path(path1, "docx");
/// assert_eq!(checked_path1, "document.docx");
///
/// let path2 = "document".to_string();
/// let checked_path2 = check_path(path2, "docx");
/// assert_eq!(checked_path2, "document.docx");
/// ```
pub(crate) fn check_path(path: String, extention: &str) -> String
{
    if Path::new(&path).extension().and_then(|s| s.to_str()) == Some(extention)
        { path }
    else
        { format!("{}.{}", path, extention) }
}

