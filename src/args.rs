use std::path::PathBuf;

use clap::{Parser, Subcommand};
use solana_sdk::pubkey::Pubkey;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Path to the keypair file.
    #[arg(short, long, global = true)]
    pub keypair_path: Option<PathBuf>,

    /// RPC URL for the Solana cluster.
    #[arg(short, long, global = true)]
    pub rpc_url: Option<String>,

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Clone, Subcommand)]
pub enum Commands {
    Decode {
        incoming_mint: Pubkey,
        outgoing_mint: Pubkey,
    },
    DecodeSwap {
        swap: Pubkey,
    },
    CreateSwap {
        nft_mint: Pubkey,
        fungible_mint: Pubkey,
        amount: u64,
    },
    In {
        nft_mint: Pubkey,
        fungible_mint: Pubkey,
    },
    Out {
        nft_mint: Pubkey,
        fungible_mint: Pubkey,
    },
}
