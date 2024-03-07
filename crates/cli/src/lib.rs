use beerus_core::config::Config;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    #[clap(short = 'c', long)]
    config: Option<String>,
}

pub fn get_config(args: Args) -> eyre::Result<Config> {
    Ok(if let Some(path) = args.config.as_ref() {
        Config::from_file(path)?
    } else {
        Config::from_env()
    })
}
