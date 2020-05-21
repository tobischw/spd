use std::fs::File;
use std::io::{Read, Error};


pub struct FileIO {
    pub file: String,
    data: Vec<u8>
}

impl FileIO {

    pub fn new(filename: String) -> FileIO {
        FileIO {
            file: filename,
            data: Vec::new()
        }
    }

    pub fn read_file(mut self) -> Result<Vec<u8>, std::io::Error>{
        let mut f = File::open(self.file)?;
        let mut data: Vec<u8> = Vec::new();
        f.read_to_end(&mut data)?;

        self.data = data.clone();
        return Ok(data)
    }
}