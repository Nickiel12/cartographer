use std::io::Write;

use crate::Menu;
use console::Term;

impl Menu {
    pub fn serve(self: &Self) -> Result<usize, std::io::Error> {
        let mut term = Term::stdout();

        term.write_line("Hiya")?;

        term.write("(Quick! Say something!): ".as_bytes())?;
        let user_response = term.read_line()?;

        println!(" -_- \n {}", user_response);

        Ok(1)
    }
}
