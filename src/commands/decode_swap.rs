use super::*;

pub struct DecodeSwapArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub incoming_mint: Pubkey,
    pub outgoing_mint: Pubkey,
}

pub fn handle_decode_swap(args: DecodeSwapArgs) -> Result<()> {
    let config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    let swap_marker = Pubkey::find_program_address(
        &[
            b"swap_marker",
            config.keypair.pubkey().as_ref(),
            args.outgoing_mint.as_ref(),
            args.incoming_mint.as_ref(),
        ],
        &MONOSWAP_PROGRAM_ID,
    )
    .0;

    let data = config.client.get_account_data(&swap_marker)?;
    let swap_marker = SwapMarker::deserialize(&mut &data[8..])?;

    println!("Swap Marker: {:#?}", swap_marker);
    Ok(())
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct SwapMarker {
    // namespace can be anything that the caller can sign
    pub namespace: Pubkey,
    // allows slicing and dicing by incoming mint
    pub mint_incoming: Pubkey,
    // allows slicing and dicing by outgoing mint
    pub mint_outgoing: Pubkey,
    pub mint_incoming_amount: u64,
    pub mint_outgoing_amount: u64,
    // an unused marker can be closed.
    // after it has been used, it can not be
    // closed to avoid a situation where a
    // holder gets trapped into a crappy token
    // and cannot go back
    pub used: bool,
}
