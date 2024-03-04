use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::commitment_config::CommitmentConfig;

use super::*;

pub struct SwapArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub nft_mint: Pubkey,
    pub fungible_mint: Pubkey,
}

pub fn handle_swap(args: SwapArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let authority = config.keypair.pubkey();

    let accounts = derive_swap_accounts(authority, args.nft_mint, args.fungible_mint);
    let mut data = vec![];
    data.extend_from_slice(&SWAP_DISC);

    let instruction = Instruction::new_with_bytes(MONOSWAP_PROGRAM_ID, &data, accounts);
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&config.keypair.pubkey()),
        &[&config.keypair],
        config.recent_blockhash,
    );

    let rpc_config = RpcSendTransactionConfig {
        skip_preflight: true,
        ..Default::default()
    };

    let sig = config
        .client
        .send_and_confirm_transaction_with_spinner_and_config(
            &transaction,
            CommitmentConfig::confirmed(),
            rpc_config,
        )?;

    println!("Signature: {:?}", sig);
    Ok(())
}

fn derive_swap_accounts(
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

    let swap_marker_reverse = Pubkey::find_program_address(
        &[
            b"swap_marker",
            authority.as_ref(),
            nft_mint.as_ref(),
            fungible_mint.as_ref(),
        ],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;

    let nft_token_account_source = get_associated_token_address(&authority, &nft_mint);

    let escrow_owner = Pubkey::find_program_address(
        &[b"swap_escrow", authority.as_ref(), nft_mint.as_ref()],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;
    let escrow_owner_reverse = Pubkey::find_program_address(
        &[b"swap_escrow", authority.as_ref(), fungible_mint.as_ref()],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;
    let escrow_nft_token_account = get_associated_token_address(&escrow_owner_reverse, &nft_mint);
    let fungible_token_account_source = get_associated_token_address(&escrow_owner, &fungible_mint);
    let fungible_token_account_destination =
        get_associated_token_address(&authority, &fungible_mint);

    vec![
        AccountMeta::new(swap_marker, false),
        AccountMeta::new(swap_marker_reverse, false),
        AccountMeta::new_readonly(nft_mint, false),
        AccountMeta::new_readonly(fungible_mint, false),
        AccountMeta::new(nft_token_account_source, false),
        AccountMeta::new_readonly(escrow_owner, false),
        AccountMeta::new_readonly(escrow_owner_reverse, false),
        AccountMeta::new(escrow_nft_token_account, false),
        AccountMeta::new(fungible_token_account_source, false),
        AccountMeta::new(fungible_token_account_destination, false),
        AccountMeta::new_readonly(authority, true),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ]
}
