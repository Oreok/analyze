use std::fs;
use std::env;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;

struct FileStats {
    occurrences: Vec<usize>,
    lines_read: usize,
    files_read: usize,
    directories_read: usize,
    global_counter: usize,
}

impl FileStats {
    fn new() -> Self {
        let occurrences = vec![0; 26];
        FileStats {
            occurrences,
            lines_read: 0,
            files_read: 0,
            directories_read: 0,
            global_counter: 0,
        }
    }

    fn process_file(&mut self, path: &Path) {
        // Process the file
        match fs::File::open(path) {
            Ok(file) => {
                // Increment files_read count
                let reader = BufReader::new(file);
                self.files_read += 1;
                
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            self.lines_read += 1;

                            for ch in line.chars() {
                                if ch.is_alphabetic() && ch.is_lowercase() {
                                    let index = (ch as u8 - b'a') as usize;
                                    self.occurrences[index] += 1;
                                } else if ch.is_alphabetic() && ch.is_uppercase() {
                                    let index = (ch.to_lowercase().next().unwrap() as u8 - b'a') as usize;
                                    self.occurrences[index] += 1;
                                }
                            }

                        }
                        Err(err) => {
                            eprintln!("Failed to read line from file {}: {}", path.display(), err);
                            return;
                        }
                        
                    }
                }
                
                // increade op count
                self.global_counter += 1;
                println!("N: {} - File {} finished", self.global_counter, path.display());
            }
            Err(err) => {
                eprintln!("Failed to read file {}: {}", path.display(), err);
            }
        }
    }

    fn traverse_directory(&mut self, dir_path: &Path) {
        // Read the directory and handle potential errors
        let entries = match fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(err) => {
                eprintln!("Failed to read directory: {}", err);
                return;
            }
        };

        // Iterate over the directory entries
        for entry in entries {
            // Handle potential errors when reading an entry
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    eprintln!("Failed to read entry: {}", err);
                    continue;
                }
            };

            // Get the path of the entry
            let path = entry.path();

            // Check if it's a directory
            if path.is_dir() {
                // Recursively traverse the subdirectory
                self.traverse_directory(&path);
            } else if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("txt")) {
                // It's a file, process it
                self.process_file(&path);
            }
        }
        self.directories_read += 1;
        self.global_counter += 1;
        println!("N: {} - directory {} finished [dist={:?}, lines={}, files={}, directories={}]", self.global_counter, dir_path.display(), self.occurrences, self.lines_read, self.files_read, self.directories_read);
    }
}

fn main() {
    let start = Instant::now();
    let args: Vec<String> = env::args().collect();
    let dir_path = Path::new(&args[1]);
    let mut file_stats = FileStats::new();

    // Start traversing the directory
    file_stats.traverse_directory(&dir_path);

    // Convert occurrences to a vector and sort it alphabetically based on the keys

    // Print the counts in array format
    println!("Occurrences: {:?}", file_stats.occurrences);

    // Print the number of lines and files read
    println!("Lines read: {}", file_stats.lines_read);
    println!("Files read: {}", file_stats.files_read);
    println!("Directories read: {}", file_stats.directories_read);
    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}
