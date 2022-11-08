use git2::{Repository, Commit};
use mapfile::Mapfile;
use mysql::{Pool, params, prelude::Queryable};
use mysql_common::chrono::NaiveDateTime;
use rayon::prelude::*;
use clap::Parser;

mod cli;
mod mapfile;

macro_rules! filter_try {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => return Some(Err(e)),
        }
    };
}

fn main() {
    let args = cli::Args::parse();

    args.repo_names.par_iter().for_each(|repo_name| {
        println!("Upsert commits for repo {}", repo_name);

        let base_dir = if !args.base_dir.ends_with("/") { 
            format!("{}/", args.base_dir)
        } else {
            args.base_dir.to_owned()
        };
    
        let repo = match Repository::open(format!("{}/{}/", base_dir, repo_name)) {
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

        let commits = revwalk.map(|c| c.unwrap()).collect();

        insert_commit(&args.mysql_connection, commits, repo_name, &args.author_name_mapfile);
    });
}

fn insert_commit(mysql_connection: &str, commits: Vec<Commit>, repo_name: &str, author_name_mapfile: &Option<String>) {
    let pool = Pool::new(mysql_connection.as_ref()).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let author_name_map = match author_name_mapfile {
        Some(filename) => Mapfile::parse(filename),
        None => Mapfile::empty()
    };

    conn.exec_batch(
        r"INSERT INTO commits (id, repository, author_name, author_email, summary, body, time)
          VALUES (:id, :repository, :author_name, :author_email, :summary, :body, :time)
          ON DUPLICATE KEY UPDATE author_name = :author_name, author_email = :author_email",
          commits.iter().map(|commit| {
            params! {
                "id" => commit.id().to_string(),
                "repository" => repo_name.to_owned(),
                "author_name" => author_name_map.map(commit.author().name().unwrap()),
                "author_email" => commit.author().email().unwrap(),
                "summary" => commit.summary().unwrap(),
                "body" => commit.body().unwrap_or_default(),
                "time" => NaiveDateTime::from_timestamp(commit.time().seconds(), 0)
            }
        })
    ).unwrap();
}