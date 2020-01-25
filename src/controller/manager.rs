/*
 * src/controller/manager.rs
 * tasinput2 - Plugin for creating TAS inputs
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::{ControllerCommand, ControllerResponse};
use std::sync::mpsc::{Receiver, Sender};

pub fn controller_manager(tx: Sender<ControllerResponse>, rx: Receiver<ControllerCommand>) {
    let mut continue_loop = true;

    'threadloop: loop {
        let break_loop = |_| {
            continue_loop = false;
        };

        // receive the command from the receiving outlet
        #[allow(deprecated)]
        match rx.recv().unwrap_or(ControllerCommand::NoOp) {
            #[allow(deprecated)]
            ControllerCommand::NoOp => tx
                .send(ControllerResponse::NoResponse)
                .unwrap_or_else(break_loop),
            ControllerCommand::End => {
                continue_loop = false;
                tx.send(ControllerResponse::NoResponse)
                    .unwrap_or_else(break_loop);
            }
        }

        if !continue_loop {
            break 'threadloop;
        }
    }
}