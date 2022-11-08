use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   #[arg(short, long)]
   pub base_dir: String,

   #[arg(short, long)]
   pub mysql_connection: String,

   #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
   pub repo_names: Vec<String>,

   #[arg(short, long)]
   pub author_name_mapfile: Option<String>,
}