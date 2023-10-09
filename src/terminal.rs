use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;
use crate::editor::Position;
use termion::{self, raw::{RawTerminal, IntoRawMode}};

pub struct Size {
    pub width: u16,
    pub height: u16
}
pub struct Terminal{
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>
}

impl Default for Terminal {
    fn default() -> Self {
        let term = termion::terminal_size().expect("Unable to open terminal");
        let _stdout = stdout().into_raw_mode().expect("Unable to open terminal");
        Terminal{
            size: Size {
                width: term.0,
                height: term.1
            },
            _stdout
        }
    }
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        stdout().flush()
    }

    pub fn cursor_position(&self, position: &Position) {
        let Position{mut x, mut y} = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);

        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    pub fn cursor_hide(&self){
        print!("{}", termion::cursor::Hide);
    }

    pub fn cursor_show(&self){
        print!("{}", termion::cursor::Show);
    }

    pub fn clear_screen(&self) -> Result<(), std::io::Error>{
        self.cursor_hide();
        print!("{}", termion::clear::All);
        self.cursor_position(&Position{x:0, y:0});
        self.cursor_show();
        self.flush()
    }

    pub fn clear_current_line(&self) {
        print!("{}", termion::clear::CurrentLine);
    }

}