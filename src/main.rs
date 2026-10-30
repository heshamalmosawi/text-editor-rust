use crate::modules::editor::Editor;

mod modules;

fn main() {
    let editor = Editor::default();
    editor.run();
}
