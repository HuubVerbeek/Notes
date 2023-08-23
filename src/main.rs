use clap::Parser;
use std::fs::{self, File};
use std::time::SystemTime;
use chrono::{DateTime,Utc};
use std::io::{self, Write};
use std::process::Command;
use std::str;

#[derive(Parser, Debug)]

#[clap(author="Huub Verbeek", version, about="A simple note taking tool.")]
struct Cli {
    #[clap(short, long, default_value_t = String::from(""))]
    filename: String,
    #[clap(short, long, default_value_t = String::from(""))]
    note: String
}

fn main() {
     let mut cli: Cli = Cli::parse();

     if cli.args_all_set(){
        store(&cli)
     }

     if cli.filename.is_empty(){
        read(String::from("Enter filename. Exit typing by pressing enter on a empty line"), &mut cli, "filename");
     }

     if cli.note.is_empty(){
        read(String::from("Enter note"), &mut cli, "note");
     }
    
     store(&cli)
}

fn store(cli: &Cli){
    let path = generate_path(&cli.filename); 
    let mut file: File = File::create(&path)
        .expect("Could not create file.");

    file.write_all(cli.note.as_bytes())
        .expect("Error while writing to file");

    println!("Stored note at {}", path)
}

fn read(msg: String, cli: &mut Cli, handle: &str) {
    println!("{}:", msg);

    let property: &mut String = property(cli, handle);

    let lines: io::Lines<io::StdinLock<'_>> = io::stdin().lines();

    for line in lines {

        let mut result: String = line.unwrap();

        if result.is_empty(){
            break;
        }

        result.push_str("\n");

        property.push_str(&result);
    }

    if property.is_empty(){
        read(String::from("Please enter a non empty value"), cli, handle);
    }
}

fn property<'a>(cli: &'a mut Cli, handle: &str) -> &'a mut String{
    match handle{
       "filename" => &mut cli.filename,
       _ =>  &mut cli.note,
    }
}

fn generate_path(filename: &String) -> String {
    let mut clone = filename.clone();

    clone.pop();

    let datetime: DateTime<Utc> = SystemTime::now().into();

    let extended = clone.to_owned() + "_" + &datetime.format("%d-%m-%Y_%H.%M.%S").to_string() + ".note";

    let output = Command::new("whoami").output().unwrap().stdout;

    let mut user = String::from_utf8(output).unwrap();

    user.pop();

    let _ = fs::create_dir_all("/Users/".to_owned() + &user + "/Notes/");

    String::from("/Users/".to_owned() + &user + "/Notes/") + &extended
}

impl Cli {
    fn args_all_set(&self) -> bool {
       ! (self.filename.is_empty() || self.note.is_empty())
    }
}
