//! Methods to interact with the file system to manage memory in an intelligent
//! way.

use std::fs::{read_to_string, write};

/// Tries to read the value if it was already computed. Otherwise, it creates it
/// and writes for further usage.
pub fn read_write<F>(path: &str, create: F) -> Result<String, String>
where
    F: Fn() -> Result<String, String>,
{
    read_to_string(path).map_or_else(
        |_| {
            let content = create()?;
            write(path, &content)
                .map(|()| content)
                .map_err(|err| format!("Failed to write to file at path {path}:\n{err}"))
        },
        Ok,
    )
}
