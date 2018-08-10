extern crate sdl2;

use maths::Vector2i;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

pub struct InputManager {
    //Keyboard state
    held_keys: HashSet<Keycode>,
    pressed_keys: HashSet<Keycode>,
    released_keys: HashSet<Keycode>,
    //Mouse state
    mouse_position: Vector2i,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            held_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            released_keys: HashSet::new(),
            mouse_position: Vector2i::new(0, 0),
        }
    }

    ///Update InputManager with new events
    pub fn update(&mut self, events: &sdl2::EventPump) {
        //Update keyboard state
        let new_held_keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        self.pressed_keys = &new_held_keys - &self.held_keys;
        self.released_keys = &self.held_keys - &new_held_keys;
        self.held_keys = new_held_keys;

        //Update mouse
        let mouse_state = events.mouse_state();
        self.mouse_position = Vector2i::new(mouse_state.x(), mouse_state.y());
    }

    ///Key is currently held down
    pub fn key_down(&self, keycode: Keycode) -> bool {
        self.held_keys.contains(&keycode)
    }

    ///Key was pressed this frame
    pub fn key_pressed(&self, keycode: Keycode) -> bool {
        self.pressed_keys.contains(&keycode)
    }

    ///Key was released this frame
    pub fn key_released(&self, keycode: Keycode) -> bool {
        self.released_keys.contains(&keycode)
    }


    ///Mouse position in pixels, relative to the top left corner.
    pub fn mouse_position(&self) -> Vector2i { self.mouse_position }
}
