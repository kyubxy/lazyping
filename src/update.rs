use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::model::App;

/// handles the key events and updates the state of [`app`].
pub fn on_key_event(app: &mut App, key: KeyEvent) {
    match (key.modifiers, key.code) {
        (_, KeyCode::Esc | KeyCode::Char('q')) | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
            app.quit()
        }
        // add other key handlers here.
        _ => {}
    }
}
