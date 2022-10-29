use crate::global::{structs::KeyAction, traits::ConfigTrait};
use crossterm::event::KeyCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typemap::Key;

// The outer HashMap<KeyCode, T> gets a value for whatever code is being pressed
// The inner HashMap<u8, KeyAction> gets the action using a u8 representing a key modifier
// The u8 for each modifier key can be found in https://docs.rs/crossterm/latest/src/crossterm/event.rs.html#587-603
// The sum of u8 for each key modifier would mean having all of those modifiers at the same time
/// `keybindings.yml`, contains a hashmap of key + key modifier to action pair
#[derive(Serialize, Deserialize, Clone)]
pub struct KeyBindingsConfig(pub HashMap<KeyCode, HashMap<u8, KeyAction>>);

impl ConfigTrait for KeyBindingsConfig {
    const LABEL: &'static str = "keybindings";
}

impl Key for KeyBindingsConfig {
    type Value = Self;
}

impl Default for KeyBindingsConfig {
    fn default() -> Self {
        let mut map = HashMap::new();

        let mut left = HashMap::new();
        left.insert(0, KeyAction::MoveLeft);
        map.insert(KeyCode::Left, left);

        let mut right = HashMap::new();
        right.insert(0, KeyAction::MoveRight);
        map.insert(KeyCode::Right, right);

        let mut up = HashMap::new();
        up.insert(0, KeyAction::MoveUp);
        map.insert(KeyCode::Up, up);

        let mut down = HashMap::new();
        down.insert(0, KeyAction::MoveDown);
        map.insert(KeyCode::Down, down);

        let mut enter = HashMap::new();
        enter.insert(0, KeyAction::Select);
        map.insert(KeyCode::Enter, enter);

        let mut escape = HashMap::new();
        escape.insert(0, KeyAction::Deselect);
        map.insert(KeyCode::Esc, escape);

        let mut q = HashMap::new();
        q.insert(0, KeyAction::Exit);
        map.insert(KeyCode::Char('q'), q);

        let mut backspace = HashMap::new();
        backspace.insert(0, KeyAction::Back);
        map.insert(KeyCode::Backspace, backspace);

        let mut r = HashMap::new();
        r.insert(2, KeyAction::Reload);
        map.insert(KeyCode::Char('r'), r);

        // vim keybindings
        let mut h = HashMap::new();
        h.insert(0, KeyAction::MoveLeft);
        map.insert(KeyCode::Char('h'), h);

        let mut j = HashMap::new();
        j.insert(0, KeyAction::MoveDown);
        map.insert(KeyCode::Char('j'), j);

        let mut k = HashMap::new();
        k.insert(0, KeyAction::MoveUp);
        map.insert(KeyCode::Char('k'), k);

        let mut l = HashMap::new();
        l.insert(0, KeyAction::MoveRight);
        map.insert(KeyCode::Char('l'), l);

        Self(map)
    }
}