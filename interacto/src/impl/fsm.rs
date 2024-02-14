use std::collections::HashSet;

use crate::dom::{Event, EventType, KeyEventType, KeyboardEvent, WheelEvent};
use crate::fsm::{InputState, OutputState, Transition, VisitorFSM};

pub struct TransitionBase<E: Event> {
    src: Box<dyn OutputState>,
    tgt: Box<dyn InputState>,
    action: Box<dyn Fn(&E)>,
    guard: Box<dyn Fn(&E) -> bool>,
    _phantom: std::marker::PhantomData<E>,
}

impl<E: Event> TransitionBase<E> {
    fn new(
        src_state: Box<dyn OutputState>,
        tgt_state: Box<dyn InputState>,
        action: Option<Box<dyn Fn(&E)>>,
        guard: Option<Box<dyn Fn(&E) -> bool>>,
    ) -> Self {
        let action_fn = action.unwrap_or_else(|| Box::new(|_: &E| {}));
        let guard_fn = guard.unwrap_or_else(|| Box::new(|_: &E| true));
        Self {
            src: src_state,
            tgt: tgt_state,
            action: action_fn,
            guard: guard_fn,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<E: Event> Transition<E> for TransitionBase<E> {
    fn execute(&self, event: &dyn Event) -> Option<&dyn InputState> {
        if self.accept(event) && (self.guard)(event) {
            // Assuming `stop_current_timeout` is a method of `OutputState`
            self.src.stop_current_timeout();
            (self.action)(unsafe { &*(event as *const dyn Event as *const E) });
            self.src.exit();
            self.tgt.enter();
            Some(&self.tgt)
        } else {
            None
        }
    }
    
    fn guard(&self, event: &E) -> bool {
        todo!()
    }

    fn accept(&self, event: &dyn Event) -> bool {
        event.is::<E>()
    }

    fn accept_visitor(&self, visitor: &dyn VisitorFSM<dyn Event>) {
        visitor.visit_transition(self);
    }

    fn get_accepted_events(&self) -> HashSet<EventType> {
        unimplemented!()
    }

    fn target(&self) -> &dyn InputState {
        &self.tgt
    }

    fn uninstall(&self) {}
}

struct KeyTransition<E> {
    base: TransitionBase<E>,
    accepted_events: HashSet<EventType>,
}

impl<E: KeyboardEvent> KeyTransition<E> {
    fn new(
        src_state: Box<dyn OutputState>,
        tgt_state: Box<dyn InputState>,
        key_type: KeyEventType,
        action: Option<fn(&dyn KeyboardEvent)>,
        guard: Option<fn(&dyn KeyboardEvent) -> bool>,
    ) -> Self {
        let mut accepted_events = HashSet::new();
        accepted_events.insert(EventType::from(key_type));
        Self {
            base: TransitionBase::new(src_state, tgt_state, action, guard),
            accepted_events,
        }
    }
}

impl<E: KeyboardEvent> Transition<E> for KeyTransition<E> {
    fn accept(&self, event: &dyn KeyboardEvent) -> bool {
        event.is_key_event() && self.accepted_events.contains(&EventType::from(event.key_type))
    }

    fn target(&self) -> &dyn InputState {
        self.base.target()
    }

    fn execute(&self, event: &dyn Event) -> Option<&dyn InputState> {
        self.base.execute(event)
    }

    fn guard(&self, event: &dyn KeyboardEvent) -> bool {
        self.base.guard(event)
    }

    fn accept_visitor(&self, visitor: &dyn VisitorFSM<dyn Event>) {
        self.base.accept_visitor(visitor)
    }

    fn get_accepted_events(&self) -> &HashSet<EventType> {
        &self.accepted_events
    }

    fn uninstall(&self) {
        self.base.uninstall()
    }
}

// Define WheelTransition struct
struct WheelTransition<E: WheelEvent> {
    base: TransitionBase<E>,

}

impl<E: WheelEvent> WheelTransition<E> {
    // Define accepted_events as a static HashSet
    const ACCEPTED_EVENTS: HashSet<EventType> = {
        let mut set = HashSet::new();
        set.insert(EventType::Wheel);
        set
    };

    // Define accept method
    fn accept(&self, event: &dyn Event) -> bool {
        if let Some(wheel_event) = event.downcast_ref::<WheelEvent>() {
            self.get_accepted_events().contains(&EventType::Wheel)
        } else {
            false
        }
    }

    // Define get_accepted_events method
    fn get_accepted_events(&self) -> &HashSet<EventType> {
        &Self::ACCEPTED_EVENTS
    }
}
