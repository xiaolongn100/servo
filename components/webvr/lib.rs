/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

#![deny(unsafe_code)]

extern crate canvas_traits;
extern crate euclid;
extern crate ipc_channel;
#[macro_use]
extern crate log;
extern crate msg;
extern crate rust_webvr;
extern crate script_traits;
extern crate servo_channel;
extern crate servo_config;
extern crate webvr_traits;

mod webvr_thread;
pub use crate::webvr_thread::{WebVRThread, WebVRCompositorHandler};
