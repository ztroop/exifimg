use clap::{App, Arg};
use exif::Reader;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::Cursor;
use walkdir::WalkDir;

fn main() {
    let matches = App::new("EXIF Utility")
        .arg(Arg::with_name("file_path").required(true))
        .arg(Arg::with_name("recursive").long("recursive").short('r'))
        .arg(Arg::with_name("strip").long("strip").short('s'))
        .get_matches();

    let file_path = matches.value_of("file_path").unwrap().to_string();
    let recursive = matches.is_present("recursive");
    let action = if matches.is_present("strip") {
        Action::Strip
    } else {
        Action::Read
    };

    let handler: Box<dyn ExifHandler> = if recursive {
        Box::new(DirectoryHandler {
            dir_path: file_path,
        })
    } else {
        Box::new(SingleFileHandler { file_path })
    };

    match action {
        Action::Read => handler.read_exif().unwrap(),
        Action::Strip => handler.strip_exif().unwrap(),
    }
}

trait ExifHandler {
    fn read_exif(&self) -> Result<(), String>;
    fn strip_exif(&self) -> Result<(), String>;
}

struct SingleFileHandler {
    file_path: String,
}

struct DirectoryHandler {
    dir_path: String,
}

impl ExifHandler for SingleFileHandler {
    fn read_exif(&self) -> Result<(), String> {
        read_exif_from_file(&self.file_path)
    }

    fn strip_exif(&self) -> Result<(), String> {
        strip_exif_from_file(&self.file_path)
    }
}

impl ExifHandler for DirectoryHandler {
    fn read_exif(&self) -> Result<(), String> {
        for entry in WalkDir::new(&self.dir_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                read_exif_from_file(entry.path().to_str().unwrap())?;
            }
        }
        Ok(())
    }

    fn strip_exif(&self) -> Result<(), String> {
        for entry in WalkDir::new(&self.dir_path) {
            let entry = entry.unwrap();
            if entry.file_type().is_file() {
                strip_exif_from_file(entry.path().to_str().unwrap())?;
            }
        }
        Ok(())
    }
}

fn read_exif_from_file(file_path: &str) -> Result<(), String> {
    let file = File::open(file_path).map_err(|e| e.to_string())?;

    if let Ok(reader) = Reader::new()
        .read_from_container(&mut BufReader::new(file))
        .map_err(|e| e.to_string())
    {
        for field in reader.fields() {
            println!(
                "{0: <10} | {1: <10}",
                field.tag,
                field.display_value().with_unit(&reader)
            );
        }
    }

    Ok(())
}

fn strip_exif_from_file(file_path: &str) -> Result<(), String> {
    let data = fs::read(file_path).map_err(|e| e.to_string())?;
    let image = image::load_from_memory_with_format(&data, image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    let mut clean_data = Cursor::new(Vec::new());
    image
        .write_to(&mut clean_data, image::ImageFormat::Jpeg)
        .map_err(|e| e.to_string())?;

    fs::write(file_path, clean_data.into_inner()).map_err(|e| e.to_string())?;
    Ok(())
}

enum Action {
    Read,
    Strip,
}
