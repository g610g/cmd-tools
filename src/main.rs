use clap::Parser;
use std::{collections::VecDeque, error::Error, fs, io, path::{Path, PathBuf}, process};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(long, default_value_t = String::from("."))]
    ls:String,
    #[clap(long, short)]
    filename:String,
    #[clap(long, short)]
    directory:String
}

fn list_direc(args: &Args)-> Result<(),  io::Error>{
    if args.ls.len() != 0 {
        let directories_iter = fs::read_dir(args.ls.clone())?;
        directories_iter.for_each(|d| {
            match d {
                Ok(entry) => {println!("{:?}", entry.path())},
                Err(e) => println!("Error: {}", e)
            }
        });
    }
    
    Ok(())
}

//arguments will be the file text we filename we want to get and the directory of the folder
fn find(args: &Args) -> Result<(), Box<dyn Error>>{
    let mut queue = VecDeque::new(); 
    
    queue.push_back(PathBuf::from(args.directory.clone()));
    loop {
        if queue.is_empty(){
            break;
        }
        if let Some(path) = queue.pop_front(){
            let directory_iter = fs::read_dir(path)?;
            for d in directory_iter{
                let d = d?;
                let file_type = d.file_type()?;
                if file_type.is_file(){
                    if let Ok(str) = d.file_name().into_string(){
                        if args.filename == str{
                            println!("{:?}", d.path());
                       } 
                    }   
                }else{
                    queue.push_back(d.path());
                }
                
            }
        }
    }
    Ok(())
}
fn main() {
    let args = Args::parse();     
    // if let Err(e) = list_direc(&args){
    //     println!("Error: {}", e);
    //     process::exit(1);
    // }
    if let Err(e) = find(&args){
        println!("Error: {}", e);
        process::exit(1);
    }
}
