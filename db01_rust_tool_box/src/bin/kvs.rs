use clap::{Args, Parser, Subcommand};

use kvs::KvStore;

#[derive(Parser, Debug)]
#[command(name = "kvs")]
#[command(version = "0.1.0")]
struct KvsArgs {
    // The action on the data
    #[command(subcommand)]
    sub_command: SubCommands,
}

#[derive(Subcommand, Debug)]
enum SubCommands {
    SET(SetArgs),
    GET(GetArgs),
    RM(RemoveArgs),
}

#[derive(Args, Debug)]
struct SetArgs {
    key: String,
    value: String,
}

#[derive(Args, Debug)]
struct GetArgs {
    key: String,
}

#[derive(Args, Debug)]
struct RemoveArgs {
    key: String,
}

fn main() {
    // Parse the input arguments
    let args = KvsArgs::parse();
    let mut kvs_db = kvs::KvStore::new();

    match args.sub_command {
        SubCommands::SET(args) => {
            kvs_db.set(args.key, args.value);
        }
        SubCommands::GET(args) => {
            kvs_db.get(args.key);
        }
        SubCommands::RM(args) => {
            kvs_db.remove(args.key);
        }
    }
}
