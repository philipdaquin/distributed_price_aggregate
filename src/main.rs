use clap::{Parser, Subcommand};
/*
    simple --mode=cache --times=10
    simple --mode=read
*/
#[derive(Debug, Parser)]
#[command(
    bin_name = "simple",
    long_about = "Simple Client for Retrieving Average BTC Prices",
    subcommand_value_name = "mode"
)]
struct Args {
    #[command(subcommand, name = "mode")]
    mode: Action,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Action {
    #[command(long_about = "Read Cache File")]
    READ,
    #[command(long_about = "Retrieve the average price of BTC/USD with given number of seconds")]
    Cache {
        #[arg(long)]
        times: i32
    },
}

fn main() {
    let cli = Args::parse();
    match cli.mode { 
        Action::Cache { times } => { 
            println!("Monitoring BTC under {times} seconds");
        },
        Action::READ => { 
            println!("Reading BTC price")
        }
    }
}
