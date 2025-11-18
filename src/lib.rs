use std::{
    collections::VecDeque,
    env,
    fs::read_dir,
    io::{self, Error},
    path::{Path, PathBuf},
};

use log::{debug, error, info, warn};

pub mod models;

/// Converts a given path to an absolute path.
///
/// If the provided path is already absolute, it is returned as is.
/// If the path is relative, it is joined with the current working directory
/// to produce an absolute path.
///
/// # Arguments
///
/// * `path` - A type that can be referenced as a `Path`, e.g., a `&str` or `PathBuf`.
///
/// # Returns
///
/// A `Result` containing a `PathBuf` with the absolute path on success,
/// or an `io::Error` if the current working directory cannot be determined.
fn to_absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        let cwd = env::current_dir()?;
        Ok(cwd.join(path))
    }
}

/// Searches for files containing a given substring in their name, starting from an initial path.
///
/// This function performs a depth-first search up to a specified maximum depth. It returns a
/// vector of strings, where each string is the absolute path of a found file.
///
/// # Arguments
///
/// * `initial_path` - The starting directory for the search. Can be a relative or absolute path.
/// * `substr_to_search` - The substring to search for in the filenames.
/// * `max_folders_depth` - The maximum depth of subdirectories to search into. A depth of 0
///   means only the initial path will be searched.
///
/// # Returns
///
/// A `Result` containing a `Vec<String>` of absolute paths for all matching files found.
/// Returns an empty vector if no files are found.
///
/// # Errors
///
/// This function will return an `io::Error` if:
/// * The `initial_path` does not exist or is not a directory.
/// * There are permission issues reading the `initial_path`.
/// * The current working directory cannot be determined when `initial_path` is relative.
///
/// Note: The function logs errors for any subdirectories it fails to read but does not
/// return an error, allowing the search to continue in other parts of the directory tree.
///
/// # Panics
///
/// This function may panic if it encounters a file path that is not valid UTF-8.
/// This is due to the use of `.unwrap()` during path-to-string conversion.
pub fn find_by_name(
    initial_path: &str,
    substr_to_search: &str,
    max_folders_depth: i32,
) -> io::Result<Vec<String>> {
    debug!("Validating data");
    // Validates the provided path
    let current_path = PathBuf::from(initial_path);
    match current_path.try_exists() {
        Ok(exists) => {
            if !exists {
                error!("The provided path does not exist");
                return Err(Error::new(
                    io::ErrorKind::NotFound,
                    format!("{initial_path}"),
                ));
            }
            if !current_path.is_dir() {
                error!("The provided path is a file not a directory");
                return Err(Error::new(
                    io::ErrorKind::NotADirectory,
                    format!("{initial_path}"),
                ));
            }
        }
        Err(_e) => {
            error!("Can't verify the given path. Check permissions.");
            return Err(Error::new(
                io::ErrorKind::PermissionDenied,
                format!("{initial_path}"),
            ));
        }
    }
    debug!("Starting recursion");
    // Starts recursion
    let mut max_depth_reached: bool = false;
    let mut total_files_inspected = 0;
    let mut found_files: Vec<String> = Vec::new();
    let mut to_explore: VecDeque<models::DirWithDepth> = VecDeque::new();
    to_explore.push_back(models::DirWithDepth {
        dir: to_absolute_path(current_path)?,
        depth: 0,
    });

    while let Some(current_path) = to_explore.pop_back() {
        // Reads the current directory
        match read_dir(current_path.dir.as_path()) {
            Ok(read_dir_obj) => {
                for entry_res in read_dir_obj {
                    match entry_res {
                        Ok(dir_entry_obj) => {
                            let tmp_path = dir_entry_obj.path();
                            if tmp_path.is_file() {
                                total_files_inspected += 1;
                                if let Some(filename) =
                                    tmp_path.file_name().and_then(|f| f.to_str())
                                {
                                    if filename.contains(substr_to_search) {
                                        let file_fp = tmp_path.to_str().unwrap().to_owned();
                                        found_files.push(file_fp);
                                    }
                                }
                            } else if tmp_path.is_dir() {
                                if current_path.depth < max_folders_depth {
                                    // Add dirs to the queue
                                    to_explore.push_back(models::DirWithDepth {
                                        dir: tmp_path,
                                        depth: current_path.depth + 1,
                                    });
                                } else {
                                    max_depth_reached = true;
                                }
                            }
                        }
                        Err(e) => error!("Error on dir_entry: {:?}", e),
                    }
                }
            }
            Err(e) => {
                error!(
                    "Error reading directory: {} - {:?}",
                    current_path.dir.to_str().unwrap(),
                    e
                );
            }
        }
    }
    info!("Inspected files: {total_files_inspected}");
    if max_depth_reached {
        warn!("Max depth reached: max_folders_depth={max_folders_depth}")
    }
    Ok(found_files)
}
