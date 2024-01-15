
#[derive(Debug)]
pub enum ServerState { 
    ConnectionSuccess,
    ConnectionFailure,
    WaitingConnection,
    WritingToDatabase
}