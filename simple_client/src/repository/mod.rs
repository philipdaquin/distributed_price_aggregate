use std::path::PathBuf;


pub mod file_repository;


pub trait RepositoryInterface { 
    fn open(path: PathBuf) -> Self;
    fn get_recent() -> String;
    fn save_to_local() -> String; 
}