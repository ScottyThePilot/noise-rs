// Copyright 2015 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! An example of generating constant valued noise

extern crate noise;

use noise::modules::Checkerboard;

mod debug;

fn main() {
    debug::render_noise_module("checkerboard.png", Checkerboard::new(), 1024, 1024, 100);
    debug::render_noise_module("checkerboard-2.png", Checkerboard::new().set_size(2), 1024, 1024, 100);
}