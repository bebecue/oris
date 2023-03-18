fn main() {
    let mut passed_tests = 0;
    let mut failed_tests = 0;

    for entry in std::fs::read_dir("tests/pass/").unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();
        eprintln!("test {}", path.display());

        let ok = test_pass(&path);
        if ok {
            passed_tests += 1;
        } else {
            failed_tests += 1;
        }
    }

    for entry in std::fs::read_dir("tests/fail/").unwrap() {
        let entry = entry.unwrap();

        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "oris") {
            eprintln!("test {}", path.display());

            let ok = test_fail(&path);
            if ok {
                passed_tests += 1;
            } else {
                failed_tests += 1;
            }
        }
    }

    eprintln!("passed x {}, failed x {}", passed_tests, failed_tests);

    if failed_tests != 0 {
        panic!("failed test x {}", failed_tests);
    }
}

fn test_pass(path: &std::path::Path) -> bool {
    let code = std::fs::read(path).unwrap();

    let mut env = oris::Env::new();
    match oris::entry(&mut env, &code) {
        Ok(_) => true,
        Err(error) => {
            eprintln!("failed, error: {}", error);
            false
        }
    }
}

fn test_fail(path: &std::path::Path) -> bool {
    let code = std::fs::read(path).unwrap();

    let mut env = oris::Env::new();
    match oris::entry(&mut env, &code) {
        Ok(_) => {
            eprintln!(" runs ok while expecting to return an error");

            false
        }
        Err(error) => {
            let expected_error_file_path = path.with_extension("oris.error");
            match std::fs::read_to_string(&expected_error_file_path) {
                Ok(mut expected_error_string) => {
                    if expected_error_string.chars().next_back() == Some('\n') {
                        expected_error_string.pop();
                    }

                    let (line, column) = error.line_column(&code);
                    let found_error_string =
                        format!("{}:{}\n{}", line + 1, column + 1, error.to_string());

                    if found_error_string == expected_error_string {
                        true
                    } else {
                        eprintln!(" failed with an unexpected error:");
                        eprintln!("   found: \n{}", found_error_string);
                        eprintln!("expected: \n{}", expected_error_string);

                        false
                    }
                }
                Err(error_) => {
                    if error_.kind() == std::io::ErrorKind::NotFound {
                        eprintln!(" failed as expected with error: \n{}", error);
                        eprintln!(
                            "but expected error file is not found at path {}",
                            expected_error_file_path.display()
                        );
                    } else {
                        eprintln!(
                            "failed to read error file at {}",
                            expected_error_file_path.display()
                        );
                    }

                    false
                }
            }
        }
    }
}
