use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

use super::RepositoryInterface;

const DEFAULT_LOCATION: &str = "";

#[derive(Debug)]
pub struct FileRepository { 
    path: PathBuf, 
    reader: BufReader<File>,
    writer: BufWriter<File>
}


impl RepositoryInterface for FileRepository {



    fn get_recent() -> String {
        todo!()
    }

    fn save_to_local() -> String {
        todo!()
    }
}