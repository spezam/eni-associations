use clap::Parser;

mod client;
use crate::client::EniAssociationsClient;

#[derive(Parser, Debug)]
#[command(about = "ENI associations", author, version, after_help = "eof")]
struct CliArgs {
    #[clap(short, long, help = "ENI")]
    eni: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let ec2 = EniAssociationsClient::new(&args.eni).await;
    ec2.list_eni_associations().await.unwrap();
}
