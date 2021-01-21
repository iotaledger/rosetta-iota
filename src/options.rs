use structopt::StructOpt;
use url::Url;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(long)]
    pub iota_endpoint: String,

    #[structopt(long)]
    pub network: String,
}
