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

/// A type for the rule names.
/// @category Checker
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum RuleName {
    Included,
    SameData,
    SameInteractions,
}

/// A type for the severity level.
/// @category Checker
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Severity {
    Err,
    Ignore,
    Warn,
}

/// A type that defines a linter rule.
/// @category Checker
pub type LinterRule = (RuleName, Severity);

/// The interaction type check.
/// @category Checker
trait Checker {
    fn set_linter_rules(&mut self, rules: Vec<LinterRule>);
    // fn check_rules(&self, binding: &Binding<Command, Interaction<InteractionData>, ()>, 
    //                binds: &[Binding<Command, Interaction<InteractionData>, ()>]);
    // fn check_same_interactions(&self, binding: &Binding<Command, Interaction<InteractionData>, ()>, 
    //                            binds: &[Binding<Command, Interaction<InteractionData>, ()>]);
    // fn check_same_data(&self, binding: &Binding<Command, Interaction<InteractionData>, ()>, 
    //                    binds: &[Binding<Command, Interaction<InteractionData>, ()>]);
    // fn check_included(&self, binding: &Binding<Command, Interaction<InteractionData>, ()>, 
    //                   binds: &[Binding<Command, Interaction<InteractionData>, ()>]);
}