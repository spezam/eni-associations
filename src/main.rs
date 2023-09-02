// playlists:
//  disco bambino: 5j58b4mI6G9kdkc9wchTVr
//  italian funk: 31wu5A5Dh83TEmGQVmOoAx
// albums:
//  3oBABPRcbDoS1zj4wu1NAM
use clap::Parser;

mod ec2;
use crate::ec2::Ec2;

#[derive(Parser, Debug)]
#[command(about = "ENI associations", author, version, after_help = "eof")]
struct CliArgs {
    #[clap(short, long, help = "ENI")]
    eni: String,
    // #[clap(short, long, help = "Limit results", default_value = "100")]
    // limit: u8,
}

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let ec2 = Ec2::new(args.eni).await;
    ec2.list_eni_associations().await.unwrap();
}
