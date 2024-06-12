use std::env;
mod database;


use database::Database;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: rodo [add|rm|ls] [args]");
        return;
    }

    let command = &args[1];
    let mut db = Database::open(".rodorc");

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Usage: rodo add [contents]");
                return;
            }
            let contents = &args[2..].join(" ");
            let id = 1;
            db.add_record(&database::Record { id, content: contents.to_string() })
        }
        "rm" => {
            if args.len() < 3 {
                println!("Usage: rodo rm [id]");
                return;
            }
            println!("Remove");
        }
        "ls" => {
            println!("List");
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }

    // println!("{:?}",args[1])

}
