//
// input.rs
//
use winit::event::{DeviceEvent, ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent};

/// Input state helper
///
/// Call `Input::update` for every `winit::event::Event` you receive from winit.
/// `Input::update` returning true indicates a step has occured.
pub struct Input {
    mouse_actions: Vec<MouseAction>,
    key_actions: Vec<KeyAction>,
    key_held: [bool; 255],
    mouse_held: [bool; 255],
    mouse_delta: Option<(f32, f32)>,
    cursor_point: Option<(f32, f32)>,
    cursor_point_prev: Option<(f32, f32)>,
}

#[derive(Clone)]
pub enum KeyAction {
    Pressed(VirtualKeyCode),
    Released(VirtualKeyCode),
}

#[derive(Clone)]
pub enum MouseAction {
    Pressed(MouseButton),
    Released(MouseButton),
}

#[allow(dead_code)]
impl Input {
    pub fn new() -> Input {
        Input {
            mouse_actions: vec![],
            key_actions: vec![],
            key_held: [false; 255],
            mouse_held: [false; 255],
            mouse_delta: None,
            cursor_point: None,
            cursor_point_prev: None,
        }
    }

    fn step(&mut self) {
        self.mouse_actions = vec![];
        self.key_actions = vec![];
        self.mouse_delta = None;
        self.cursor_point_prev = self.cursor_point;
    }

    fn handle_window_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(keycode) = input.virtual_keycode {
                    match input.state {
                        ElementState::Pressed => {
                            self.key_held[keycode as usize] = true;
                            self.key_actions.push(KeyAction::Pressed(keycode));
                        }
                        ElementState::Released => {
                            self.key_held[keycode as usize] = false;
                            self.key_actions.push(KeyAction::Released(keycode));
                        }
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    self.mouse_held[mouse_button_to_int(*button)] = true;
                    self.mouse_actions.push(MouseAction::Pressed(*button));
                }
                ElementState::Released => {
                    self.mouse_held[mouse_button_to_int(*button)] = false;
                    self.mouse_actions.push(MouseAction::Released(*button));
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_point = Some((position.x as _, position.y as _));
            }
            _ => (),
        }
    }

    fn handle_device_event(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.mouse_delta = Some((delta.0 as _, delta.1 as _))
            }
            _ => (),
        }
    }

    /// Pass every winit event to this function and run your application logic when it returns true.
    ///
    /// The following winit events are handled:
    /// * `Event::NewEvents` clears all internal state.
    /// * `Event::MainEventsCleared` causes this function to return true, signifying a "step" has completed.
    /// * `Event::WindowEvent` and `Event::DeviceEvent` updates internal state, this will affect the result of accessor methods immediately.
    pub fn update<T>(&mut self, event: &Event<T>) -> bool {
        match &event {
            Event::NewEvents(_) => {
                self.step();
                false
            }
            Event::WindowEvent { event, .. } => {
                self.handle_window_event(event);
                false
            }
            Event::DeviceEvent { event, .. } => {
                self.handle_device_event(event);
                false
            }
            Event::MainEventsCleared => true,
            _ => false,
        }
    }

    /// Returns true when the specified keyboard key goes from "not pressed" to "pressed"
    /// Otherwise returns false
    pub fn key_pressed(&self, key_code: VirtualKeyCode) -> bool {
        for action in &self.key_actions {
            if let &KeyAction::Pressed(code) = action {
                if code == key_code {
                    return true;
                }
            }
        }
        false
    }

    /// Returns true when the specified mouse button goes from "not pressed" to "pressed"
    /// Otherwise returns false
    pub fn mouse_pressed(&self, mouse_button: MouseButton) -> bool {
        for action in &self.mouse_actions {
            if let &MouseAction::Pressed(button) = action {
                if button == mouse_button {
                    return true;
                }
            }
        }
        false
    }

    /// Returns true when the specified keyboard key goes from "pressed" to "not pressed"
    /// Otherwise returns false
    pub fn key_released(&self, key_code: VirtualKeyCode) -> bool {
        for action in &self.key_actions {
            if let &KeyAction::Released(code) = action {
                if code == key_code {
                    return true;
                }
            }
        }
        false
    }

    /// Returns true when the specified mouse button goes from "pressed" to "not pressed"
    /// Otherwise returns false
    pub fn mouse_released(&self, mouse_button: MouseButton) -> bool {
        for action in &self.mouse_actions {
            if let &MouseAction::Released(button) = action {
                if button == mouse_button {
                    return true;
                }
            }
        }
        false
    }

    /// Returns true while the specified keyboard key remains "pressed"
    /// Otherwise returns false
    pub fn key_held(&self, key_code: VirtualKeyCode) -> bool {
        self.key_held[key_code as usize]
    }

    /// Returns true while the specified mouse button remains "pressed"
    /// Otherwise returns false
    pub fn mouse_held(&self, mouse_button: MouseButton) -> bool {
        self.mouse_held[mouse_button_to_int(mouse_button) as usize]
    }

    /// Returns the change in mouse coordinates that occured during the last step.
    /// Returns `(0.0, 0.0)` if None
    pub fn mouse_diff(&self) -> (f32, f32) {
        self.mouse_delta.unwrap_or((0.0, 0.0))
    }

    /// Returns `None` when the cursor is outside of the window.
    /// Otherwise returns the cursor coordinates in pixels
    pub fn cursor(&self) -> Option<(f32, f32)> {
        self.cursor_point
    }

    /// Returns the change in cursor coordinates that occured during the last step.
    /// Returns `(0.0, 0.0)` if the cursor is outside of the window.
    pub fn cursor_diff(&self) -> (f32, f32) {
        match (self.cursor_point, self.cursor_point_prev) {
            (Some(cur), Some(prev)) => (cur.0 - prev.0, cur.1 - prev.1),
            _ => (0.0, 0.0),
        }
    }
}

fn mouse_button_to_int(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => byte as usize,
    }
}
