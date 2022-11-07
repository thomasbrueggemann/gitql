use git2::Repository;
use mysql::{Pool, params, prelude::Queryable};
use mysql_common::chrono::NaiveDateTime;
use rayon::prelude::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   mysql_connection: String,

   #[arg(short, long, use_value_delimiter = true, value_delimiter = ',')]
   repo_names: Vec<String>,
}

macro_rules! filter_try {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => return Some(Err(e)),
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    args.repo_names.par_iter().for_each(|repo_name| {

        println!("Upsert commits for repo {}", repo_name);
        let repo = match Repository::open(format!("../{}/", repo_name)) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
    
        let mut revwalk = repo.revwalk().unwrap();
        revwalk.push_head().unwrap();
    
        let revwalk = revwalk
            .filter_map(|id| {
                let id = filter_try!(id);
                let commit = filter_try!(repo.find_commit(id));
    
                Some(Ok(commit))
            });
    
        let url = &args.mysql_connection;
        let pool = Pool::new(url.as_ref()).unwrap();
    
        let mut conn = pool.get_conn().unwrap();
    
        conn.exec_batch(
            r"INSERT IGNORE INTO commits (id, repository, author_name, author_email, summary, body, time)
              VALUES (:id, :repository, :author_name, :author_email, :summary, :body, :time)",
              revwalk.map(|c| {
                let commit = c.unwrap();
                params! {
                    "id" => commit.id().to_string(),
                    "repository" => repo_name.to_owned(),
                    "author_name" => commit.author().name().unwrap(),
                    "author_email" => commit.author().email().unwrap(),
                    "summary" => commit.summary().unwrap(),
                    "body" => commit.body().unwrap_or_default(),
                    "time" => NaiveDateTime::from_timestamp(commit.time().seconds(), 0)
                }
            })
        ).unwrap();
    });

    Ok(())
}