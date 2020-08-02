

use multitags_lib::database::*;
use multitags_lib::tags::*;
use multitags_lib::query::*;
use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "Richard S.")]
struct Opts {
    #[clap(short = 'f', long = "file", default_value = "./.multitags")]
    db: String,
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Clap, Debug)]
enum Subcommand {
    #[clap(name = "add", version = "0.1.0")]
    Add(Add),
    #[clap(name = "search", version = "0.1.0")]
    Search(Search)
}

#[derive(Clap, Debug)]
struct Add {
    file : String,
    tags : Vec<String>,
}

#[derive(Clap, Debug)]
struct Search {
    tag : String,
}



fn main() {
    let opts: Opts = Opts::parse();
    let mut db = DB::read_db(opts.db.clone())
        .unwrap_or(DB::init(opts.db.clone()));

    match opts.subcmd {
        Subcommand::Add(add_c) => {
            let file = add_c.file;
            db.add_file(file.clone());
            for mut tag in add_c.tags {
                if let Some(tag) = Tag::from_str(&mut tag) {
                    db.add_tag_to_file(file.clone(), &tag);
                }
            }
            match db.dump_db() {
                Err(_) => println!("Error Writing DB"),
                Ok(_) => println!("DB stored at {}", opts.db),
            }
        },
        Subcommand::Search(mut search_c) => {
            if let Some(tag) = Tag::from_str(&mut search_c.tag) {
                let query = Query::contains_c(tag);
                let results = db.match_query(&query);
                for result in results{
                    println!("{}", result);
                }
            }
        },
    }





    
}
