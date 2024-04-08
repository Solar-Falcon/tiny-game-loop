use rustc_hash::FxHashMap;
use std::{fmt::Debug, hash::Hash};

#[cfg(feature = "winit")]
use winit::{
    event::{ElementState, Modifiers, MouseButton, WindowEvent},
    keyboard::{Key, KeyCode, ModifiersKeyState, NamedKey, PhysicalKey},
};

/// Keyboard modifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KeyMods {
    /// Left "shift" key.
    pub lshift: bool,
    /// Right "shift" key.
    pub rshift: bool,
    /// Left "alt" key.
    pub lalt: bool,
    /// Right "alt" key.
    pub ralt: bool,
    /// Left "control" key.
    pub lcontrol: bool,
    /// Right "control" key.
    pub rcontrol: bool,
    /// Left "super" key. This is the "windows" key on PC and "command" key on Mac.
    pub lsuper: bool,
    /// Right "super" key. This is the "windows" key on PC and "command" key on Mac.
    pub rsuper: bool,
}

/// Input state of a mouse button/keyboard key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InputState {
    /// The button has just been pressed.
    Pressed,
    /// The button is being held down.
    Down,
    /// The button has just been released.
    ///
    /// Note that it means that the key has **just** been released, **not** that it isn't held.
    Released,
}

impl InputState {
    /// The state is [`InputState::Pressed`].
    #[inline]
    pub fn is_pressed(&self) -> bool {
        matches!(self, InputState::Pressed)
    }

    /// The state is [`InputState::Pressed`] or [`InputState::Down`].
    #[inline]
    pub fn is_any_down(&self) -> bool {
        matches!(self, InputState::Pressed | InputState::Down)
    }

    /// The state is [`InputState::Released`].
    #[inline]
    pub fn is_released(&self) -> bool {
        matches!(self, InputState::Released)
    }
}

/// Input handler.
#[derive(Debug)]
pub struct Input<E: EventTrait> {
    mods: KeyMods,
    physical_keys: FxHashMap<E::PhysicalKey, InputState>,
    logical_keys: FxHashMap<E::LogicalKey, InputState>,
    mouse_buttons: FxHashMap<E::MouseButton, InputState>,
    mouse_pos: (f32, f32),
    mouse_scroll: (f32, f32),
}

impl<E> Input<E>
where
    E: EventTrait,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            mods: KeyMods::default(),
            physical_keys: FxHashMap::default(),
            logical_keys: FxHashMap::default(),
            mouse_buttons: FxHashMap::default(),
            mouse_pos: (0., 0.),
            mouse_scroll: (0., 0.),
        }
    }

    pub fn clear(&mut self) {
        self.mods = KeyMods::default();
        self.physical_keys.clear();
        self.logical_keys.clear();
        self.mouse_buttons.clear();
        self.mouse_pos = (0., 0.);
        self.mouse_scroll = (0., 0.);
    }

    /// Cursor position (from [`WindowEvent::CursorMoved`](https://docs.rs/winit/0.29.5/winit/event/enum.WindowEvent.html#variant.CursorMoved)).
    #[inline]
    pub fn mouse_pos(&self) -> (f32, f32) {
        self.mouse_pos
    }

    #[inline]
    pub fn mouse_scroll(&self) -> (f32, f32) {
        self.mouse_scroll
    }

    /// Get current keyboard modifiers.
    #[inline]
    pub fn key_mods(&self) -> KeyMods {
        self.mods
    }

    /// All input states of physical keys.
    #[inline]
    pub fn physical_keys(&self) -> &FxHashMap<E::PhysicalKey, InputState> {
        &self.physical_keys
    }

    /// Returns `true` if a physical key has just been pressed.
    #[inline]
    pub fn is_physical_key_pressed(&self, scancode: E::PhysicalKey) -> bool {
        self.physical_keys
            .get(&scancode)
            .map_or(false, InputState::is_pressed)
    }

    /// Returns `true` if a physical key is down.
    #[inline]
    pub fn is_physical_key_down(&self, scancode: E::PhysicalKey) -> bool {
        self.physical_keys
            .get(&scancode)
            .map_or(false, InputState::is_any_down)
    }

    /// Returns `true` if a physical key has just been released.
    #[inline]
    pub fn is_physical_key_released(&self, scancode: E::PhysicalKey) -> bool {
        self.physical_keys
            .get(&scancode)
            .map_or(false, InputState::is_released)
    }

    /// All input states of logical keys.
    #[inline]
    pub fn logical_keys(&self) -> &FxHashMap<E::LogicalKey, InputState> {
        &self.logical_keys
    }

    /// Returns `true` if a logical key has just been pressed.
    #[inline]
    pub fn is_logical_key_pressed(&self, key: E::LogicalKey) -> bool {
        self.logical_keys
            .get(&key)
            .map_or(false, InputState::is_pressed)
    }

    /// Returns `true` if a logical key is down.
    #[inline]
    pub fn is_logical_key_down(&self, key: E::LogicalKey) -> bool {
        self.logical_keys
            .get(&key)
            .map_or(false, InputState::is_any_down)
    }

    /// Returns `true` if a logical key has just been released.
    #[inline]
    pub fn is_logical_key_released(&self, key: E::LogicalKey) -> bool {
        self.logical_keys
            .get(&key)
            .map_or(false, InputState::is_released)
    }

    /// All input states of mouse buttons.
    #[inline]
    pub fn mouse_buttons(&self) -> &FxHashMap<E::MouseButton, InputState> {
        &self.mouse_buttons
    }

    /// Returns `true` if a mouse button has just been pressed.
    #[inline]
    pub fn is_mouse_button_pressed(&self, button: E::MouseButton) -> bool {
        self.mouse_buttons
            .get(&button)
            .map_or(false, InputState::is_pressed)
    }

    /// Returns `true` if a mouse button is down.
    #[inline]
    pub fn is_mouse_button_down(&self, button: E::MouseButton) -> bool {
        self.mouse_buttons
            .get(&button)
            .map_or(false, InputState::is_any_down)
    }

    /// Returns `true` if a mouse button has just been released.
    #[inline]
    pub fn is_mouse_button_released(&self, button: E::MouseButton) -> bool {
        self.mouse_buttons
            .get(&button)
            .map_or(false, InputState::is_released)
    }

    pub fn update_keys(&mut self) {
        self.physical_keys.retain(|_, state| match state {
            InputState::Pressed => {
                *state = InputState::Down;
                true
            }
            InputState::Down => true,
            InputState::Released => false,
        });

        self.logical_keys.retain(|_, state| match state {
            InputState::Pressed => {
                *state = InputState::Down;
                true
            }
            InputState::Down => true,
            InputState::Released => false,
        });

        self.mouse_buttons.retain(|_, state| match state {
            InputState::Pressed => {
                *state = InputState::Down;
                true
            }
            InputState::Down => true,
            InputState::Released => false,
        });
    }

    pub fn process_event(&mut self, event: &E) {
        if let Some(event) = event.convert() {
            match event {
                InputEvent::Modifiers(mods) => {
                    self.mods = mods.into();
                }
                InputEvent::Key {
                    physical_key,
                    logical_key,
                    repeat,
                    state,
                } => {
                    if !repeat {
                        self.physical_keys.insert(physical_key, state);

                        if let Some(logical_key) = logical_key {
                            self.logical_keys.insert(logical_key, state);
                        }
                    }
                }
                InputEvent::MouseButton { button, state } => {
                    self.mouse_buttons.insert(button, state);
                }
                InputEvent::MouseMoved(mouse_x, mouse_y) => {
                    self.mouse_pos = (mouse_x, mouse_y);
                }
                InputEvent::MouseScroll(scroll_x, scroll_y) => {
                    self.mouse_scroll = (scroll_x, scroll_y);
                }
            }
        }
    }
}

