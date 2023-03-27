use std::path::Path;
use colored::Colorize;
use termion::raw::IntoRawMode;
use termion::cursor;
use std::io::{self, stdout, Write};
use crate::utils::get_mtime;

pub fn output(entry: &Path) -> io::Result<()> {
    let mtime = get_mtime(entry);

    // setting up terminal in raw mode 
    let stdout = stdout().into_raw_mode()?;
    let mut stdout = stdout.lock();

    let entry_string = entry.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();
    
    if entry_string.starts_with('.') && !entry.is_dir(){
        write!(stdout, "\n{}{} {}",cursor::Left(100),mtime, entry_string.red())?;
    }else if entry.is_dir(){
        write!(stdout, "\n{}{} {}", cursor::Left(100),mtime, entry_string.bold().blue())?;
    }else if entry.is_symlink(){
        write!(stdout, "\n{}{} {}", cursor::Left(100), mtime, entry_string.yellow())?;
    }else{
        write!(stdout, "\n{}{} {}", cursor::Left(100), mtime, entry_string)?;
    }
    Ok(())
}
