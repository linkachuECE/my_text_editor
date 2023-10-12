pub mod editor;
pub mod terminal;
pub mod row;
pub mod document;

use editor::Editor;

fn main() -> Result<(), std::io::Error>{
    let args: Vec<String> = std::env::args().collect();
    let default = "test.txt".to_string();
    let filename = args.get(1).unwrap_or(&default);

    let mut editor = Editor::default();
    editor.open(filename.to_string())?;
    editor.run()?;

    Ok(())
}
