use std::str::FromStr;

use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signer::Signer,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;

use super::*;

pub struct CreateSwapArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub nft_mint: String,
    pub fungible_mint: String,
    pub amount: u64,
}

pub fn handle_create_swap(args: CreateSwapArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let authority = config.keypair.pubkey();
    let nft_mint = Pubkey::from_str(&args.nft_mint)?;
    let fungible_mint = Pubkey::from_str(&args.fungible_mint)?;

    let accounts = derive_create_swap_accounts(authority, nft_mint, fungible_mint);
    let mut data = vec![];
    data.extend_from_slice(&CREATE_SWAP_DISC);
    data.extend(1u64.to_le_bytes().to_vec());
    data.extend(args.amount.to_le_bytes().to_vec());

    println!("data: {:?}", data);

    let instruction = Instruction::new_with_bytes(MONOSWAP_PROGRAM_ID, &data, accounts);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        config.recent_blockhash,
    );

    let sig = config.client.send_and_confirm_transaction(&transaction)?;

    println!("Signature: {:?}", sig);

    Ok(())
}

fn derive_create_swap_accounts(
    authority: Pubkey,
    nft_mint: Pubkey,
    fungible_mint: Pubkey,
) -> Vec<AccountMeta> {
    let swap_marker = Pubkey::find_program_address(
        &[
            b"swap_marker",
            authority.as_ref(),
            fungible_mint.as_ref(),
            nft_mint.as_ref(),
        ],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;
    let fungible_token_account_source = get_associated_token_address(&authority, &fungible_mint);

    let escrow_owner = Pubkey::find_program_address(
        &[b"swap_escrow", authority.as_ref(), nft_mint.as_ref()],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;
    let escrow_token_account = get_associated_token_address(&escrow_owner, &fungible_mint);

    vec![
        AccountMeta::new(swap_marker, false),
        // Mint Incoming
        AccountMeta::new(nft_mint, false),
        // Mint Outgoing
        AccountMeta::new_readonly(fungible_mint, false),
        // Mint Outgoing Token Account Source
        AccountMeta::new(fungible_token_account_source, false),
        // Escrow Holder
        AccountMeta::new_readonly(escrow_owner, false),
        // Mint Outgoing Token Account Escrow
        AccountMeta::new(escrow_token_account, false),
        // Payer
        AccountMeta::new(authority, true),
        // Mint Outgoing Owner
        AccountMeta::new(authority, true),
        // Namespace
        AccountMeta::new_readonly(authority, true),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
    ]
}
