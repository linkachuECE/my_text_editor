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
    // Opens a document, returns error if the file cannot be opened
    pub fn open(&mut self, filepath: String) -> std::io::Result<()>{
        self.document = Document::open(filepath)?;
        Ok(())
    }

    // Clears the screen and continually refreshes
    pub fn run(&mut self) -> Result<(), std::io::Error> {

        // Set up the terminal in raw character mode
        let _stdout = stdout().into_raw_mode()?;

        // Clear the screen
        self.term.clear_screen()?;

        // Draw the rows initially
        self.draw_rows()?;

        // Loop until exit
        loop {

            // Refresh the screen, process any errors
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            // Wait until any keypresses, process any errors
            if let Err(error) = self.process_keypress() {
                die(error);
            }

            // If we are now set to quit, do so
            if self.should_quit {
                self.refresh_screen()?;
                break;
            }
        }

        // Exit succesfully
        Ok(())
    }

    // Refresh the screen
    pub fn refresh_screen(&mut self) -> Result<(), std::io::Error> {

        // Hide the cursor initially
        self.term.cursor_hide();

        // Set the cursor position to the very beginning
        self.term.cursor_position(&Position::default());

        // If we should quit, clear the screen, otherwise, set the cursor position back to the right spot
        if self.should_quit {
            self.term.clear_screen()?;
        } else {
            self.draw_rows()?;
            self.term.cursor_position(&self.cursor_position);
        }

        // Show the cursor and flush the terminal
        self.term.cursor_show();
        self.term.flush()
    }

    // Keypress handler for moving the cursor
    fn move_cursor(&mut self, key: Key) {
        
        // Grab the current position
        let Position{mut x, mut y} = self.cursor_position;

        // Find the key
        match key {

            // Move left
            Key::Left => {
                // If we are in middle or end of the row, move left
                if x != 0 {
                    x = x.saturating_sub(1);
                }
                // Otherwise, move to the end of the previous row 
                else if y != 0 {
                    y = y.saturating_sub(1);
                    x = self.document.row_len(y);
                }
            }
            
            // Move right
            Key::Right => {
                // If we are in the middle or beginning of the row, move right
                if x < self.document.row_len(y){
                    x = x.saturating_add(1);
                }
                // Otherwise, move to the beginning of the next row 
                else if y < self.document.line_count() - 1{
                    y += 1;
                    x = 0;
                }
            }

            // Move up
            Key::Up => {
                if y > 0 { 
                    y = y.saturating_sub(1);
                }

                if x > self.document.row_len(y){
                    x = self.document.row_len(y);
                }
            } 

            // Move down
            Key::Down => {
                // If we are not yet at the bottom of the document, move down a row
                if y < self.document.line_count() - 1 { 
                    y = y.saturating_add(1);
                }

                // If we are past the last letter in the row, move back to the end
                if x > self.document.row_len(y) - 1 {
                    x = self.document.row_len(y) - 1;
                }
            } 

            // Page down
            Key::PageDown => {
                y = self.document.line_count() - 1;
                x = self.document.row_len(y) - 1;
            },

            // Page up
            Key::PageUp => {
                y = 0;
                x = 0;
            },

            // Home
            Key::Home => {
                x = 0;
            }, 

            // End
            Key::End => {
                x = self.document.row_len(y) - 1;
            },
            _ => ()        
        }

        self.cursor_position.x = x;
        self.cursor_position.y = y;
    }

    // Handles keypresses for actually editing the text itself
    fn edit_text(&mut self, key: Key) {
        // Get the current cursor position
        let Position{mut x, mut y} = self.cursor_position;
        
        // Match the key
        match key {
            // Backspace
            Key::Backspace => {
                // If we're at the beginning of the row
                if self.cursor_position.x == 0 {
                    // If the row has at least one character, remove the row and append it to the previous row
                    if self.document.row_len(y) != 0 {
                        self.document.remove_and_append_to_previous_row(y); 
                    }
                    // Otherwise if the row has no characters 
                    else {
                        self.document.remove_row(y);
                    }
                }
                // Otherwise if we're in the middle or end of the row, just remove the character 
                else {
                    self.document.remove_from_row(self.cursor_position.y, self.cursor_position.x);
                }
                // Move to the left
                self.move_cursor(Key::Left);
            },

            // Delete
            Key::Delete => {
                // If we're in the middle or beginning of a row
                if self.cursor_position.x < self.document.row_len(self.cursor_position.y) {
                    self.document.remove_from_row(self.cursor_position.y, self.cursor_position.x + 1);
                }
                // Or if we are the end of a row, append the next row to the current
                else {
                    self.document.remove_and_append_to_previous_row(self.cursor_position.y + 1);
                }
            },
            // Enter key
            Key::Char('\n') => {
                self.document.add_row(self.cursor_position.y);
                self.move_cursor(Key::Right);
            }, 
            // Any other character
            Key::Char(c) => {
                self.document.insert_into_row(self.cursor_position.y, self.cursor_position.x, c);
                self.move_cursor(Key::Right);
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