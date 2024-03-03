use super::*;

pub struct SwapArgs {
    pub keypair_path: Option<PathBuf>,
    pub rpc_url: Option<String>,
    pub value: String,
}

pub fn handle_swap(args: SwapArgs) -> Result<()> {
    let _config = CliConfig::new(args.keypair_path, args.rpc_url)?;

    Ok(())
}

