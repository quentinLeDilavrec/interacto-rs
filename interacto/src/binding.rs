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

use std::collections::HashMap;
use std::fmt;

use crate::checker::{RuleName, Severity};
use crate::command::Command;
use crate::interaction::{Interaction, InteractionData};
// use rx::Observable;

/**
 * The concept of binding and its related services.
 * @typeParam C - The type of the command that will produce this binding.
 * @typeParam I - The type of the interaction that will use this binding.
 * @typeParam A - The type of the accumulator.
 * @typeParam D - The interaction data type (infered from the interaction type)
 * @category API Binding
 */
pub trait Binding<C, I, A, D>
where
    C: Command,
    I: Interaction<D>,
    D: InteractionData,
{
    /// The name of the binding
    fn name(&self) -> &str;

    /// Logs (or not) usage information of the binding for usage analysis
    fn log_usage(&self) -> bool;

    /// Logs (or not) binding execution information.
    fn log_binding(&self) -> bool;

    /// Logs (or not) command production information
    fn log_cmd(&self) -> bool;

    /// The accumulator used during the binding.
    fn accumulator(&self) -> &A;

    /// The user interaction.
    fn interaction(&self) -> &I;

    /// The command in progress or None.
    fn command(&self) -> Option<&C>;

    /// The linter rules specific to this binding.
    fn linter_rules(&self) -> &HashMap<RuleName, Severity>;

    /// States whether the binding is activated.
    fn activated(&self) -> bool;

    /// States whether the binding is running.
    fn running(&self) -> bool;

    /// States whether the command must be executed on each step of the interaction (and not only at the
    /// end of the interaction execution).
    fn continuous_cmd_execution(&self) -> bool;

    /// Information method.
    /// @returns The number of times the binding successfully ended (nevermind a command was created or not).
    fn times_ended(&self) -> usize;

    /// Information method.
    /// The number of times the binding was cancelled (nevermind a command was created or not).
    fn times_cancelled(&self) -> usize;

    // /// An RX observable objects that will provide the commands produced by the binding.
    // fn produces(&self) -> &Observable<C>;

    /// Does this binding has a 'when' predicate defined?
    fn is_when_defined(&self) -> bool;

    fn uninstall_binding(&self);

    /// Visiting the binding.
    fn accept_visitor(&self, visitor: &dyn VisitorBinding);
}

/// Visitor trait
pub trait VisitorBinding {
    fn visit_binding<C, I, A, D>(&self, binding: &dyn Binding<C, I, A, D>)
    where
        C: Command,
        I: Interaction<D>,
        D: InteractionData;
}