use console::{style, Key, Term};
use std::{io, time};

fn show_code(term: &Term, code: &TextArea, cursor: &mut Cursor) {
    cursor.reset_pos(&term);
    eprintln!("This is cyan: {}", style("rust").blink_fast().bold().red());
    cursor.move_cursor(term, 0, 2);
    let code_char = &code.buffer_char;
    for (_idx, code_text_char) in code_char.iter().enumerate() {
        eprint!("{}", style(code_text_char).for_stderr().bright().black())
    }

    cursor.reset_pos(&term);
    cursor.move_cursor(term, 0, 2);
}
fn show_user_code(code: &TextArea, user: &TextArea) {
    let code_char = &code.buffer_char;
    let user_char = &user.buffer_char;
    for (idx, user_text_char) in user_char.iter().enumerate() {
        if code_char[idx] == user_char[idx] {
            eprint!("{}", style(user_text_char).for_stderr().bright().green());
        } else {
            eprint!("{}", style(user_text_char).for_stderr().bright().red());
        }
    }
}

fn fill_code(code: &mut TextArea) {
    // let text = "alfred\nmarshall\nalfred".to_string();
    let text = "alfred\nmarshall\nalfred\nmarshall\nalfred\nmarshall\nalfred\nmarshall\nalfred"
        .to_string();
    for t in text.chars() {
        code.buffer_char.push(t);
    }
}

struct Cursor {
    pos_x: usize,
    pos_y: usize,
}

impl Cursor {
    fn move_cursor(&mut self, term: &Term, x: usize, y: usize) {
        term.move_cursor_to(self.pos_x + x, self.pos_y + y).unwrap();
        self.pos_x += x;
        self.pos_y += y;
    }

    fn reset_pos(&mut self, term: &Term) {
        self.pos_x = 0;
        self.pos_y = 3;
        term.move_cursor_to(self.pos_x, self.pos_y).unwrap();
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self { pos_x: 0, pos_y: 0 }
    }
}

struct TextArea {
    buffer_char: Vec<char>,
}

impl Default for TextArea {
    fn default() -> Self {
        Self {
            buffer_char: "".to_string().chars().collect::<Vec<char>>(),
        }
    }
}

fn main() -> io::Result<()> {
    let term = Term::stdout();
    term.clear_screen().unwrap();

    let heading = "Program type speed test".to_string();
    eprintln!("{}", style(heading).bold().underlined().on_bright().red());

    let mut cursor = Cursor {
        ..Default::default()
    };
    let mut code = TextArea {
        ..Default::default()
    };
    fill_code(&mut code);

    let mut user = TextArea {
        ..Default::default()
    };

    let start_time = time::Instant::now();
    loop {
        show_code(&term, &code, &mut cursor);
        show_user_code(&code, &user);
        let key = term.read_key()?;
        if let Key::Char(ch) = key {
            user.buffer_char.push(ch);
        }
        if key == Key::Backspace {
            term.clear_chars(1)?;
            user.buffer_char.pop();
        }
        if key == Key::Enter {
            user.buffer_char.push('\n');
        }
        if key == Key::Escape {
            break;
        }
        if code.buffer_char == user.buffer_char {
            eprintln!("\n\n\nDONE----------{:?}", start_time.elapsed());
            break;
        }
    }
    Ok(())
}
