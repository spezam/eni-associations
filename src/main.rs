use clap::Parser;

mod ec2;
use crate::ec2::Ec2;

#[derive(Parser, Debug)]
#[command(about = "ENI associations", author, version, after_help = "eof")]
struct CliArgs {
    #[clap(short, long, help = "ENI")]
    eni: String,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let ec2 = Ec2::new(args.eni).await;
    ec2.list_eni_associations().await.unwrap();
}
