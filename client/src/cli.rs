use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("127.0.0.1:11000"))]
    pub geyser_quic_address: String,

    #[arg(short, long, default_value_t = String::from("connection_identity.json"))]
    pub identity: String,
}
