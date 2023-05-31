use std::path::PathBuf;
use colored::{Colorize, ColoredString};
use crate::utils::{get_mtime, get_size, get_longest_len, get_file_permissions};
use is_executable::IsExecutable;
use file_owner::PathExt;

enum LenVec{
    FileSize,
    FileName,
    FileGroup,
    FileOwner,
}

pub fn output(entry: Vec<PathBuf>){

    // don't know if thats better or just worse?    

    // get the longest file_size/name/group/owner len
    let file_size_len = file_len(LenVec::FileSize, &entry);
    let file_name_len = file_len(LenVec::FileName, &entry);
    let file_group_name_len = file_len(LenVec::FileGroup, &entry);
    let file_owner_name_len = file_len(LenVec::FileOwner, &entry);
    
    let row_end_string = "-".repeat(45+file_size_len+file_name_len+file_group_name_len+file_owner_name_len);

    println!("{}", row_end_string);
    println!("{:<11} | {:width_owner_name$} | {:<width$} | {:<19} | {:width_file_name$} |", "Permissions", "Owner", "Size", "Last modified", "Name", width=file_size_len, width_file_name=file_name_len, width_owner_name=file_owner_name_len+file_group_name_len+1);
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

        let file_group:String = entry_string.group().unwrap().name().unwrap().unwrap();
        let file_owner:String = entry_string.owner().unwrap().name().unwrap().unwrap();

        // format file size/name ouput to have enough width 
        let file_size_output = format!("{:>width$}", file_size, width = file_size_len);
        let file_name_output = format!("{:<width$}", entry_string, width=file_name_len);
        let file_group_output = format!("{:<width$}", file_group, width=file_group_name_len);
        let file_owner_output = format!("{:width$}", file_owner, width=file_owner_name_len);
        
        // TODO: WHY?

        //idk
        //at least it works
        if entry_string.starts_with('.') && !entry.is_dir(){
            print_output(&file_name_output.cyan(), &file_size_output, &file_permissions, &mtime, &file_owner_output, &file_group_output)
        }else if entry.is_dir(){
            print_output(&file_name_output.bold().blue(), &file_size_output, &file_permissions, &mtime, &file_owner_output, &file_group_output)
        }else if entry.is_symlink(){
            print_output(&file_name_output.yellow(), &file_size_output, &file_permissions, &mtime, &file_owner_output, &file_group_output)
        }else if entry.is_executable(){
            print_output(&file_name_output.bold().green(), &file_size_output, &file_permissions, &mtime, &file_owner_output, &file_group_output)
        }else{
            print_output(&file_name_output.white(), &file_size_output, &file_permissions, &mtime, &file_owner_output, &file_group_output)
        }
    }
    println!("{}", row_end_string);
}

fn print_output(entry_string:&ColoredString, file_size_output:&String, file_permissions:&String, mtime:&String, file_owner:&String, file_group:&String){
        println!("{:<11} | {:2} {:2} | {} | {:>19} | {} |", file_permissions.blue(), file_group.green(), file_owner.green(), file_size_output.purple(), mtime.yellow(), entry_string);
}


// get the longest len of the file name/size/group/owner as usize
fn file_len(len_vec: LenVec, entry: &Vec<PathBuf>) -> usize{
    match len_vec{
        LenVec::FileSize => get_longest_len(entry.iter().map(|entry| get_size(entry).len()).collect()),
        LenVec::FileName => get_longest_len(entry.iter().map(|entry| entry.file_name().unwrap().to_owned().into_string().unwrap().len()).collect()),
        LenVec::FileGroup => get_longest_len(entry.iter().map(|entry| entry.file_name().unwrap().to_owned().into_string().unwrap().group().unwrap().name().unwrap().unwrap().len()).collect()),
        LenVec::FileOwner => get_longest_len(entry.iter().map(|entry| entry.file_name().unwrap().to_owned().into_string().unwrap().owner().unwrap().name().unwrap().unwrap().len()).collect()),
    }
}