mod event_type;

pub use event_type::*;

/// TODO DOM stuff
pub trait Node {}

/// TODO DOM stuff
pub trait Event {}

/// TODO DOM stuff
pub trait KeyboardEvent: Event {}

/// TODO DOM stuff
pub trait MouseEvent: Event {}

/// TODO DOM stuff
pub trait WheelEvent: MouseEvent {}