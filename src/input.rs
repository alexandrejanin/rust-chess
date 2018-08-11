use maths::Vector2i;
use sdl2::{self, keyboard::Keycode, mouse::MouseButton};
use std::collections::HashMap;

///Represents the current state of a keyboard key.
#[derive(Copy, Clone, PartialEq, Eq)]
struct KeyState {
    ///Is the key currently held down?
    down: bool,
    ///Did the key's state change this frame?
    changed: bool,
}

impl KeyState {
    fn down(&self) -> bool {
        self.down
    }
    fn pressed(&self) -> bool {
        self.down && self.changed
    }
    fn released(&self) -> bool {
        !self.down && self.changed
    }

    fn update(&mut self, pressed: bool) {
        self.changed = pressed != self.down;
        self.down = pressed;
    }
}

pub struct InputManager {
    //Keyboard state
    key_state: HashMap<Keycode, KeyState>,
    //Mouse state
    mouse_position: Vector2i,
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            key_state: HashMap::new(),
            mouse_position: Vector2i::new(0, 0),
        }
    }

    ///Update InputManager with new events
    pub fn update(&mut self, events: &sdl2::EventPump) {
        //Update keyboard state
        let new_key_state: Vec<(Keycode, bool)> = events
            .keyboard_state()
            .scancodes()
            //Convert from scancode to keycode
            .filter_map(|(scancode, pressed)|
                if let Some(keycode) = Keycode::from_scancode(scancode) {
                    Some((keycode, pressed))
                } else {
                    None
                }
            )
            .collect();

        for (keycode, pressed) in &new_key_state {
            if !self.key_state.contains_key(keycode) {
                self.key_state.insert(
                    *keycode,
                    KeyState {
                        down: *pressed,
                        changed: false,
                    },
                );
            }

            self.key_state
                .get_mut(keycode)
                .expect(&format!("Keycode not found: {:?}", keycode))
                .update(*pressed);
        }

        //Update mouse
        let mouse_state = events.mouse_state();
        self.mouse_position = Vector2i::new(mouse_state.x(), mouse_state.y());
    }

    ///Key is currently held down
    pub fn key_down(&self, keycode: Keycode) -> bool {
        self.key_state[&keycode].down()
    }

    ///Key was pressed this frame
    pub fn key_pressed(&self, keycode: Keycode) -> bool {
        self.key_state[&keycode].pressed()
    }

    ///Key was released this frame
    pub fn key_released(&self, keycode: Keycode) -> bool {
        self.key_state[&keycode].released()
    }

    ///Mouse position in pixels, relative to the top left corner.
    pub fn mouse_position(&self) -> Vector2i {
        self.mouse_position
    }
}
