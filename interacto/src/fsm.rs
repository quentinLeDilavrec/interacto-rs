/*
 * This file is part of Interacto.
 * Interacto is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * Interacto is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * You should have received a copy of the GNU General Public License
 * along with Interacto.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::collections::HashSet;

use crate::dom::{Event, EventType};
// use super::{FSMHandler, InputState, OutputState, State, VisitorFSM};

/// A finite state machine that defines the behavior of a user interaction.
/// # Category
/// API FSM
pub trait FSM {
    /// The set of the states that compose the FSM.
    /// This returns a copy of the real set.
    fn states(&self) -> &[&dyn State];

    /// The current state of the FSM.
    fn current_state(&self) -> &dyn OutputState;

    // /// An observable value for observing the current state of FSM during its execution.
    // fn current_state_observable(&self) -> broadcast::Receiver<(OutputState, OutputState)>;

    /// The initial state of the FSM.
    fn init_state(&self) -> &dyn OutputState;

    /// By default an FSM triggers its 'start' event when it leaves its initial state.
    /// In some cases, this is not the case. For example, a double-click interaction is an FSM that must trigger
    /// its start event when the FSM reaches... its terminal state. Similarly, a DnD must trigger its start event
    /// on the first move, not on the first press.
    /// The goal of this attribute is to identify the state of the FSM that must trigger the start event.
    /// By default, this attribute is set with the initial state of the FSM.
    fn starting_state(&self) -> &dyn State;

    /// True: The FSM started.
    fn started(&self) -> bool;

    /// Defines whether the FSM is an inner FSM (ie, whether it is included into another FSM as
    /// a sub-FSM transition).
    fn is_inner(&self) -> bool;

    // /// The current sub FSM in which this FSM is while running.
    // fn current_sub_fsm(&self) -> Option<Arc<dyn FSM>>;

    /// Logs (or not) information about the execution of the FSM.
    fn log(&self) -> bool;

    /// Processes the provided event to run the FSM.
    /// Returns true if the FSM correctly processed the event.
    fn process(&mut self, event: &dyn Event) -> bool;

    /// Starts the state machine.
    async fn on_starting(&mut self);

    /// Updates the state machine.
    async fn on_updating(&mut self);

    /// Cancels the state machine.
    async fn on_cancelling(&mut self);

    /// Terminates the state machine.
    async fn on_terminating(&mut self);

    /// Processes an error produced in the FSM.
    async fn on_error(&mut self, err: Box<dyn std::error::Error + Send + Sync>);

    /// Jobs to do when a timeout transition is executed.
    /// Because the timeout transition is based on a separated thread, the job
    /// done by this method must be executed in the UI thread.
    /// UI Platforms must override this method to do that.
    async fn on_timeout(&mut self);

    /// Stops the current timeout transition.
    async fn stop_current_timeout(&mut self);

    /// Enters a standard state.
    fn enter_std_state(&mut self, state: &dyn InputState);

    /// Adds an FSM handler.
    fn add_handler(&mut self, handler: Box<dyn FSMHandler>);

    /// Removes the given FSM handler from this FSM.
    fn remove_handler(&mut self, handler: &dyn FSMHandler);

    /// Reinitializes the FSM.
    /// Remaining events to process are however not clear.
    /// See `full_reinit` for that.
    fn reinit(&mut self);

    /// Reinitializes the FSM.
    /// Compared to `reinit` this method
    /// flushes the remaining events to process.
    fn full_reinit(&mut self);

    /// Uninstalls the FSM.
    /// Useful for flushing memory.
    /// The FSM must not be used after that.
    fn uninstall(&mut self);

    /// Visiting the FSM.
    fn accept_visitor(&self, visitor: &dyn VisitorFSM<dyn Event>);
}

struct FsmHandle;

/// The base type of an FSM state.
/// # category API FSM
pub trait State {
    /// The name of the state.
    fn name(&self) -> &str;

    /// The FSM that contains the state.
    fn fsm(&self) -> &FsmHandle;

    /// Checks whether the starting state of the fsm is this state.
    /// In this case, the fsm is notified about the starting of the FSM.
    /// # Throws
    /// - `CancelFSMException`
    fn check_starting_state(&self) -> Result<(), CancelFSMException>;

    /// Uninstall (ie flushes) the state.
    /// Useful to clear data.
    /// The state must not be used after that.
    fn uninstall(&mut self);

    /// Visiting the state.
    /// # Arguments
    /// - `visitor`: The visitor.
    fn accept_visitor(&self, visitor: &mut dyn VisitorFSM<dyn Event>);
}

/// Exception indicating cancellation of FSM.
pub struct CancelFSMException;

/**
 * The concept of FSM transition.
 * @category API FSM
 */
