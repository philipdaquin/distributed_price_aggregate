use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::sync::Arc;

use super::RepositoryInterface;
use crate::error::{Result, ServerError};
use crate::models::agg_ticker_prices::AggTickerPrices;


const DEFAULT_CACHE_LOCATION: &str = "";
const DEFAULT_FILE_NAME: &str = "cache";

#[derive(Debug)]
pub struct FileRepository { 
    path: Arc<PathBuf>, 
    reader: BufReader<File>,
    writer: BufWriter<File>
}


impl FileRepository {
    ///
    /// Create a new directory that stores all cached prices 
    // pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
    //     let path = Arc::new(path.into());
    //     fs::create_dir_all(&*path)?;
    //     // return Self { 
    //     //     path
    //     // }
    //     todo!()
    // }

    pub fn load_from_file() -> Result<AggTickerPrices> {
        let file = File::open(DEFAULT_FILE_NAME)
            .map(|f| f)
            .map_err(|e | ServerError::MissingCacheFile(e))?;
        let reader = BufReader::new(file);
        let agg_data: AggTickerPrices = serde_json::from_reader(reader)?;

        Ok(agg_data)
    }

    pub fn save(agg_price_data: &AggTickerPrices) -> Result<()> {
        
        let file = File::create(DEFAULT_FILE_NAME)?;
        serde_json::to_writer(file, agg_price_data)?;
        Ok(())
    }
}