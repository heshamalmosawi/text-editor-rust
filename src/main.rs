use crate::modules::editor::Editor;

mod modules;

fn main() {
    Editor::default().run().unwrap();
}
