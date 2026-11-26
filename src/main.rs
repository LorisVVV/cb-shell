use std::io::{stdin, BufRead, stdout, Write};
use std::env;
use std::fs::File;

mod builtins;

// Main function of the shell
fn main() {

    // Print ascii art
    // print_intro();


    // Get the different extension of file available
    let exts = get_exts_from_env();

    // Get the different path to search for programs
    let paths = get_paths_from_env();


    // Initialize status variable
    let mut status = 1;

    // Main loop
    while status == 1 {

        ask_for_cmd(); // Print current directory and symbol for input of command
        let args = get_cmd(); // Get and split line to get command
        status = exec_cmd(args, &paths, &exts); // Exec process and change status
    }

}

// Get and print current directory and arrow for input
fn ask_for_cmd() {

    // Get current dir from env
    let current_dir = env::current_dir();

    // If current dir is found display the command input prompt
    if current_dir.is_ok() {
        print!("CB {} > ", current_dir.unwrap().display().to_string());
        stdout().flush().unwrap();
    } else { // If not display error message
        eprint!("Error while getting the current directory");
    }

}

// Get the command input
fn get_cmd() -> Vec<String> {
    
    // Read the stdin
    let mut raw_input = String::new(); // Declare variable
    let mut handle = stdin().lock(); // Locking the stdin to create a handle
    let _ = handle.read_line(&mut raw_input); // Reading the line


    // Cut the raw input in arguments
    let trimmed_input = raw_input.trim();
    let mut args : Vec<String> = Vec::new(); // Initialize vec of args
    let mut word = String::new(); // Initialize word
    let bytes = trimmed_input.as_bytes(); // Parse the input into bytes

    let mut flag_in_str = false;
    // For each bytes, add word between two space or two "
    for (_i, &item) in bytes.iter().enumerate() {

        if item == b'"' && flag_in_str {
            args.push(word.clone());
            word.clear();
            flag_in_str = false;
        } else if item == b'"' && !flag_in_str {
            flag_in_str = true;
        } else if item == b' ' && !flag_in_str {
            args.push(word.clone());
            word.clear();
        } else {
            word.push(char::from(item));
        }
    }

    // Check last word has it doesn't have a space at the end
    if !word.is_empty() {
        args.push(word.clone());
        word.clear();
    }

    return args; // Return value
}

// Execute the command prompted
fn exec_cmd(args:Vec<String>, paths:&Vec<String>, exts:&Vec<String>) -> i8 {

    let mut status = 1;

    // Case where no args has been entered
    if args.len() < 1 {
        return status
    }

    // Check the builtins
    if args[0] == String::from("exit") {
        status = builtins::exit()
    } else if args[0] == String::from("cd") {
        status = builtins::cd(args);
    } else if args[0] == String::from("wrapcmd") {
        status = builtins::wrapcmd(args); // Pass the args to a cmd.exe and piped everything
    } else if args[0] == String::from("ls") {
        status = builtins::ls(args);
    } else { 
        // Try to find the program in the PATH files and execute it
        status = builtins::exec_extern_program(args, paths, exts);
    }

    return status
}

// Get all the type of extension from the env variable
fn get_exts_from_env() -> Vec<String> {

    let mut exts = Vec::new(); // Initalize new vec to get the exts as string

    // Get the variable from the key 'PATHEXT'
    match env::var_os("PATHEXT") {
        Some(raw_exts) => {
            // Pushing each extension in the vec
            for ext in env::split_paths(&raw_exts) {
                exts.push(ext.to_str().expect("Extension failed to parse to str").to_string());
            }
        }
        None => println!("PATHEXT is not defined in the environment.")
    }

    return exts
}

// Get all the path from the env variable
fn get_paths_from_env() -> Vec<String> {
    let mut paths = Vec::new(); // Initalize new vec to get the paths as string
    
    // Get the variable from the key 'PATH'
    match env::var_os("PATH") {
        Some(raw_paths) => {
            // Pushing each path in the vec
            for path in env::split_paths(&raw_paths) {
                paths.push(path.to_str().expect("Path failed to parse to str").to_string());
            }
        }
        None => println!("PATH is not defined in the environment.")
    }

    return paths
}


// Little intro ascii art
// fn print_intro() {
//     println!("		
//       ::::::::        :::::::::            ::::::::        :::    :::        ::::::::::        :::         :::  
//     :+:    :+:       :+:    :+:          :+:    :+:       :+:    :+:        :+:               :+:         :+:   
//    +:+              +:+    +:+          +:+              +:+    +:+        +:+               +:+         +:+    
//   +#+              +#++:++#+           +#++:++#++       +#++:++#++        +#++:++#          +#+         +#+     
//  +#+              +#+    +#+                 +#+       +#+    +#+        +#+               +#+         +#+      
// #+#    #+#       #+#    #+#          #+#    #+#       #+#    #+#        #+#               #+#         #+#       
// ########        #########            ########        ###    ###        ##########        ##########  ########## 
// ===============================================================================================================
//     ");
// }