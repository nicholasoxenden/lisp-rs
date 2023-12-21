use lisp_rs::env;
use lisp_rs::eval;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::cell::RefCell;
use std::rc::Rc;

const PROMPT: &str = "lisp-rs> ";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let mut env = Rc::new(RefCell::new(env::Env::new()));

    loop {
        let readline = rl.readline(PROMPT);
        match readline {
            Ok(line) => {
                let val = eval::eval(&line, &mut env)?;
                val.show()
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
    Ok(())
}