pub trait Transition<E: Event> {
    /**
     * The target state of the transition
     */
    fn target(&self) -> &dyn InputState;

    /**
     * Executes the transition.
     * @param event - The event to process.
     * @returns The potential output state.
     * @throws CancelFSMException If the execution cancels the FSM execution.
     */
    fn execute(&mut self, event: &dyn Event) -> Result<&dyn InputState,CancelFSMException>;

    fn guard(&self, event: &dyn Event) -> bool;

    fn accept(&self, event: &dyn Event) -> bool;

    /**
     * Visiting the transition.
     * @param visitor - The visitor.
     */
    fn accept_visitor<EV:Event, V: VisitorFSM<EV>>(&self, visitor: &mut V);

    /**
     * @returns The set of events accepted by the transition.
     */
    fn get_accepted_events(&self) -> &HashSet<EventType>;

    /**
     * Clean the transition when not used anymore.
     */
    fn uninstall(&mut self);
}

// Define the OutputState trait extending the State trait
pub trait OutputState: State {

    fn transitions(&self) -> &[Box<dyn Transition<dyn Event>>];

    fn exit(&mut self) -> Result<(), CancelFSMException>;

    fn process(&mut self, event: &dyn Event) -> bool;

    fn add_transition(&mut self, tr: Box<dyn Transition<dyn Event>>);
}

/// Defines a type of state that can receive input events.
/// # category
/// API FSM
pub trait InputState: State {
    /// Actions done while entering this state.
    /// # Errors
    /// - `CancelFSMException`: If entering the state leads to a cancelling of the FSM execution.
    fn enter(&self) -> Result<(), CancelFSMException>;
}

pub trait FSMHandler {
    fn fsm_starts(&mut self) -> Result<(), CancelFSMException>;

    fn fsm_updates(&mut self) -> Result<(), CancelFSMException>;

    fn fsm_stops(&mut self) -> Result<(), CancelFSMException>;

    fn fsm_cancels(&mut self);

    fn fsm_error(&mut self, err: Box<dyn std::error::Error>);

    fn pre_fsm_start(&mut self);

    fn pre_fsm_update(&mut self);

    fn pre_fsm_stop(&mut self);
}

/// The main interface for visiting FSMs.
/// # category API FSM
pub trait VisitorFSM<Event> {
    fn visit_fsm<F:FSM>(&mut self, fsm: &F);
    // fn visit_and_concurrent_fsm(&mut self, fsm: &dyn ConcurrentFSM<dyn FSM<Event>>);
    // fn visit_xor_concurrent_fsm(&mut self, fsm: &dyn ConcurrentFSM<dyn FSM<Event>>);
    fn visit_state(&mut self, state: &dyn OutputState);
    fn visit_init_state(&mut self, state: &dyn OutputState);
    fn visit_cancelling_state(&mut self, state: &dyn InputState);
    fn visit_terminal_state(&mut self, state: &dyn InputState);
    fn visit_transition(&mut self, transition: &dyn Transition<Event>);
    fn visit_timeout_transition(&mut self, transition: &dyn Transition<Event>);
}
