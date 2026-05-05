use regex::Regex;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

fn search_pattern_not_in_file(file_path: &str, pattern: &str) -> io::Result<Vec<(usize, String)>> {
    let path = Path::new(file_path);

    if !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "The path is not a file."));
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let regex = Regex::new(pattern).unwrap();

    let mut matching_lines = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if !regex.is_match(&line) {
            matching_lines.push((index + 1, line.to_string())); // Store line number and content
        }
    }

    Ok(matching_lines)
}


fn search_not_pattern(root_dir: &str, pattern: &str) {
    let queue: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let root_path = Path::new(root_dir).to_path_buf();
    let pattern = Arc::new(pattern.to_owned());

    queue.lock().unwrap().push(root_path);

    let thread1_queue = Arc::clone(&queue);
    let thread1_pattern = Arc::clone(&pattern);
    let thread1 = thread::spawn(move || {
        search_thread_not_matching_pattern(thread1_queue, thread1_pattern);
    });

    let thread2_queue = Arc::clone(&queue);
    let thread2_pattern = Arc::clone(&pattern);
    let thread2 = thread::spawn(move || {
        search_thread_not_matching_pattern(thread2_queue, thread2_pattern);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn search_files_not_matching_pattern(folder_path: &str, regex_pattern: &str) {
    let regex = Regex::new(regex_pattern).unwrap();

    for file in fs::read_dir(folder_path).unwrap() {
        if let Ok(file) = file {
            let file_name = file.file_name();

            if let Some(file_name) = file_name.to_str() {
                if !regex.is_match(file_name) {
                    println!("{}", file.path().display());
                }
            }
        }
    }
}

fn search_thread_not_matching_pattern(queue: Arc<Mutex<Vec<PathBuf>>>, pattern: Arc<String>) {
    loop {
        let search_case = {
            let mut queue = queue.lock().unwrap();
            if queue.is_empty() {
                break;
            }
            queue.remove(0)
        };

        if let Ok(metadata) = fs::metadata(&search_case) {
            if metadata.is_file() {
                match search_pattern_not_in_file(&search_case.to_string_lossy(), &pattern) {
                    Ok(matching_lines) => {
                            for (line_number, line) in matching_lines {
                                println!(
                                    "File Path: {}, Line {}: {}",
                                    search_case.to_string_lossy(),
                                    line_number,
                                    line
                        );
                        }
                    }
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            } else if metadata.is_dir() {
                search_files_not_matching_pattern(&search_case.to_string_lossy(), &pattern);
                if let Ok(entries) = fs::read_dir(&search_case) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Ok(child_path) = entry.path().into_os_string().into_string() {
                                let mut queue = queue.lock().unwrap();
                                queue.push(child_path.into());
                            }
                        }
                    }
                }
            }
        }
    }
}


fn search_pattern_in_file(file_path: &str, pattern: &str) -> io::Result<Vec<(usize, String)>> {
    let path = Path::new(file_path);

    if !path.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "The path is not a file."));
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);

    let regex = Regex::new(pattern).unwrap();

    let mut matching_lines = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if regex.is_match(&line) {
            matching_lines.push((index + 1, line.to_string())); // Store line number and content
        }
    }

    Ok(matching_lines)
}


fn search_files_matching_pattern(folder_path: &str, regex_pattern: &str) {
    let regex = Regex::new(regex_pattern).unwrap();

    for file in fs::read_dir(folder_path).unwrap() {
        if let Ok(file) = file {
            let file_name = file.file_name();

            if let Some(file_name) = file_name.to_str() {
                if regex.is_match(file_name) {
                    println!("{}", file.path().display());
                }
            }
        }
    }
}


fn search_thread(queue: Arc<Mutex<Vec<PathBuf>>>, pattern: Arc<String>) {
    loop {
        let search_case = {
            let mut queue = queue.lock().unwrap();
            if queue.is_empty() {
                break;
            }
            queue.remove(0)
        };

        if let Ok(metadata) = fs::metadata(&search_case) {
            if metadata.is_file() {
                match search_pattern_in_file(&search_case.to_string_lossy(), &pattern) {
                    Ok(matching_lines) => {
                            for (line_number, line) in matching_lines {
                                println!(
                                    "File Path: {}, Line {}: {}",
                                    search_case.to_string_lossy(),
                                    line_number,
                                    line
                        );
                        }
                    }
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            } else if metadata.is_dir() {
                search_files_matching_pattern(&search_case.to_string_lossy(), &pattern);
                if let Ok(entries) = fs::read_dir(&search_case) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Ok(child_path) = entry.path().into_os_string().into_string() {
                                let mut queue = queue.lock().unwrap();
                                queue.push(child_path.into());
                            }
                        }
                    }
                }
            }
        }
    }
}



