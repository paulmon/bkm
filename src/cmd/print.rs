use clap::{App, Arg, ArgMatches, SubCommand};
use std::process;

use database::DB;

pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("print")
        .about("Print bookmark")
        .arg(Arg::from_usage("<ID>... 'Print bookmarks matching the specified ids{n}\
                             (If id is not specified, print all bookmarks)'").required(false))
        .arg_from_usage("-T 'Print all tags'")
}

pub fn execute(args: &ArgMatches) {
    let db = DB::open();

    if args.is_present("T") {
        if db.get_record_count("tags") == 0 {
            println!("Error: Tag does not exist");
            process::exit(1);
        }

        let tags = db.get_all_tag();
        for tag in tags {
            println!("{}", tag);
        }

        process::exit(0);
    }
    if ! args.is_present("ID") {

        if db.get_record_count("bookmarks") == 0 {
            println!("Error: Bookmark does not exist");
            process::exit(1);
        }

        let bookmarks = db.get_all_bookmark();
        for bookmark in bookmarks {
            bookmark.print();
        }

        process::exit(0);
    }

    let ids = values_t!(args, "ID", i64).unwrap();
    for id in ids {
        match db.get_bookmark_by_id(id) {
            Ok(b) => {
                b.print();
            },
            Err(e) => println!("{} {}", e, id),
        }
    }
}
