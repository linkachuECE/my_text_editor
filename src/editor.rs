use termion::{raw::IntoRawMode, event::Key};
use std::io::{stdout, Error};
use crate::{terminal::Terminal, document::Document, row::Row};

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

#[derive(Default)]
pub struct Editor{
    should_quit: bool,
    term: Terminal,
    cursor_position: Position,
    document: Document
}

fn die(e: Error){
    panic!("{:?}", e);
}

impl Editor {
    pub fn open(&mut self, filepath: String) -> std::io::Result<()>{
        self.document = Document::open(filepath)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), std::io::Error> {
        let _stdout = stdout().into_raw_mode()?;
        self.term.clear_screen()?;
        self.draw_rows()?;

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }

            if self.should_quit {
                self.refresh_screen()?;
                break;
            }
        }
        Ok(())
    }

    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        self.term.cursor_hide();
        self.term.cursor_position(&Position::default());

        if self.should_quit {
            self.term.clear_screen()?;
        } else {
            self.term.cursor_position(&self.cursor_position);
        }

        self.term.cursor_show();
        self.term.flush()
    }

    fn move_cursor(&mut self, key: Key) {
        let Position{mut x, mut y} = self.cursor_position;
        match key {
            Key::Left => {
                if x != 0 {
                    x = x.saturating_sub(1);
                } else if y != 0 {
                    y = y.saturating_sub(1);
                    x = self.document.row_len(y) - 1;
                }
            }
            Key::Right => {
                if x < self.document.row_len(y) - 1 {
                    x = x.saturating_add(1);
                } else if y < self.document.line_count() - 1{
                    y += 1;
                    x = 0;
                }
            }
            Key::Up => {
                if y > 0 { 
                    y = y.saturating_sub(1);
                }

                if x > self.document.row_len(y) - 1 {
                    x = self.document.row_len(y) - 1;
                }
            } 
            Key::Down => {
                if y < self.document.line_count() - 1 { 
                    y = y.saturating_add(1);
                }

                if x > self.document.row_len(y) - 1 {
                    x = self.document.row_len(y) - 1;
                }
            } 
            Key::PageDown => {
                y = self.document.line_count() - 1;
                x = self.document.row_len(y) - 1;
            },
            Key::PageUp => {
                y = 0;
                x = 0;
            },
            Key::Home => {
                x = 0;
            }, 
            Key::End => {
                x = self.document.row_len(y) - 1;
            },
            _ => ()        
        }

        self.cursor_position.x = x;
        self.cursor_position.y = y;
    }

    fn edit_text(&mut self, key: Key) {
        let Position{mut x, mut y} = self.cursor_position;
        match key {
            Key::Backspace => {
                if self.cursor_position.x == 0 {
                    if self.document.row_len(y) == 0 {
                        self.document.remove_and_append_to_previous_row(y); 
                    } else {
                        self.document.remove_from_row(x - 1, y);
                        self.move_cursor(Key::Left)
                    }
                } else {
                    self.document.remove_from_row(self.cursor_position.x, self.cursor_position.y);
                    self.move_cursor(Key::Left);
                }
            },
            Key::Delete => {
                
            },
            Key::Char('\n') => {

            }, 
            Key::Char(k) => {
                if self.cursor_position.x == (self.term.size().width - 1) as usize{
                    self.move_cursor(Key::Down);
                    self.cursor_position.x = 0;
                } else {
                    self.move_cursor(Key::Right);
                }
            },
            _ => ()
        }
    }

    fn process_keypress(&mut self) -> Result<(),std::io::Error> {
        let pressed_key = self.term.read_key()?;

        match pressed_key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::Home |
            Key::PageDown |
            Key::PageUp |
            Key::End |
            Key::Left |
            Key::Right |
            Key::Up | 
            Key::Down => self.move_cursor(pressed_key),
            Key::Backspace |
            Key::Delete => self.edit_text(pressed_key), 
            Key::Char(_) => self.edit_text(pressed_key),
            _ => ()
            
        }

        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        println!("{}\r", row.render(0, self.term.size().width as usize));
    }

    fn draw_rows(&self) -> Result<(), std::io::Error> {
        for i in 0..self.term.size().height - 1 {
            self.term.clear_current_line();
            if let Some(row) = self.document.row(i as usize) {
                self.draw_row(row);
            } else {
                println!("~\r");
            }
        }
        Ok(())
    }
}