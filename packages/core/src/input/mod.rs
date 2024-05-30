use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, KeyEvent, MouseButton},
    keyboard::{KeyCode, PhysicalKey},
};

pub enum GameCommand {
    Forward = 1 << 0,
    Backward = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
    Up = 1 << 4,
    Down = 1 << 5,
    MouseLeft = 1 << 6,
    MouseRight = 1 << 7,
    Other = 1 << 8,
}

#[derive(Default)]
pub struct InputSystem {
    game_command: u32,
    cursor_delta: glam::Vec2,
    last_cursor: glam::Vec2,
}

impl InputSystem {
    pub fn game_command(&self) -> u32 {
        self.game_command
    }

    pub fn cursor_delta(&self) -> glam::Vec2 {
        self.cursor_delta
    }

    pub fn reset_cursor_delta(&mut self) {
        self.cursor_delta = glam::Vec2::ZERO;
    }

    pub fn handle_keyboard_input(&mut self, event: KeyEvent) {
        let game_command = match event.physical_key {
            PhysicalKey::Code(code) => match code {
                KeyCode::KeyW => GameCommand::Forward,
                KeyCode::KeyS => GameCommand::Backward,
                KeyCode::KeyA => GameCommand::Left,
                KeyCode::KeyD => GameCommand::Right,
                KeyCode::Space => GameCommand::Up,
                KeyCode::ShiftLeft => GameCommand::Down,
                _ => GameCommand::Other,
            },
            PhysicalKey::Unidentified(_) => GameCommand::Other,
        };
        if let GameCommand::Other = game_command {
            return;
        }

        match event.state {
            ElementState::Pressed => self.game_command |= game_command as u32,
            ElementState::Released => self.game_command &= !(game_command as u32),
        }
    }

    pub fn handle_cursor_moved(&mut self, position: PhysicalPosition<f64>) {
        if self.game_command & GameCommand::MouseRight as u32 != 0 {
            self.cursor_delta.x = position.x as f32 - self.last_cursor.x;
            self.cursor_delta.y = position.y as f32 - self.last_cursor.y;
        }
        self.last_cursor.x = position.x as f32;
        self.last_cursor.y = position.y as f32;
    }

    pub fn handle_mouse_input(&mut self, button: MouseButton, state: ElementState) {
        let game_command = match button {
            MouseButton::Left => GameCommand::MouseLeft,
            MouseButton::Right => GameCommand::MouseRight,
            _ => GameCommand::Other,
        };
        if let GameCommand::Other = game_command {
            return;
        }

        match state {
            ElementState::Pressed => self.game_command |= game_command as u32,
            ElementState::Released => self.game_command &= !(game_command as u32),
        }
    }
}
