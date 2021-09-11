use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, fs, path::Path};

pub struct FileReader;

impl FileReader {
    pub fn process(path: &Path) -> Result<FileContainer, Box<dyn Error>> {
        let empty_file_list = Ok(Vec::new());
        let files = FileReader::get_file_contents(path, empty_file_list)?;
        let output_filename = FileReader::get_output_filename(path)?;

        Ok(FileContainer {
            files,
            output_filename,
        })
    }

    fn strip_extension(filename: &str) -> String {
        lazy_static! {
            static ref EXT_RE: Regex = Regex::new("\\..+$").unwrap();
        }

        EXT_RE.replace(filename, "").to_string()
    }

    fn get_file_contents(path: &Path, file_list: FileListResult) -> FileListResult {
        let mut file_list = file_list?;

        if path.is_dir() {
            for entry_result in path.read_dir()? {
                let entry = entry_result?;
                file_list = FileReader::get_file_contents(&entry.path(), Ok(file_list))?;
            }
        }

        if FileReader::should_process(&path) {
            let filename = FileReader::strip_extension(path.to_str().unwrap());
            let contents = fs::read_to_string(path)?;
            file_list.push(FileData { filename, contents });
        }

        Ok(file_list)
    }

    fn should_process(path: &Path) -> bool {
        path.is_file() && path.extension().unwrap() == "vm"
    }

    fn get_output_filename(path: &Path) -> Result<String, Box<dyn Error>> {
        let filename = if path.is_dir() {
            path.to_string_lossy().into_owned()
        } else {
            FileReader::strip_extension(path.to_str().unwrap())
        };

        Ok(format!("{}.asm", filename))
    }
}

pub struct FileData {
    pub filename: String,
    pub contents: String,
}

type FileList = Vec<FileData>;

pub struct FileContainer {
    pub files: FileList,
    pub output_filename: String,
}

type FileListResult = Result<FileList, Box<dyn Error>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_file() {
        match FileReader::process(Path::new("test_input/foo.vm")) {
            Ok(file_data) => {
                let files = vec![(
                    String::from("test_input/foo"),
                    String::from("push argument 3\n"),
                )];

                assert_eq!(files, file_data.files);
                assert_eq!("test_input/foo.asm", file_data.output_filename);
            }
            Err(err) => {
                panic!("Error processing input path: {}", err)
            }
        };
    }

    #[test]
    fn process_directory() {
        match FileReader::process(Path::new("test_input")) {
            Ok(file_data) => {
                let files = vec![
                    (
                        String::from("test_input/bar"),
                        String::from("pop static 2\n"),
                    ),
                    (
                        String::from("test_input/foo"),
                        String::from("push argument 3\n"),
                    ),
                    (
                        String::from("test_input/baz/qux"),
                        String::from("push static 4\n"),
                    ),
                ];

                assert_eq!(files, file_data.files);
                assert_eq!("test_input.asm", file_data.output_filename);
            }
            Err(err) => {
                panic!("Error processing input path: {}", err)
            }
        };
    }
}
