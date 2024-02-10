use clap::{Parser, Subcommand, Args};
use std::{collections::VecDeque, error::Error, fs, io, path::{PathBuf}, process};
#[derive(Parser, Debug)]
#[command(version, about = "A basic cli-tools" , name = "cli-tools", long_about = None)]
#[command(propagate_version = true)]
struct Arg{
    #[arg(long, default_value_t = String::from("."))]
    ls:String,
    #[arg(long, short, default_value_t= String::from("test.txt"))]
    ///File name to be searched
    filename:String,

    #[clap(long, short, default_value_t= String::from("test.txt"))]
    ///The start directory
    directory:String,
    #[arg(short, long, action = clap::ArgAction::Count)]
    /// test count for count option of arguments
    count:u8,
    #[command(subcommand)]
    command:Option<Commands>
}
#[derive(Subcommand, Debug)]
enum Commands{
    Add(AddArgs),   
}

#[derive(Args, Debug)]
struct AddArgs{
    name:Option<String>,
}

fn list_direc(args: &Arg)-> Result<(),  Box<dyn Error>>{
    if args.ls.len() != 0 {
        let directories_iter = fs::read_dir(args.ls.clone())?;
        directories_iter.for_each(|d| {
            match d {
                Ok(entry) => {
                    let path_buf = entry.path();
                    let os_string = path_buf.into_os_string();
                    let name = os_string.into_string().unwrap();
                    println!("{name}");
                },
                Err(e) => {println!("Error: {}", e)}
            }
        });
    }
    Ok(())
}

//arguments will be the file text we filename we want to get and the directory of the folder
fn find(args: &Arg) -> Result<(), Box<dyn Error>>{
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
    let args = Arg::parse();     
    if let Err(e) = list_direc(&args){
        println!("Error: {}", e);
        process::exit(1);
    }
    // match &args.command{
    //     Commands::Add(args) => {
    //         if let Some(name) = args.name.as_ref(){
    //             println!("Name: {}", name);
    //         }
           
    //     }
    // }
    // if let Err(e) = find(&args){
    //     println!("Error: {}", e);
    //     process::exit(1);
    // }
}
