use std::fs;
use std::path::Path;
use clap::Parser;
use std::path::PathBuf;

mod format;
mod utils;

/// simple ls implementation in rust for the crust shell
#[derive(Parser)]
#[command(author, version)]
struct Opts{
    /// custom dir
    #[arg(default_value=".")]
    dir:String,

    /// shows everything thats in a dir
    #[arg(short='a')]
    #[arg(long="show-all")]
    show_all:bool,

    /// shows only dirs
    #[arg(short='d')]
    #[arg(long="dir-only")]
    dir_only:bool,

}

fn main() {
    let args = Opts::parse();
    let user_path = Path::new(&args.dir);

    if let Ok(entries) = fs::read_dir(user_path){

        let mut to_format_entries: Vec<PathBuf>= Vec::new();

        for entry in entries.flatten(){

                let entry = entry.path();
                let entry_string = entry.file_name()
                                        .unwrap()
                                        .to_os_string()
                                        .into_string()
                                        .unwrap();

                // works for now
                // normal output
                if !args.show_all && !args.dir_only && !entry_string.starts_with('.'){
                    to_format_entries.push(entry);
                // all flag output
                }else if args.show_all && !args.dir_only{
                        to_format_entries.push(entry);

                // all flag and dir_only flag output
                }else if args.show_all && args.dir_only && entry.is_dir(){
                    to_format_entries.push(entry);
                // dir_only flag output
                }else if !args.show_all && args.dir_only && entry.is_dir() && !entry_string.starts_with('.'){
                        to_format_entries.push(entry);
                }
                }

        format::output(to_format_entries);
        }
}
