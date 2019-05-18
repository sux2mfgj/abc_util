/*
use std::io::{self, BufRead, Write};

#[macro_use]
extern crate clap;
use clap::App;

mod contest;
mod task;

use contest as atcoder;

fn evel_command(contest: &mut atcoder::Contest, command_line: String) -> Result<bool, String> {
    let command: Vec<_> = command_line.split(' ').collect();
    match command[0] {
        "contest_name" | "c" => {
            if command.len() != 2 {
                match &contest.name {
                    Some(name) => {
                        println!("{}", name);
                        Ok(true)
                    }
                    None => Err(
                        "contest_name requires a argument.\ne.g. > contest_name abc125".to_string(),
                    ),
                }
            } else {
                println!("setting...");

                let path = &format!("/contests/{0}/tasks", command[1]);
                let domain = "atcoder.jp".to_string();

                if let Some(tasks) = contest.set_name_and_get_tasks(&domain, path) {
                    println!("titles are ");
                    for task in tasks {
                        println!("{}", task);
                    }
                    contest.name = Some(command[1].to_string());
                    Ok(true)
                } else {
                    Err(format!("the argument `{}` is invalid", command[1]).to_string())
                }
            }
        }
        "show" | "s" => {
            if command.len() != 2 {
                if None == contest.name {
                    Err("set the contest_name first".to_string())
                }
                else
                {
                    Err("not yet implemented".to_string())
                }
            }
            else
            {
                Err("not yet implemented".to_string())
            }
        }
        "help" | "h" => {
            assert!(false, "not yet implemented");
            Err("not yet implemented".to_string())
        }
        "exit" => Ok(false),
        _ => Err("invalid command".to_string()),
    }
}

fn interactive_mode(contest: &mut atcoder::Contest) {
    loop {
        // show the prompt
        print!("> ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        let line = iterator.next().unwrap().unwrap();
        // throw away second line or later.

        // TODO
        let is_continue = match evel_command(contest, line) {
            Ok(cnt) => cnt,
            Err(why) => {
                println!("failed\n{}", why);
                true
            }
        };

        if !is_continue {
            println!("bye");
            break;
        }
    }
}


fn main() {
    /*
     * parse the command line arugmnet
     */
    let yaml = load_yaml!("../arguments.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut contest = atcoder::Contest::new();

    if matches.is_present("interactive") {
        interactive_mode(&mut contest);
        return;
    }

    assert!(false, "not yet implemented");
}
*/

mod extracter;

fn main() {

}
