use std::fs;
use std::path::Path;
use clap::Parser;

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

    //TODO: need to add the long format before adding color for executable files
    //because what is executable is user specific and you can see that on linux in the permissions
}

fn main() {
    let args = Opts::parse();
    let user_path = Path::new(&args.dir);

    if let Ok(entries) = fs::read_dir(user_path){
        for entry in entries.flatten(){

                let entry = entry.path();
                let entry_string = entry.file_name()
                                        .unwrap()
                                        .to_os_string()
                                        .into_string()
                                        .unwrap();

                // works for now
                // normal output
                if !args.show_all && !args.dir_only{
                    if !entry_string.starts_with('.'){
                        format::output(&entry).unwrap();
                    }
                // all flag output
                }else if args.show_all && !args.dir_only{
                    format::output(&entry).unwrap();

                // all flag and dir_only flag output
                }else if args.show_all && args.dir_only{
                    if entry.is_dir(){
                        format::output(&entry).unwrap();
                    }
                // dir_only flag output
                }else if !args.show_all && args.dir_only && entry.is_dir() && !entry_string.starts_with('.'){
                        format::output(&entry).unwrap();
                    }
                }
        }
}
