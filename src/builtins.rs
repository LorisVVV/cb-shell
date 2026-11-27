// Exit the shell
pub fn exit() -> i8 {
    println!("Exiting cb shell...");
    return 0
}

// Change current directory
pub fn cd(args:Vec<String>) -> i8 {
    use std::env;
    use std::path::Path;

    if args.len() < 2 {
        // Go to homedir if no args (args [0] is "cd") are given
        if !env::set_current_dir(env::home_dir().unwrap().as_path()).is_ok() {
            eprintln!("Couldn't change directory :(")
        } 
    } else {
        // Create path
        let path = Path::new(&args[1]);

        // Set the current dir and if is not ok display error message
        if !env::set_current_dir(&path).is_ok() {
            eprintln!("Directory \" {} \" not found", &args[1]);
        }
    }   

    return 1
}

// Display the content of the current directory, d shows directories
pub fn ls(args:Vec<String>) -> i8 {

    // Get the entries of the current dir
    let entries = get_current_files();

    // Initiate the vec of dir and files
    let mut dirs:Vec<String> = Vec::new();
    let mut files:Vec<String> = Vec::new();

    // For each entries
    for entry in entries {
        // Check if a file is a directory or a file
        if entry.contains('.') {
            files.push(entry);
        } else {
            dirs.push(entry);
        }
    }

    // Print each dirs
    for dir in dirs {
        println!(" d - {}", dir);
    }

    // Then print each files
    for file in files {
        println!("   - {}", file);
    }

    return 1
}

// Get the files of the current directory
pub fn get_current_files() -> Vec<String> {
    use std::fs;
    use std::env;

    // Initialie return vec
    let mut result:Vec<String> = Vec::new();

    // Get current dir unc for unchecked
    let mut current_dir_unc = env::current_dir(); 

    // Check current dir
    if !current_dir_unc.is_ok() {
        eprintln!("Couldn't find current directory");
        return result;
    }

    let current_dir = current_dir_unc.unwrap();

    // Create an iterator on the current directory (unwraping current dir be cause checked above)
    let mut iterator_unc = fs::read_dir(&current_dir); 

    if !iterator_unc.is_ok() {
        eprintln!("Couldn't find the entries for directory {}, you might not have the correct rights", current_dir.display());
        return result;
    }

    let mut iterator = iterator_unc.unwrap();

    let mut there_is_more_entry = true; // Initialize flag
    while there_is_more_entry { // While there is more entry

        // Get the entry
        let entry = iterator.next();

        // Check if there is more entry (ending condition)
        if entry.is_none() {
            there_is_more_entry = false;
        } else {

            let path = entry.unwrap().unwrap().path(); // Get path
            let file_name = path.file_name().expect("no file name").to_str().unwrap(); // Strip file name from path

            // Push in the result vec
            result.push(file_name.to_string());
        }
    }

    // Sort all of the entries
    result.sort();

    return result
}

// Exec a command by calling the cmd.exe file and give it the args (args starting at the second as the first will be 'wrapcmd')
pub fn wrapcmd(args:Vec<String>) -> i8 {
    use std::process::{Command};

    // Checking if the args as at least 2 value (1 is wrapcmd, 2 is the program to find)
    if args.len() > 1 {
        Command::new("cmd").arg("/C").args(&args[1..]).spawn().expect("Process couldn't be launch").wait().expect("We cannot be sure that the process is not a zombie now...");
    }

    return 1
}

// Execute an extern program by searching in every path with every exts
pub fn exec_extern_program(args:Vec<String>, paths:&Vec<String>, exts:&Vec<String>) -> i8 {
    use std::process::{Command};
    use std::path::Path;
    use std::env;

    // Get current dir
    let current_dir = env::current_dir(); 

    // Get the absolute path by joining the current dir with the first arg
    let absolute_path = Path::new(&current_dir.unwrap()).join(&args[0]);
    
    // If the program exist and is a file we launch the process
    if absolute_path.is_file() {
        Command::new(&args[0]).args(&args[1..]).spawn().expect("Process couldn't be launch").wait().expect("We cannot be sure that the process is not a zombie now...");

    } else { // Else we search the env path

        let mut programs = Vec::new(); // Initialize the vec to store all the programs
        // For every paths in the variable env and every extension possible
        for p in paths {
            for e in exts {
                let program_name = args[0].clone() + &e;
                let program_path = Path::new(&p).join(&program_name); // Creation of a path : path/to/program.extension

                // If the path exist
                if program_path.is_file() {
                    // println!("{} in {}", program_name, program_path.to_string_lossy());
                    programs.push(program_path); // The path is pushed to the vec
                }
            }
        }

        // If there is at least one program
        if programs.len() > 0 {
            // Launching the first program in the vec
            Command::new(&programs[0]).args(&args[1..]).spawn().expect("Process couldn't be launch").wait().expect("We cannot be sure that the process is not a zombie now...");
        } else {
            // Else print an error
            eprintln!("Program {} wasn't found...", &args[0]);
        }
    }
  

    return 1
}

pub fn auto_complete(query:String) -> Vec<String> {

    // Initialize the result vec
    let mut result:Vec<String> = Vec::new();

    let current_files = get_current_files();

    for file in current_files {

        let mut half_name = file.clone();

        half_name.truncate(query.len());

        if half_name.contains(&query) {
            result.push(file);
        }
    }

    return result
}