use std::io::prelude::*;
use std::fs::File;
use std::env;

#[derive(Debug)]
struct FileData {
    path: String,
    size: usize
}


struct SizeMeasure;

impl SizeMeasure {
    pub const B: f32  = 10e0;
    pub const KB: f32 = 10e3;
    pub const MB: f32 = 10e6;
    pub const GB: f32 = 10e9;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()){
        print_help();
        return Ok(());
    }

    let file_paths = get_file_paths(args[1..].to_vec());
    let size_measure = get_size_measure(args);
    let files_data = get_files_info(file_paths, size_measure);
    println!("{:?}", files_data);
    return Ok(());
}

fn print_help() {
    println!("Usage mode: size-measurust [OPTIONS][FILE_PATH]\n");
    println!("Options:");
    println!("-h, --help: Print this text showing the usage instructions");
    println!("Example:");
    println!("\tsize-measurust file.txt");
}

fn get_file_paths(args: Vec<String>) -> Vec<String> {
    let mut paths = Vec::new();
    for arg in args {
        if arg.chars().nth(0).unwrap() != '-' {
            paths.push(arg);
        }
    }
    return paths;
}

fn get_size_measure(args: Vec<String>) -> f32 {
    for i in 0..args.len() {
        let first_char = args[i].chars().nth(0).unwrap();
        if first_char == '-' && args[i] == "-s".to_string(){
            if i < args.len() -1 {
                match args[i+1].to_lowercase().as_str() {
                    "k" | "K" | "kb" | "Kb" | "kB" | "KB" => return SizeMeasure::KB,
                    "m" | "M" | "mb" | "Mb" | "mB" | "MB" => return SizeMeasure::MB,
                    "g" | "G" | "gb" | "Gb" | "gB" | "GB" => return SizeMeasure::GB,
                    _ => return SizeMeasure::B
                }
            }
            
        }
    }
    return SizeMeasure::B;
}

fn get_files_info(files: Vec<String>, size_measure: f32) -> Vec<FileData> {
    let mut files_info = Vec::new();
    for file in files {
        let s = get_file_size(&file, size_measure);
        let file_info = FileData{path: file.clone(), size: s};
        files_info.push(file_info);
    }
    println!("{:?}", files_info);
    return files_info;
}

fn get_file_size(file_path: &String, size_measure: f32) -> usize{
    match File::open(file_path) {
        Ok(file) => return file.bytes().count() / size_measure as usize,
        Err(_) => return 0
    }
}
