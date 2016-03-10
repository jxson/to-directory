extern crate clap;

use types::ToResult;
use std::path::PathBuf;
use std::fmt;
use std::env;
use std::fs;

#[derive(Debug)]
pub enum Action {
    Get,
    Put,
    Delete,
    List,
    Last,
    ChangeDirectory,
}

pub struct CLI<'a, 'b, 'd> where 'a: 'b, 'd {
    pub actions: [clap::Arg<'a, 'b>; 4],
    pub matches: clap::ArgMatches<'d>,
}

impl<'a, 'b, 'c> CLI<'a, 'b, 'c> {
    pub fn new<'d>(actions: [clap::Arg<'a, 'b>; 4], app: clap::App<'c, 'd>) -> Self {
        let matches = app.get_matches();

        println!("is_present delete: {:?}",  matches.is_present("delete"));
        println!("matches.value_of(\"action\"): {:?}",  matches.value_of("action"));

        // let names = actions.iter().map(|a| a.name).collect::<Vec<_>>();

        return CLI{ actions: actions, matches: matches};
    }
    //
    // pub fn run(&self) -> ToResult<Request> {
    //     for i in self.actions.iter() {
    //         println!("> {:?}", i.name);
    //     }
    //
    //
    //     return Ok(req);
    // }
}

pub fn parse_matches(matches: clap::ArgMatches) -> ToResult<Request> {
    let (get, put, delete, list) = (matches.is_present("get"),
                                    matches.is_present("put"),
                                    matches.is_present("delete"),
                                    matches.is_present("list"));

    let action = match (get, put, delete, list) {
        (true, _, _, _) => Action::Get,
        (_, true, _, _) => Action::Put,
        (_, _, true, _) => Action::Delete,
        (_, _, _, true) => Action::List,
        _               => Action::ChangeDirectory,
    };

    println!("action: {:?}", action);
    let req = Request::new("foo", &PathBuf::from("."));
    return Ok(req);
}


//     // // Create a group, make it required, and add the above arguments
//     // .group(ArgGroup::with_name("action")
//     //     .required(true)
//     //     .args(&ACTIONS)
//
//
//     // to --save,--put # Save current dir as a bookmark
//     // to --save,--put foo # Save current directory as bookmark foo
//     // to --delete foo # Delete bookmark foo
//     // to --info foo # Show details of the bookmark foo
//     // to --list # List all the bookmarks
//     // to foo # Go to the foo bookmark
//     // reserve "-" so it can be used later.
//     // to - # go to the last bookmark you visited
//
//     // Take the matches from clap and convert them into name, directory, and
//     // action.
//     let request = match parse(program) {
//         Ok(value) => value,
//         Err(err) => panic!(err),
//     };
//
//
// }
//
// fn parse(cli: clap::App) -> ToResult<Request> {
//     let matches = cli.get_matches();
//     let pathname = matches.value_of("directory").unwrap_or("");
//
//     let directory = match resolve(pathname) {
//         Ok(value) => value,
//         Err(err) => return Err(err),
//     };
//
//     // Validate that the directory exists and is a directory.
//     // TODO: Move this into the resolve step.
//     if !is_valid_dir(&directory) {
//         panic!("invalid directory");
//     }
//
//     let name = match matches.value_of("name") {
//         Some(value) => value,
//         // directory.file_stem().map(|stem| stem.to_str()).unwrap()
//         None => "",
//     };
//
//     println!("directory: {:?}", directory);
//     println!("name: {:?}", name);
//
//     // let mut action = "save";
//     // // Increment the one requested (in a real program, we'd reset the lower numbers)
//     // let (save, delete, info, list) = (matches.is_present("save"),
//     //                              matches.is_present("delete"),
//     //                              matches.is_present("info"),
//     //                              matches.is_present("list"));
//     //
//     // match (save, delete, info, list) {
//     //     (true, _, _, _) => action = "save",
//     //     (_, true, _, _) => action = "delete",
//     //     (_, _, true, _) => action = "info",
//     //     (_, _, _, true) => action = "list",
//     //     _            => unreachable!(),
//     // };
//     //
//     // println!("action: {:?}", action);
//
//     let req = Request{ name: name.to_string(), directory: directory };
//
//     return Ok(req);
// }
//
// fn resolve(pathname: &str) -> ToResult<PathBuf> {
//     // TODO: Use a custom results tuple instead of panic!.
//     let mut absolute = match env::current_dir() {
//         Ok(value) => value,
//         Err(err) => return Err(ToError::Io(err)),
//     };
//
//     // Don't default to "." since it will be a literal translation creating
//     // dumb directories like "/foo/bar/."
//     if pathname != "." {
//         absolute.push(pathname);
//     }
//
//     return Ok(absolute);
// }
//
// fn is_valid_dir(directory: &PathBuf) -> bool {
//     let path = directory.clone();
//
//     let metadata = match fs::metadata(path) {
//         Ok(value) => value,
//         Err(_) => return false,
//     };
//
//     if metadata.is_dir() {
//         return true;
//     }
//
//     return false;
// }
//
//
// // static NUMBERS: &'static [i32] = &'static [1, 2, 3, 4, 5];
// // static C: [E; 3] = [E::V0, E::V1(0xDEADBEE), E::V0];
// static ACTIONS: [Arg; 1] = [
//     Arg::with_name("save")
//         .help("Saves bookmark")
//         .long("save")
//         .short("s")
// ];
//
// // .arg(Arg::with_name("delete")
// //     .help("Delete bookmark")
// //     .long("delete")
// //     .short("d"))
//
// // ["save", "delete", "info", "list"];

#[derive(Debug)]
pub struct Request {
    pub name: String,
    pub directory: PathBuf,
    pub action: Action,
}

impl Request {
    fn new(name: &str, directory: &PathBuf) -> Request {
        Request{ name: "foo".to_string(), directory: PathBuf::from("."), action: Action::Get}
    }
}
