use clap::Parser;
use std::{fs,io, process};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(long)]
    ls:String
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
fn main() {
    let args = Args::parse();
    if let Err(e) = list_direc(&args){
        println!("Error: {}", e);
        process::exit(1);
    }
    
}
