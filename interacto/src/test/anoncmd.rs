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

use std::sync::{Arc, Mutex};

use crate::command::Command;
use crate::{
    anoncmd::AnonCmd,
    command::{Cmd, CustomCmd},
};

#[test]
fn can_do_ok_cmd() {
    assert_eq!(AnonCmd::new(|| {}).as_command().can_execute(), true);
}

#[test]
fn execute() {
    let ok = Arc::new(Mutex::new(false));
    let mut cmd = Cmd::new(AnonCmd::new(|| {
        let mut data = ok.lock().unwrap();
        *data = true;
    }));
    cmd.execute();
    assert_eq!(*ok.lock().unwrap(), true);
}

#[test]
fn had_effect() {
    let mut cmd = Cmd::new(AnonCmd::new(|| {}));
    cmd.execute();
    cmd.done();
    assert_eq!(cmd.had_effect(), true);
}
