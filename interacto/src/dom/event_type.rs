use std::fmt;

/**
 * The supported event types.
 * @category Helper
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Mouse(MouseEventType),
    Touch(TouchEventType),
    Key(KeyEventType),
    Input,
    Scroll,
    Change,
    Wheel,
}

/**
 * The mouse event type
 * @category Helper
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseEventType {
    Mousedown,
    Mouseup,
    Mousemove,
    Mouseover,
    Mouseout,
    Mouseenter,
    Mouseleave,
    Click,
    Auxclick,
}

/**
 * The touch event type
 * @category Helper
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchEventType {
    Touchstart,
    Touchend,
    Touchmove,
}

/**
 * The key event type
 * @category Helper
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    Keydown,
    Keyup,
}
