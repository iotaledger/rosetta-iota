use structopt::StructOpt;
use url::Url;

#[derive(Clone, Debug, StructOpt)]
pub struct Options {
    #[structopt(long, parse(try_from_str = Url::parse))]
    pub iota_endpoint: String,

    #[structopt(long)]
    pub network: String,
}
