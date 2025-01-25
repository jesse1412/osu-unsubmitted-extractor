use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use secrecy::SecretString;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[arg(short, long)]
    pub songs_folder: PathBuf,

    #[arg(long)]
    /// Your client secret. Created here: https://osu.ppy.sh/home/account/edit#oauth
    pub secret: SecretString,

    #[arg(short, long)]
    /// Your client id. Created here: https://osu.ppy.sh/home/account/edit#oauth
    pub client_id: u64,

    #[arg(short, long, default_value_t=reqwest::Url::from_str("https://osu.ppy.sh/api/v2/beatmaps").unwrap())]
    pub lookup_beatmap_api_url: reqwest::Url,

    #[arg(short, long, default_value_t = 50)]
    pub rate_limit_per_minute: u64,

    #[arg(short, long)]
    pub output_zip_path: PathBuf,
}
