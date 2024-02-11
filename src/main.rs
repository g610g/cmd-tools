use clap::{Parser, Subcommand, Args};
use std::{collections::VecDeque, error::Error, fs, io, path::{PathBuf}, process};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
#[derive(Parser, Debug)]
#[command(version, about = "A basic cli-tools" , name = "cli-tools", long_about = None)]
#[command(propagate_version = true)]
struct Arg{
    #[arg(long, default_value_t = String::from("."))]
    ls:String,
    #[arg(long, short, default_value_t= String::from("test.txt"))]
    ///File name to be searched
    filename:String,

    #[arg(long, short, default_value_t = false)]
    clip:bool,
    #[clap(long, short, default_value_t= String::from("test.txt"))]
    ///The start directory
    directory:String,
    // #[arg(short, long, action = clap::ArgAction::Count)]
    /// test count for count option of arguments
    // count:u8,
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
//copies the contents of the file a rust tool
fn clip(args: &Arg) -> Result<(String), Box<dyn Error>>{
    //initialize clipboard
    let mut ctx = ClipboardContext::new()?;
    let contents = fs::read_to_string(args.filename.clone())?;
    ctx.set_contents(contents)?;
    let string = ctx.get_contents()?;
    Ok(string)
}
//concatenates two files must check if given filename is a file and also concatenates the content of the file and put into existing one
fn cat() -> Result<(), Box<dyn Error>>{
    Ok(())
}
fn main() {
    let args = Arg::parse();     
    if let Err(e) = clip(&args){
        println!("Error: {}", e);
        process::exit(1);
    }else{
        println!("Contents copied into clipboard!");
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
