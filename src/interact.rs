use std::io::Write;

use crate::Menu;
use console::Key;
use console::Term;

impl Menu {
    pub fn serve(self: &Self) -> Result<usize, std::io::Error> {
        let mut term = Term::stdout();

        let mut usr_input = String::new();

        loop {
            let usr_key = term.read_key()?;

            match usr_key {
                Key::Char(c) => usr_input.push(c),
                Key::ArrowUp => {
                    term.clear_line()?;
                    println!("you went up!");
                }
                Key::Enter => {
                    term.clear_line()?;
                    term.write_line(&usr_input)?;
                    break;
                }
                _ => break,
            }
            term.clear_line()?;
            term.write(usr_input.as_bytes())?;
        }

        term.write_line("Hiya")?;

        term.write("(Quick! Say something!): ".as_bytes())?;
        //let user_response = term.read_line()?;

        println!(" -_- \n {}", usr_input);

        Ok(1)
    }
}
