use std::fs;

#[derive(Debug)]
struct File {
    filename: String,
    length: usize,
}

#[derive(Debug)]
struct Directory {
    name: String,
    // List of references to directories
    directories: Vec<usize>,
    // List of references to files
    files: Vec<usize>
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();

    let mut directories = vec![
        Directory {
            name: String::from(""),
            directories: Vec::new(),
            files: Vec::new()
        }
    ];
    let mut files: Vec<File> = Vec::new();

    let mut current_directory_idx: usize = 0;

    println!("Current directory index: {}, current directory: {:?}", current_directory_idx, directories[current_directory_idx]);

    for line in file_contents.lines() {
        match line {
            // $ cd X
            line if line.starts_with("$ cd") => {
                let path = line.split(" ").nth(2).unwrap();
                println!("change directory: '{}'", path);
                if path == "/" {
                    current_directory_idx = 0;
                } else if path == ".." {
                    // Find directory for which current_directory_idx is in its directories
                    for (i, directory) in directories.iter().enumerate() {
                        if directory.directories.contains(&current_directory_idx) {
                            current_directory_idx = i;
                            break;
                        }
                    }
                    println!("cd .. got us to '{}'", directories[current_directory_idx].name);
                } else {
                    // Cd to this directory
                    let mut found = false;
                    for dir_idx in &directories[current_directory_idx].directories {
                        if directories[*dir_idx].name == path {
                            current_directory_idx = *dir_idx;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        // Create the directory
                        let new_dir = Directory {
                            name: String::from(path),
                            directories: Vec::new(),
                            files: Vec::new()
                        };
                        directories.push(new_dir);
                        let new_dir_idx = directories.len() - 1;
                        directories[current_directory_idx].directories.push(new_dir_idx);
                        current_directory_idx = new_dir_idx;
                    }
                }
            }
            // Ignore ls
            line if line.starts_with("$ ls") => {
                println!("ls");
            }
            // integer_file_size filename
            line if line.split(" ").nth(0).unwrap().parse::<usize>().is_ok() => {
                let file_size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
                let filename = line.split(" ").nth(1).unwrap();
                println!("file: '{}', size: {}", filename, file_size);
                let new_file = File {
                    filename: String::from(filename),
                    length: file_size
                };
                files.push(new_file);
                directories[current_directory_idx].files.push(files.len() - 1);
            }
            line => {
                println!("line: {}", line);
                let bits = line.split(" ").collect::<Vec<&str>>();
                println!(" . bits: {:?}", bits);
            }
        }
    }
    // Debug print files
    println!("Files:");
    for file in &files {
        println!(" . {:?}", file);
    }
    // Debug print directories
    println!("Directories:");
    for directory in &directories {
        println!(" . {:?}", directory);
    }

    // Determine the total size of each directory, including nested files
    let mut total_sizes = vec![0; directories.len()];
    
    // Looping through directories backwards should accumulate the total size of each directory
    for (i, directory) in directories.iter().enumerate().rev() {
        for file_idx in &directory.files {
            total_sizes[i] += files[*file_idx].length;
        }
        for dir_idx in &directory.directories {
            total_sizes[i] += total_sizes[*dir_idx];
        }
    }
    println!("Total sizes: {:?}", total_sizes);
    // Find all of the directories with a total size of at most 100000.
    let mut small_directories = Vec::new();
    for (i, size) in total_sizes.iter().enumerate() {
        if *size <= 100000 {
            small_directories.push(i);
        }
    }
    // What is the sum of the total sizes of those directories?
    let mut total_size_small_directories = 0;
    for dir_idx in &small_directories {
        total_size_small_directories += total_sizes[*dir_idx];
    }
    println!("Total size of small directories: {}", total_size_small_directories);

}
