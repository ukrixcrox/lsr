use std::path::PathBuf;
use colored::Colorize;
use crate::utils::{get_mtime, get_size, get_longes_file_size_len, get_file_permissions};
use is_executable::IsExecutable;

//TODO: sort entry: dir - file - hidden dir - hidden file
pub fn output(entry: Vec<PathBuf>){

    // collect all the file_sizes in a vec
    let file_size_len_vec: Vec<usize> = entry.iter().map(|entry| get_size(entry).len()).collect();

    // get the longes file_size len
    let file_size_len = get_longes_file_size_len(file_size_len_vec);

    let row_end_string = "-".repeat(55+file_size_len);

    println!("{}", row_end_string);
    println!("{:<11} | {:<width$} | {:<19} | {:<15}|", "Permissions", "Size", "Last modified", "Name", width=file_size_len);
    println!("{}", row_end_string);

    for entry in entry{
        let mtime = get_mtime(&entry);
        let file_size = get_size(&entry);
        let file_permissions = get_file_permissions(&entry);

        let entry_string = entry.file_name()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap();

        // format file size ouput to always right align 
        let file_size_output = format!("{:>width$}", file_size, width = file_size_len);
        
        //TODO: put this in a function or something, this is just stupid to work with
        if entry_string.starts_with('.') && !entry.is_dir(){
            println!("{:<11} | {} | {:>19} | {:<15}|", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.cyan());
        }else if entry.is_dir(){
            println!("{:<11} | {} | {:>19} | {:<15}|", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.bold().blue());
        }else if entry.is_symlink(){
            println!("{:<11} | {} | {:>19} | {:<15}|", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.yellow());
        }else if entry.is_executable(){
            println!("{:<11} | {} | {:>19} | {:<15}|", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.bold().green());
        }else{
            println!("{:<11} | {} | {:>19} | {:<15}|", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string);
        }
    }
    println!("{}", row_end_string);
}