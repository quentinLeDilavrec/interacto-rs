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

use crate::{dom::Node, fsm::FSM};

/// Interaction data type.
pub trait InteractionData {}

/// Visitor interaction trait.
pub trait VisitorInteraction {}

/// Finite State Machine trait.

/// The concept of user interaction.
pub trait Interaction<D: InteractionData> {
    /// Sets whether the user interaction will stop immediately the propagation
    /// of events processed by this user interaction to others listeners.
    fn set_stop_immediate_propagation(&mut self, stop: bool);

    /// Sets whether the default behavior associated to the event
    /// will be executed.
    fn set_prevent_default(&mut self, prevent: bool);

    /// The FSM of the user interaction.
    fn fsm(&self) -> &dyn FSM;

    /// The interaction data of the user interaction. Cannot be null.
    fn data(&self) -> &D;

    /// The real name of the interaction.
    fn name(&self) -> &str;

    /// The registered nodes.
    fn registered_nodes(&self) -> &HashSet<()>;

    /// The nodes for which the user interaction will register their child nodes dynamically.
    fn dynamic_registered_nodes(&self) -> &HashSet<()>;

    /// Whether the user interaction is running.
    fn is_running(&self) -> bool;

    /// True if the user interaction is activated.
    fn is_activated(&self) -> bool;

    /// Sets whether the user interaction is activated.
    fn set_activated(&mut self, activated: bool);

    /// Sets the logging of the user interaction.
    fn log(&mut self, log: bool);

    /// Register to nodes.
    fn register_to_nodes(&mut self, widgets: &[()]);

    /// Permits to listen any change in the content (ie children) of the given node.
    fn register_to_node_children(&mut self, element_to_observe: &dyn Node);

    /// Sets the timeout (in ms) to be used by the throttling.
    fn set_throttle_timeout(&mut self, timeout: u64);

    /// Fully reinitialises the user interaction, its data and its FSM (flushes FSM revents).
    fn full_reinit(&mut self);

    /// Reinitialises the user interaction, its data and its FSM.
    fn reinit(&mut self);

    /// Reinitialises the interaction data.
    fn reinit_data(&mut self);

    /// Uninstall the user interaction. Used to free memory.
    /// Then, user interaction can be used any more.
    fn uninstall(&mut self);

    /// Visiting the interaction and its FSM.
    fn accept_visitor(&self, visitor: &dyn VisitorInteraction);
}