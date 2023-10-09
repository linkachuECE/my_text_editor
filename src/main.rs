pub mod editor;
pub mod terminal;
pub mod row;
pub mod document;
use editor::Editor;
use docopt::Docopt;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = args.get(1).expect("Please pass in a filename as an argument");

    let mut editor = Editor::default();

    editor.open(filename.clone());
    editor.run();
}
