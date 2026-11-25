// Exit the shell
pub fn exit() -> i8 {
    print!("Exiting cb shell...");
    return 0
}

// Change current directory
pub fn cd(args:Vec<String>) -> i8 {
    use std::env;
    use std::path::Path;

    if args.len() == 1 {
        // Go to homedir if no args (args [0] is "cd") are given
        if !env::set_current_dir(env::home_dir().unwrap().as_path()).is_ok() {
            eprintln!("Couldn't change directory :(")
        } 
    } else {
        // Create path
        let path = Path::new(&args[1]);

        // Set the current dir and if is not ok display error message
        if !env::set_current_dir(&path).is_ok() {
            eprintln!("Couldn't change directory :(")
        }
    }   

    return 1
}

// Display the content of the current directory
pub fn ls(args:Vec<String>) -> i8 {
    use std::fs;
    use std::env;


    let current_dir = env::current_dir(); // Get current dir

    // Check current dir
    if !current_dir.is_ok() {
        eprint!("Couldn't find current directory");
        return 1;
    }

    // Initiate the vec of dir and files
    // let mut dirs = Vec::new();
    // let mut files = Vec::new();

    // Create an iterator on the current directory (unwraping current dir be cause checked above)
    let mut iterator = fs::read_dir(current_dir.unwrap()).unwrap(); 

    let mut flag = 0; // Initialize flag
    while flag == 0 {

        // Get the entry
        let entry = iterator.next();

        // Check if there is more entry (ending condition)
        if entry.is_none() {
            flag = 1;
        } else {

            let path = entry.unwrap().unwrap().path(); // Get path
            let file_name = path.file_name().expect("no file name").to_str().unwrap(); // Strip file name from path

            if file_name.contains('.') {
                // files.push(file_name);
                println!("   - {}", file_name);
            } else {
                // dirs.push(file_name);
                println!(" d - {}", file_name);
            }
        }
    }

    return 1
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