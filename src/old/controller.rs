use piston_window::*;

#[derive(Default)]
pub struct Controller {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub fire: bool,
}

impl Controller {
    pub fn update(&mut self, button: Button, state: ButtonState) {
        use Button::*;
        match button {
            Keyboard(Key::W) => { self.up = state == ButtonState::Press; },
            Keyboard(Key::A) => { self.left = state == ButtonState::Press; },
            Keyboard(Key::S) => { self.down = state == ButtonState::Press; },
            Keyboard(Key::D) => { self.right = state == ButtonState::Press; },
            Keyboard(Key::Space) => { self.fire = state == ButtonState::Press; }
            _ => (),
        };
    }
}
