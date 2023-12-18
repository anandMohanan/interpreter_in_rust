use std::io::BufRead;

use interpreter::repl::Repl;

pub fn main() {
    let repl = Repl::new();
    let stdin = std::io::stdin();

    loop {
        if let Some(Ok(ref line)) = stdin.lock().lines().next() {
            for item in repl.line(line).iter() {
                println!("{:#?}", item);
            }
        }
    }
}
