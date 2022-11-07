use git2::Repository;
use mysql::{Pool, params, prelude::Queryable};
use mysql_common::chrono::NaiveDateTime;

macro_rules! filter_try {
    ($e:expr) => {
        match $e {
            Ok(t) => t,
            Err(e) => return Some(Err(e)),
        }
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let repo_names = vec![""];

    for repo_name in repo_names {

        println!("Upsert commits for repo {}", repo_name);
        let repo = match Repository::open(format!("../{}/", repo_name)) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
    
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
    
        let revwalk = revwalk
            .filter_map(|id| {
                let id = filter_try!(id);
                let commit = filter_try!(repo.find_commit(id));
    
                Some(Ok(commit))
            });
    
        let url = "mysql://git:git123@localhost:3306/db";
        let pool = Pool::new(url)?;
    
        let mut conn = pool.get_conn()?;
    
        conn.exec_batch(
            r"INSERT IGNORE INTO commits (id, repository, author_name, author_email, summary, body, time)
              VALUES (:id, :repository, :author_name, :author_email, :summary, :body, :time)",
              revwalk.map(|c| {
                let commit = c.unwrap();
                params! {
                    "id" => commit.id().to_string(),
                    "repository" => repo_name,
                    "author_name" => commit.author().name().unwrap(),
                    "author_email" => commit.author().email().unwrap(),
                    "summary" => commit.summary().unwrap(),
                    "body" => commit.body().unwrap_or_default(),
                    "time" => NaiveDateTime::from_timestamp(commit.time().seconds(), 0)
                }
            })
        )?;    
    }

    Ok(())
}