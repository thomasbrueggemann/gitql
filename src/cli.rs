use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   #[arg(short, long)]
   pub mysql_connection: String,

   #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
   pub repo_names: Vec<String>,
}