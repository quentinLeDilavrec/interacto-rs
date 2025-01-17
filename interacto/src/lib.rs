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
pub mod command;
pub mod undoble;
pub mod anoncmd;
pub mod binder;
pub mod binding;
pub mod checker;
pub mod interaction;
pub mod fsm;
pub mod dom;
pub mod r#impl;
pub mod undohistory;
pub mod undo;
pub mod linearhistory;

#[cfg(test)]
mod test;