
pub mod file_repository;


pub trait RepositoryInterface { 
    fn get_recent() -> String;
    fn save_to_local() -> String; 
}