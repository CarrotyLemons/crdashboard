fn string_to_pathbuf(potential_path: Option<String>) -> Result<std::path::PathBuf, ()> {
    let path = match potential_path {
        Some(path) => path,
        None => return Err(()),
    };

    let path = std::path::Path::new(&path);
    let path = path.to_path_buf();
    Ok(path)
}

fn search_path_for_todo(aggregate_string: String, target_path: std::path::PathBuf) -> String {
    // Designed not to fail, if a file cannot be read the issue will not be propagated and instead ignored
    if target_path.is_file() {
        if target_path.ends_with("todo.md") {
            let file_contents = match std::fs::read(target_path) {
                Ok(contents) => contents,
                Err(_) => {
                    return aggregate_string;
                }
            };

            let file_contents = match std::str::from_utf8(&file_contents) {
                Ok(contents) => contents,
                Err(_) => {
                    return aggregate_string;
                }
            };

            let mut altered_string = aggregate_string;
            altered_string.push_str("\n\n");
            altered_string.push_str(file_contents);
            return altered_string;
        } else {
            return aggregate_string;
        };
    } else {
        let target_path = match target_path.read_dir() {
            Ok(contents) => contents,
            Err(_) => {
                return aggregate_string;
            }
        };

        let mut altered_string = aggregate_string;
        for child_path in target_path {
            let child_path = match child_path {
                Ok(contents) => contents,
                Err(_) => {
                    return altered_string;
                }
            };

            altered_string = search_path_for_todo(altered_string, child_path.path())
        }

        altered_string
    }
}

fn main() {
    let mut cli_arguments = std::env::args();

    match cli_arguments.next() {
        None => {
            println!("Arguments are required!")
        }
        Some(_) => {}
    };

    let search_dir_path = match string_to_pathbuf(cli_arguments.next()) {
        Ok(path) => path,
        Err(_) => {
            println!("Path to search dir is required!");
            return;
        }
    };

    let target_file_path = match string_to_pathbuf(cli_arguments.next()) {
        Ok(path) => path,
        Err(_) => {
            println!("Path to target dir is required!");
            return;
        }
    };

    let scraped_text = search_path_for_todo("".to_string(), search_dir_path);

    match std::fs::write(target_file_path, scraped_text) {
        Ok(_) => {}
        Err(issue) => {
            println!("Could not write file! {}", issue)
        }
    };
}
