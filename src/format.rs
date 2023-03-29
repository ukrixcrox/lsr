use std::path::Path;
use colored::Colorize;
use termion::raw::IntoRawMode;
use termion::cursor;
use std::io::{self, stdout, Write};
use crate::utils::{get_mtime, get_size};

pub fn output(entry: &Path) -> io::Result<()> {
    let mtime = get_mtime(entry);
    let file_size = get_size(entry);


    // setting up terminal in raw mode 
    let stdout = stdout().into_raw_mode()?;
    let mut stdout = stdout.lock();

    let entry_string = entry.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();

    // format file size ouput to always right align 
    // TODO: somehow sometime i need to rewrite the whole thing prob,
    // because i first need to get all the lengths of the file sizes and then 
    // calculate the width to format the output the right way
    let file_size_output = format!("{:>width$}", file_size, width = 8);
    
    if entry_string.starts_with('.') && !entry.is_dir(){
        write!(stdout, "\n{} {} {} {}",cursor::Left(100), file_size_output.purple(), mtime, entry_string.red())?;
    }else if entry.is_dir(){
        write!(stdout, "\n{} {} {} {}", cursor::Left(100), file_size_output.purple(), mtime, entry_string.bold().blue())?;
    }else if entry.is_symlink(){
        write!(stdout, "\n{} {} {} {}", cursor::Left(100), file_size_output.purple(), mtime, entry_string.yellow())?;
    }else{
        write!(stdout, "\n{} {} {} {}", cursor::Left(100), file_size_output.purple(), mtime, entry_string)?;
    }
    Ok(())
}
