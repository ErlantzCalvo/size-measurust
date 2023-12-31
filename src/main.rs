use std::fs::File;
use std::fs::read_dir;
use std::fs::metadata;
use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;
use std::io::Read;
use std::time::Instant;



#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(author="Erlantz Calvo", version, about="A Very simple file size displayer")]
struct Args {
    /// Path of the files to be displayed
    path: PathBuf,

    #[arg(short, long, default_value_t=false)]
    recursive: bool,

    /// Format of size. I.e.: MB, KB, GB, B
    #[arg(short, long, default_value_t=String::from("B"))]
    format: String
}

#[derive(Debug, PartialEq)]
struct FileData {
    path: String,
    size: f32
}


#[derive(Debug, PartialEq)]
struct SizeMeasure {
    value: f32,
    acronym: String
}

impl SizeMeasure {
    fn new() -> Self {
         SizeMeasure{value: 1e0, acronym: "B".to_string()}
    }

    fn from(format: String) -> Self {
        match format.to_lowercase().as_str() {
            "k" | "K" | "kb" | "Kb" | "kB" | "KB" => SizeMeasure{value: 1e3, acronym: "KB".to_string()},
            "m" | "M" | "mb" | "Mb" | "mB" | "MB" => SizeMeasure{value: 1e6, acronym: "MB".to_string()},
            "g" | "G" | "gb" | "Gb" | "gB" | "GB" => SizeMeasure{value: 1e9, acronym: "GB".to_string()},
            _ => SizeMeasure::new()
        }

    }
}

struct FinalInfo {
    num_files: u64,
    total_size: f32
}


static mut FINAL_INFO: FinalInfo = FinalInfo{num_files: 0, total_size: 0.0};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    let files_data = process_path(args.path, args.recursive)?;
    let elapsed = start_time.elapsed();

    let size_measure = SizeMeasure::from(args.format);
    print_files_info(files_data, &size_measure);
    print_total_info(size_measure, elapsed);
    Ok(())
}

fn process_path(path: PathBuf, recursive: bool) -> Result<Vec<FileData>, std::io::Error> {
    let mut files_info = Vec::new();
    match metadata(&path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                let paths = read_dir(path).unwrap();
                for p in paths {
                    if p.as_ref().unwrap().metadata().unwrap().is_file() {
                        
                        let file_info = get_file_info(p.unwrap().path());

                        unsafe {
                            print!("\rFiles read: {}", FINAL_INFO.num_files);
                            FINAL_INFO.total_size += file_info.size;
                            FINAL_INFO.num_files +=1;
                        }

                        files_info.push(file_info);
                    } else if recursive {
                        if let Ok(mut recursive_sizes) = process_path(p.unwrap().path(), recursive) {
                            files_info.append(&mut recursive_sizes);
                        }
                    }
                }
            } else {
                files_info.push(get_file_info(path));
            }
            Ok(files_info)
        },
        Err(error) => Err(error)

    }

}

fn get_file_info(path: PathBuf) -> FileData {
    let s = get_file_size(&path);
    FileData{path: path.as_path().display().to_string(), size: s as f32}
}

fn get_file_size(file_path: &PathBuf) -> usize{
    match File::open(file_path) {
        Ok(file) => file.bytes().count(),
        Err(_) => 0
    }
}

fn print_files_info(files_data: Vec<FileData>, format: &SizeMeasure) {
    for fd in &files_data {
        let size = fd.size / format.value;
        println!("{}   {:.2} {}", fd.path.yellow(), size, format.acronym);
    }
}

fn print_total_info(format: SizeMeasure, elapsed_time: std::time::Duration) {
    println!("-- Total --");
    unsafe {
    let text1 = format!("Files number: {0}", FINAL_INFO.num_files).blue();
    println!("{}", text1);
    let text2 = format!("Total size:   {:.2}{}", FINAL_INFO.total_size / format.value, format.acronym).blue();
    println!("{}", text2);
    let text3 = format!("Total time:   {0}s", elapsed_time.as_secs()).blue();
    println!("{}", text3);
    }
}

#[cfg(test)]
mod tests {
    use crate::SizeMeasure;

    #[test]
    fn format_not_specified() {
        let size_measure = SizeMeasure::from("".to_string());
        let result = super::SizeMeasure{value: 1e0, acronym: "B".to_string()};
        assert_eq!(size_measure, result);
    }

    #[test]
    fn format_specified() {
        {
            let size_measure = SizeMeasure::from("b".to_string());
            let result = SizeMeasure{value: 1e0, acronym: "B".to_string()};

            assert_eq!(size_measure, result);
        }{
            let size_measure = SizeMeasure::from("m".to_string());
            let result = SizeMeasure{value: 1e6, acronym: "MB".to_string()};

            assert_eq!(size_measure, result);
        }{
            let size_measure = SizeMeasure::from("g".to_string());
            let result = SizeMeasure{value: 1e9, acronym: "GB".to_string()};

            assert_eq!(size_measure, result);
        }
    }
    
    #[test]
    fn inexistent_path() {
        let result = super::process_path(super::PathBuf::from("/invented/madeup/alcachofa"), true).map_err(|e| e.kind());

        assert_eq!(result, Err(std::io::ErrorKind::NotFound));
    }

}
