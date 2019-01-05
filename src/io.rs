use std::fs::File;
use std::io::prelude::*;

pub mod fmng {
    struct FileInfo {
        offset: u32,
        size: u32,
    }

    struct FileManager {
        files: Vec<FileInfo>,
    }

    impl FileManager {
        pub fn new(dbpath: String) -> FileManager {
            let mut f = std::fs::File::open(dbpath).expect("File Manager file is not found.");

            FileManager {
                files: [],
            }
        }

        pub fn import(self) {

        }

        pub fn read(self) {

        }
    }
}