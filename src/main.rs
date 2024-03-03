use anyhow::Result;
use clap::Parser;

use swap::{
    args::{Args, Commands},
    commands::{handle_create_swap, handle_swap, CreateSwapArgs, SwapArgs},
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::CreateSwap {
            nft_mint,
            fungible_mint,
            amount,
        } => handle_create_swap(CreateSwapArgs {
            keypair_path,
            rpc_url,
            nft_mint,
            fungible_mint,
            amount,
        }),
        Commands::Swap { value } => handle_swap(SwapArgs {
            keypair_path,
            rpc_url,
            value,
        }),
    }
}
