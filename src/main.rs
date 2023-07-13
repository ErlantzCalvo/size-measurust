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
    size: usize
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
    total_size: usize
}


static mut FINAL_INFO: FinalInfo = FinalInfo{num_files: 0, total_size: 0};

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();
    let size_measure = SizeMeasure::from(args.format);

    let files_data = process_path(args.path, &size_measure, args.recursive)?;
    let elapsed = start_time.elapsed();
    print_files_info(files_data, size_measure.acronym.clone());
    print_total_info(size_measure.acronym, elapsed);
    Ok(())
}

fn process_path(path: PathBuf, size_measure: &SizeMeasure, recursive: bool) -> Result<Vec<FileData>, std::io::Error> {
    let mut files_info = Vec::new();
    match metadata(&path) {
        Ok(path_metadata) => {
            if path_metadata.is_dir() {
                let paths = read_dir(path).unwrap();
                for p in paths {
                    if p.as_ref().unwrap().metadata().unwrap().is_file() {
                        
                        print!("\r\t{file}",  file = p.as_ref().unwrap().path().display());
                        let file_info = get_file_info(p.unwrap().path(), size_measure);

                        unsafe {
                            print!(" {}", FINAL_INFO.num_files);
                            FINAL_INFO.total_size += file_info.size;
                            FINAL_INFO.num_files +=1;
                        }

                        files_info.push(file_info);
                    } else if recursive {
                        print!("\r{}", p.as_ref().unwrap().path().display());

                        if let Ok(mut recursive_sizes) = process_path(p.unwrap().path(), size_measure, recursive) {
                            files_info.append(&mut recursive_sizes);
                        }
                    }
                }
            } else {
                files_info.push(get_file_info(path, size_measure));
            }
            Ok(files_info)
        },
        Err(error) => Err(error)

    }

}

fn get_file_info(path: PathBuf, size_measure: &SizeMeasure) -> FileData {
    let s = get_file_size(&path, size_measure.value);
    FileData{path: path.as_path().display().to_string(), size: s}
}

fn get_file_size(file_path: &PathBuf, size_measure: f32) -> usize{
    match File::open(file_path) {
        Ok(file) => file.bytes().count() / size_measure as usize,
        Err(_) => 0
    }
}

fn print_files_info(files_data: Vec<FileData>, format: String) {
    for fd in &files_data {
        println!("{0}   {1} {2}", fd.path.yellow(), fd.size, format);
    }
}

fn print_total_info(format: String, elapsed_time: std::time::Duration) {
    println!("-- Total --");
    unsafe {
    let text1 = format!("Files number: {0}", FINAL_INFO.num_files).blue();
    println!("{}", text1);
    let text2 = format!("Total size:   {0}{1}", FINAL_INFO.total_size, format).blue();
    println!("{}", text2);
    let text3 = format!("Total time:   {0}s", elapsed_time.as_secs()).blue();
    println!("{}", text3);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn format_not_specified() {
        let size_measure = super::get_size_measure("".to_string());
        let result = super::SizeMeasure{value: 1e0, acronym: "B".to_string()};
        assert_eq!(size_measure, result);
    }

    #[test]
    fn format_specified() {
        {
            let size_measure = super::get_size_measure("b".to_string());
            let result = super::SizeMeasure{value: 1e0, acronym: "B".to_string()};

            assert_eq!(size_measure, result);
        }{
            let size_measure = super::get_size_measure("m".to_string());
            let result = super::SizeMeasure{value: 1e6, acronym: "MB".to_string()};

            assert_eq!(size_measure, result);
        }{
            let size_measure = super::get_size_measure("g".to_string());
            let result = super::SizeMeasure{value: 1e9, acronym: "GB".to_string()};

            assert_eq!(size_measure, result);
        }
    }
    
    #[test]
    fn inexistent_path() {
        let size_measure = super::get_size_measure("b".to_string());
        let result = super::process_path(super::PathBuf::from("/invented/madeup/alcachofa"), &size_measure, true).map_err(|e| e.kind());

        assert_eq!(result, Err(std::io::ErrorKind::NotFound));
    }

}
