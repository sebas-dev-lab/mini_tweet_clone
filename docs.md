### Diesel steps

- Import diesel and features required into cargo.toml and instal diesel_cli
- To install diesel_cli = cargo install diesel_cli --no-default-features --features postgres
- Set DATABASE_URL
- Exec = diesel setup // create migrations files
- Exec = diesel migration generate <>table // build table
- Config migrations files
- Exec = diesel setup 
- Exec = diesel migration run


> r2d2 => generci pull conections to connect to database