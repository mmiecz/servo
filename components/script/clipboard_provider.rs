/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use script_traits::ScriptToConstellationChan;

pub trait ClipboardProvider {
    // blocking method to get the clipboard contents
    fn clipboard_contents(&mut self) -> String;
    // blocking method to set the clipboard contents
    fn set_clipboard_contents(&mut self, _: String);
}

impl ClipboardProvider for ScriptToConstellationChan {
    fn clipboard_contents(&mut self) -> String {
        unimplemented!()
    }
    fn set_clipboard_contents(&mut self, _s: String) {
        unimplemented!()
    }
}
