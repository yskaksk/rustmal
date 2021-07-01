use rustmal::step0_repl::rep;

use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => println!("{}", rep(&line)),
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
}
