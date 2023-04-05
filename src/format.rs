use std::path::PathBuf;
use colored::Colorize;
use crate::utils::{get_mtime, get_size, get_longes_file_size_len, get_file_permissions};

//TODO: sort entry: dir - file - hidden dir - hidden file
pub fn output(entry: Vec<PathBuf>){

    // collect all the file_sizes in a vec
    let file_size_len_vec: Vec<usize> = entry.iter().map(|entry| get_size(entry).len()).collect();

    // get the longes file_size len
    let file_size_len = get_longes_file_size_len(file_size_len_vec);

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
        
        if entry_string.starts_with('.') && !entry.is_dir(){
            println!("{} {} {} {}", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.cyan());
        }else if entry.is_dir(){
            println!("{} {} {} {}", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.bold().blue());
        }else if entry.is_symlink(){
            println!("{} {} {} {}", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string.yellow());
        }else{
            println!("{} {} {} {}", file_permissions.blue(), file_size_output.purple(), mtime.yellow(), entry_string);
        }
    }
}
