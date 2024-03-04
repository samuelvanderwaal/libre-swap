use anyhow::Result;
use clap::Parser;

use swap::{
    args::{Args, Commands},
    commands::*,
};

fn main() -> Result<()> {
    solana_logger::setup_with_default("solana=info");

    let args = Args::parse();

    let keypair_path = args.keypair_path.clone();
    let rpc_url = args.rpc_url.clone();

    match args.command {
        Commands::Decode {
            incoming_mint,
            outgoing_mint,
        } => handle_decode(DecodeArgs {
            keypair_path,
            rpc_url,
            incoming_mint,
            outgoing_mint,
        }),
        Commands::DecodeSwap { swap } => handle_decode_swap(DecodeSwapArgs {
            keypair_path,
            rpc_url,
            swap,
        }),
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
        Commands::Swap {
            nft_mint,
            fungible_mint,
        } => handle_swap(SwapArgs {
            keypair_path,
            rpc_url,
            nft_mint,
            fungible_mint,
        }),
    }
}
