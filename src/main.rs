
use std::env;
extern crate walkdir;
use walkdir::{WalkDir, DirEntry};

struct Arguments {
    max_depth: usize,
    exclude: Vec<String>,
    include: Vec<String>,
}

impl Arguments {
    pub fn new(args: &Vec<String>) -> Self {
        
        let mut arguments = Arguments {
            max_depth: 1,
            exclude: Vec::new(),
            include: Vec::new(),
        };

        for (index, s) in args.iter().enumerate() {
            if s.contains(&"--".to_string()) {
                if *s == "--max".to_string() {
                        arguments.max_depth = args[index + 1].parse().expect("Not a valid number");
                }
                if *s == "--exclude".to_string() {
                    arguments.exclude = args
                        .iter()
                        .skip(index + 1)
                        .take_while(|arg| !arg.starts_with('-'))
                        .cloned()
                        .collect();
                }
                if *s == "--include".to_string() {
                    arguments.include = args
                        .iter()
                        .skip(index + 1)
                        .take_while(|arg| !arg.starts_with('-'))
                        .cloned()
                        .collect();
                }
            }
        };

        arguments
    }
}

fn is_excluded(entry: &DirEntry, excludes: &[String]) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| excludes.contains(&name.to_string()))
        .unwrap_or(false)
}

fn is_included(entry: &DirEntry, includes: &[String]) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| includes.contains(&name.to_string()))
        .unwrap_or(false)
}

fn main() {

    
    let args: Vec<String> = env::args().collect();
    let arguments: Arguments = Arguments::new(&args);

    print!("Max Depth: {} \n", arguments.max_depth);
    println!("Excluding Files/Folders: {:?}", arguments.exclude);
    println!(" ");

    for entry in WalkDir::new(".")
        .max_depth(arguments.max_depth)
        .into_iter()
        .filter_entry(|e| !is_excluded(e, &arguments.exclude))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        let depth = entry.depth();

        
        if let Some(name) = path.file_name() {
            let indent = "  ".repeat(depth - 1);

            if path.is_dir() {
                println!("{}{}/", indent, name.to_string_lossy());
            } else if path.is_file() {
                println!("{}{}", indent, name.to_string_lossy());
            }
        }
    }
}
