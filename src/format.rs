use std::path::PathBuf;
use colored::{Colorize, ColoredString};
use crate::utils::{get_mtime, get_size, get_longest_len, get_file_permissions};
use is_executable::IsExecutable;

pub fn output(entry: Vec<PathBuf>){

    // collect all the file_sizes/name in a vec
    let file_size_len_vec: Vec<usize> = entry.iter().map(|entry| get_size(entry).len()).collect();
    let file_name_len_vec: Vec<usize> = entry.iter().map(|entry| entry.file_name().unwrap().to_owned().into_string().unwrap().len()).collect();

    // get the longes file_size/name len
    let file_size_len = get_longest_len(file_size_len_vec);
    let file_name_len = get_longest_len(file_name_len_vec);
    println!("{}", file_name_len);
    
    let row_end_string = "-".repeat(41+file_size_len+file_name_len);

    println!("{}", row_end_string);
    println!("{:<11} | {:<width$} | {:<19} | {:width_file_name$} |", "Permissions", "Size", "Last modified", "Name", width=file_size_len, width_file_name=file_name_len);
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

        // format file size/name ouput to always right align 
        let file_size_output = format!("{:>width$}", file_size, width = file_size_len);
        let file_name_output = format!("{:<width$}", entry_string, width=file_name_len);
        
        //idk
        //at least it works
        if entry_string.starts_with('.') && !entry.is_dir(){
            print_output(&file_name_output.cyan(), &file_size_output, &file_permissions, &mtime)
        }else if entry.is_dir(){
            print_output(&file_name_output.bold().blue(), &file_size_output, &file_permissions, &mtime)
        }else if entry.is_symlink(){
            print_output(&file_name_output.yellow(), &file_size_output, &file_permissions, &mtime)
        }else if entry.is_executable(){
            print_output(&file_name_output.bold().green(), &file_size_output, &file_permissions, &mtime)
        }else{
            print_output(&file_name_output.white(), &file_size_output, &file_permissions, &mtime)
        }
    }
    println!("{}", row_end_string);
}

fn print_output(entry_string:&ColoredString, file_size_output:&String, file_permissions:&String, mtime:&String){
        println!("{:<11} | {} | {:>19} | {} |", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string);
}