pub trait EventTrait: Sized {
    type PhysicalKey: Copy + Debug + Eq + Hash;
    type LogicalKey: Copy + Debug + Eq + Hash;
    type MouseButton: Copy + Debug + Eq + Hash;

    fn convert(&self) -> Option<InputEvent<Self>>;
}

#[derive(Clone, Debug)]
pub enum InputEvent<E: EventTrait> {
    Key {
        physical_key: E::PhysicalKey,
        logical_key: Option<E::LogicalKey>,
        repeat: bool,
        state: InputState, // Down shouldn't be used here
    },
    Modifiers(KeyMods),
    MouseMoved(f32, f32),
    MouseButton {
        button: E::MouseButton,
        state: InputState,
    },
    MouseScroll(f32, f32),
}

#[cfg(feature = "winit")]
impl From<ElementState> for InputState {
    #[inline]
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => InputState::Pressed,
            ElementState::Released => InputState::Released,
        }
    }
}

#[cfg(feature = "winit")]
impl From<Modifiers> for KeyMods {
    fn from(mods: Modifiers) -> Self {
        Self {
            lshift: mods.lshift_state() == ModifiersKeyState::Pressed,
            rshift: mods.rshift_state() == ModifiersKeyState::Pressed,
            lalt: mods.lalt_state() == ModifiersKeyState::Pressed,
            ralt: mods.ralt_state() == ModifiersKeyState::Pressed,
            lcontrol: mods.lcontrol_state() == ModifiersKeyState::Pressed,
            rcontrol: mods.rcontrol_state() == ModifiersKeyState::Pressed,
            lsuper: mods.lsuper_state() == ModifiersKeyState::Pressed,
            rsuper: mods.rsuper_state() == ModifiersKeyState::Pressed,
        }
    }
}

#[cfg(feature = "winit")]
impl EventTrait for WindowEvent {
    type PhysicalKey = KeyCode;
    type LogicalKey = NamedKey;
    type MouseButton = MouseButton;

    fn convert(&self) -> Option<InputEvent<Self>> {
        match self {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: false,
            } => {
                if let PhysicalKey::Code(key_code) = event.physical_key {
                    Some(InputEvent::Key {
                        physical_key: key_code,
                        logical_key: match event.logical_key {
                            Key::Named(key) => Some(key),
                            _ => None,
                        },
                        repeat: event.repeat,
                        state: event.state.into(),
                    })
                } else {
                    None
                }
            },
            WindowEvent::ModifiersChanged(mods) => {
                Some(InputEvent::Modifiers((*mods).into()))
            }
            WindowEvent::CursorMoved { device_id: _, position } => {
                Some(InputEvent::MouseMoved(position.x as _, position.y as _))
            }
            WindowEvent::MouseInput { device_id: _, state, button } => {
                Some(InputEvent::MouseButton { button: *button, state: (*state).into() })
            }
            WindowEvent::MouseWheel { device_id: _, delta, phase: _ } => {
                todo!()
            }
            _ => None,
        }
    }
}
