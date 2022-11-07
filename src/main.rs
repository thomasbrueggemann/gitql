use git2::{Repository, Commit};
use mysql::{Pool, params, prelude::Queryable};
use mysql_common::chrono::NaiveDateTime;
use rayon::prelude::*;
use clap::Parser;

mod cli;

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

        let commits = revwalk.map(|c| c.unwrap()).collect();

        insert_commit(&args.mysql_connection, commits, repo_name);
    });
}

fn insert_commit(mysql_connection: &str, commits: Vec<Commit>, repo_name: &str) {
    let pool = Pool::new(mysql_connection.as_ref()).unwrap();

    let mut conn = pool.get_conn().unwrap();

    conn.exec_batch(
        r"INSERT IGNORE INTO commits (id, repository, author_name, author_email, summary, body, time)
          VALUES (:id, :repository, :author_name, :author_email, :summary, :body, :time)",
          commits.iter().map(|commit| {
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
}