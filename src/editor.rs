use termion::{raw::IntoRawMode, event::Key};
use termion::input::TermRead;
use std::{io::{stdin, stdout}, io::Error};

pub struct Editor {

}

fn die(e: Error){
    panic!("{:?}", e);
}

impl Editor {
    pub fn default() -> Self{
        Editor{}
    }
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        for key in stdin().keys() {
            match key {
                Ok(k) => {
                    match k {
                        Key::Char(c) => {
                            println!("{c}\r");
                        },
                        Key::Ctrl(c) => {
                            println!("ctrl-{c}\r");
                            if c.to_ascii_lowercase() == 'c' {
                                break;
                            }
                        },
                        Key::Alt(c) => {
                            println!("alt-{c}\r");
                        },
                        _ => {
                            println!("{:?}\r", k);
                        }
                    }
                },
                Err(e) => {
                    die(e);
                }
            }
        } 
    }
}