extern crate sdl2;

use sdl2::keyboard::Keycode;

use std::collections::HashSet;

pub struct InputManager {
    held_keys: HashSet<Keycode>,
    pressed_keys: HashSet<Keycode>,
    released_keys: HashSet<Keycode>,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            held_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, events: &sdl2::EventPump) {
        let new_held_keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        self.pressed_keys = &new_held_keys - &self.held_keys;
        self.released_keys = &self.held_keys - &new_held_keys;
        self.held_keys = new_held_keys;
    }

    pub fn get_key_down(&self, keycode: Keycode) -> bool {
        self.held_keys.contains(&keycode)
    }

    pub fn get_key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    pub fn get_key_released(&self, keycode: Keycode) -> bool {
        self.released_keys.contains(&keycode)
    }
}
