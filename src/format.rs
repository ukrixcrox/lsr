use std::path::Path;
use colored::Colorize;

pub fn output(entry: &Path){
    let entry_string = entry.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();
    
    if entry_string.starts_with('.') && !entry.is_dir(){
        println!("{}", entry_string.red());
    }else if entry.is_dir(){
        println!("{}", entry_string.bold().blue());
    }else if entry.is_symlink(){
        println!("{}", entry_string.yellow());
    }else{
        println!("{}", entry_string);
    }
}
