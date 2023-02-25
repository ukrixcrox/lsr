use std::fs;
use std::path::Path;
use clap::Parser;
use colored::Colorize;

/// simple ls implementation in rust for the crust shell
#[derive(Parser)]
#[command(author, version)]
struct Opts{
    /// custom dir
    #[arg(default_value=".")]
    dir:String,

    /// shows everything thats in a dir
    #[arg(short='a', long)]
    show_all:bool,

    /// shows only dirs
    #[arg(short='d', long)]
    dir_only:bool,
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
                if !args.show_all && !args.dir_only{
                    if !entry_string.starts_with('.'){
                        format_output(&entry);
                    }

                }else if args.show_all && !args.dir_only{
                    format_output(&entry);

                }else if args.show_all && args.dir_only{
                    if entry.is_dir(){
                        format_output(&entry);
                    }

                }else if !args.show_all && args.dir_only && entry.is_dir() && !entry_string.starts_with('.'){
                        format_output(&entry);
                    }
                }
        }
}

fn format_output(entry: &Path){
    let entry_string = entry.file_name()
                            .unwrap()
                            .to_os_string()
                            .into_string()
                            .unwrap();

    if entry_string.starts_with('.') && !entry.is_dir(){
        println!("{}", entry_string.red());

    }else if entry.is_dir(){
        println!("{}", entry_string.bold().blue());

    }else{
        println!("{}", entry_string);
    }
}