fn search_thread_depth(queue: Arc<Mutex<Vec<(PathBuf, u32)>>>, pattern: Arc<String>, max_depth: u32) {
    loop {
        let (search_case, depth) = {
            let mut queue = queue.lock().unwrap();
            if queue.is_empty() {
                break;
            }
            queue.remove(0)
        };

        if let Ok(metadata) = fs::metadata(&search_case) {
            if metadata.is_file() {
                match search_pattern_in_file(&search_case.to_string_lossy(), &pattern) {
                    Ok(matching_lines) => {

                            for (line_number, line) in matching_lines {
                                println!(
                                    "File Path: {}, Line {}: {}",
                                    search_case.to_string_lossy(),
                                    line_number,
                                    line
                                );
                            
                        }
                    }
                    Err(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            } else if metadata.is_dir() && depth < max_depth {
                search_files_matching_pattern(&search_case.to_string_lossy(), &pattern);
                if let Ok(entries) = fs::read_dir(&search_case) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if let Ok(child_path) = entry.path().into_os_string().into_string() {
                                let mut queue = queue.lock().unwrap();
                                queue.push((child_path.into(), depth + 1));
                            }
                        }
                    }
                }
            }
        }
    }
}



fn search_directory_depth(root_dir: &str, pattern: &str, max_depth: u32) {
    let queue: Arc<Mutex<Vec<(PathBuf, u32)>>> = Arc::new(Mutex::new(Vec::new()));
    let root_path = Path::new(root_dir).to_path_buf();
    let pattern = Arc::new(pattern.to_owned());

    queue.lock().unwrap().push((root_path, 0));

    let thread1_queue = Arc::clone(&queue);
    let thread1_pattern = Arc::clone(&pattern);
    let thread1_max_depth = max_depth;
    let thread1 = thread::spawn(move || {
        search_thread_depth(thread1_queue, thread1_pattern, thread1_max_depth);
    });

    let thread2_queue = Arc::clone(&queue);
    let thread2_pattern = Arc::clone(&pattern);
    let thread2_max_depth = max_depth;
    let thread2 = thread::spawn(move || {
        search_thread_depth(thread2_queue, thread2_pattern, thread2_max_depth);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}



fn search(root_dir: &str, pattern: &str) {
    let queue: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let root_path = Path::new(root_dir).to_path_buf();
    let pattern = Arc::new(pattern.to_owned());

    queue.lock().unwrap().push(root_path);

    let thread1_queue = Arc::clone(&queue);
    let thread1_pattern = Arc::clone(&pattern);
    let thread1 = thread::spawn(move || {
        search_thread(thread1_queue, thread1_pattern);
    });

    let thread2_queue = Arc::clone(&queue);
    let thread2_pattern = Arc::clone(&pattern);
    let thread2 = thread::spawn(move || {
        search_thread(thread2_queue, thread2_pattern);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}
fn search_with_multiple_threads(root_dir: &str, pattern: &str, num_threads: u32) {
    let queue: Arc<Mutex<Vec<PathBuf>>> = Arc::new(Mutex::new(Vec::new()));
    let root_path = Path::new(root_dir);

    queue.lock().unwrap().push(PathBuf::from(root_path));
    let pattern_arc: Arc<String> = Arc::new(pattern.to_owned());

    let mut handles = vec![];
    for _ in 0..num_threads {
        let queue_clone = Arc::clone(&queue);
        let pattern_clone = Arc::clone(&pattern_arc);

        let handle = thread::spawn(move || {
            search_thread(queue_clone, pattern_clone);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        println!("Insufficient arguments. Usage: cargo run -- <mode> <path> <pattern> [<depth>]");
        return;
    }

    let mode = &args[1];
    let path = &args[2];
    let pattern = &args[3];

    match mode.as_str() {
        "f" => {
            match search_pattern_in_file(path, pattern) {
                Ok(matching_lines) => {
                    if matching_lines.is_empty() {
                        println!("No matching lines found.");
                    } else {
                        for (line_number, line) in matching_lines {
                            println!("File Path: {}, Line {}: {}", path, line_number, line);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("Error: {}", error);
                }
            }
        }
        "s" => {
            search(path, pattern);
        }
        "sd" => {
            if args.len() < 5 {
                println!("Insufficient arguments. Usage: cargo run -- sd <path> <pattern> <depth>");
                return;
            }

            let depth = match args[4].parse::<u32>() {
                Ok(d) => d,
                Err(_) => {
                    println!("Invalid depth value. Please provide a positive integer for depth.");
                    return;
                }
            };

            search_directory_depth(path, pattern, depth);
        }
        "rg" => {
            search_not_pattern(path, pattern);
        }
        "nt" => {
            if args.len() < 5 {
                println!("Insufficient arguments. Usage: cargo run -- nt <path> <pattern> <num_threads>");
                return;
            }

            let num_threads = match args[4].parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid number of threads. Please provide a positive integer.");
                    return;
                }
            };

            search_with_multiple_threads(path, pattern, num_threads);
        }
        _ => {
            println!("Invalid mode. Available modes: 'f' (file search), 's' (directory search), 'sd' (directory search with depth), 'rg' (search files not matching pattern or reverse grep), and 'nt' (directory search with multiple threads).");
        }
    }
}