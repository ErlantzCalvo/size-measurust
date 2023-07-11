use std::io::prelude::*;
use std::fs::File;
use std::fs::read_dir;
use std::fs::metadata;
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(author="Erlantz Calvo", version, about="A Very simple file size displayer")]
struct Args {
    /// Path of the files to be displayed
    path: PathBuf,

    /// Format of size. I.e.: MB, KB, GB, B
    #[arg(short, long, default_value_t=String::from("B"))]
    format: String
}

#[derive(Debug)]
struct FileData {
    path: String,
    size: usize
}


struct SizeMeasure {
    value: f32,
    acronym: String
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let size_measure = get_size_measure(args.format);

    let files_data = process_path(args.path, &size_measure)?;
    print_files_info(files_data, size_measure.acronym);
    return Ok(());
}

fn get_size_measure(format: String) -> SizeMeasure {
    match format.to_lowercase().as_str() {
        "k" | "K" | "kb" | "Kb" | "kB" | "KB" => return SizeMeasure{value: 1e3, acronym: "KB".to_string()},
        "m" | "M" | "mb" | "Mb" | "mB" | "MB" => return SizeMeasure{value: 1e6, acronym: "MB".to_string()},
        "g" | "G" | "gb" | "Gb" | "gB" | "GB" => return SizeMeasure{value: 1e9, acronym: "GB".to_string()},
        _ => return SizeMeasure{value: 1e0, acronym: "B".to_string()}

    }
}

fn process_path(path: PathBuf, size_measure: &SizeMeasure) -> Result<Vec<FileData>, std::io::Error> {
    let mut files_info = Vec::new();
    // let path_metadata = metadata(&path);
    match metadata(&path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                let paths = read_dir(path).unwrap();
                for p in paths {
                    if p.as_ref().unwrap().metadata().unwrap().is_file() {
                        files_info.push(get_file_info(p.unwrap().path(), size_measure));
                    }
                }
            } else {
                files_info.push(get_file_info(path, size_measure));
            }
            return Ok(files_info);
        },
        Err(error) => return Err(error)

    }

}

fn get_file_info(path: PathBuf, size_measure: &SizeMeasure) -> FileData {
    let s = get_file_size(&path, size_measure.value);
    return FileData{path: path.as_path().display().to_string(), size: s};
}

fn get_file_size(file_path: &PathBuf, size_measure: f32) -> usize{
    match File::open(file_path) {
        Ok(file) => return file.bytes().count() / size_measure as usize,
        Err(_) => return 0
    }
}

fn print_files_info(files_data: Vec<FileData>, format: String) {
    for fd in &files_data {
        println!("{0}   {1} {2}", fd.path.red(), fd.size, format);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn path_not_exist() {
        let size_measure = super::get_size_measure("".to_string());
    }
}
