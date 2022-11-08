# 👔 GitQL

GitQL - High performance git commit history to MySQL importer

```bash
Usage: gitql [OPTIONS] --base-dir <BASE_DIR> --mysql-connection <MYSQL_CONNECTION>

Options:
  -b, --base-dir <BASE_DIR>                        
  -m, --mysql-connection <MYSQL_CONNECTION>        
  -r, --repo-names <REPO_NAMES>                    
  -a, --author-name-mapfile <AUTHOR_NAME_MAPFILE>  
  -h, --help                                       Print help information
  -V, --version                                    Print version information
```

## Repo Names

This is a comma seperated list of repository names in the base directory to process.
E.g.

```
test_repo,my_other_repo,andsoon
```

## Author Name Mapfile

You can specify a mapping file that maps the author name to a different name.
This is useful if commits have been made with slightly differing names of the team mates.
The format of the file goes as follows:

```
Original name -> New Name
Some other name -> Mapped to this name
Thomas Brueggemann -> Thomas Brüggemann
Max mustermann -> Max Mustermann
```

You get the idea.