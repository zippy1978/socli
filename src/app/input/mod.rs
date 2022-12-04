use self::key::Key;

pub mod key;
pub mod events;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}