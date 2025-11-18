#[cfg(test)]
mod tests {
    use assert_fs::prelude::*;
    use rust_find::find_by_name;
    use std::error::Error;

    #[test]
    fn find_by_name_test() -> Result<(), Box<dyn Error>> {
        // 1. Set up a temporary directory with a file hierarchy.
        let temp = assert_fs::TempDir::new()?;
        temp.child("a/main.rs").touch()?;
        temp.child("a/b/another_main.txt").touch()?;
        temp.child("other.rs").touch()?;
        temp.child("a/b/sub/and_another_main.rs").touch()?;

        // 2. Run the function to test against the temporary directory.
        let root_path = temp.path().to_str().unwrap();
        let substr_to_search = "main";
        let max_depth = 10; // High enough to search all subdirectories

        let mut found_files = find_by_name(root_path, substr_to_search, max_depth)?;

        // 3. Assert that the results are correct.
        // The order of results from the function is not guaranteed, so we sort both
        // the actual and expected results to ensure the comparison is stable.
        found_files.sort();

        let mut expected_files = vec![
            temp.child("a/main.rs").path().to_str().unwrap().to_string(),
            temp.child("a/b/another_main.txt")
                .path()
                .to_str()
                .unwrap()
                .to_string(),
            temp.child("a/b/sub/and_another_main.rs")
                .path()
                .to_str()
                .unwrap()
                .to_string(),
        ];
        expected_files.sort();

        assert_eq!(found_files, expected_files);

        // The temporary directory and its contents are automatically cleaned up
        // when `temp` goes out of scope at the end of the test.
        Ok(())
    }

    #[test]
    fn find_by_name_with_depth_limit_test() -> Result<(), Box<dyn Error>> {
        // Set up a similar file hierarchy
        let temp = assert_fs::TempDir::new()?;
        temp.child("a/main.rs").touch()?; // depth 1
        temp.child("a/b/another_main.txt").touch()?; // depth 2

        // Run the function with a depth limit of 1
        let root_path = temp.path().to_str().unwrap();
        let substr_to_search = "main";
        let max_depth = 1; // Only search in `a/`, not `a/b/`

        let mut found_files = find_by_name(root_path, substr_to_search, max_depth)?;
        found_files.sort();

        // We only expect to find the file at depth 1.
        let mut expected_files = vec![temp.child("a/main.rs").path().to_str().unwrap().to_string()];
        expected_files.sort();

        assert_eq!(found_files, expected_files);

        Ok(())
    }
}
