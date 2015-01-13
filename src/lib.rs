// Copyright 2015 the simplectic-noise developers. For a full listing of the
// authors, refer to the AUTHORS file at the top-level directory of this
// distribution.
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

#![allow(unstable)]

pub use seed::Seed;
pub use math::{Point2, Point3, Point4};
pub use simplectic::{simplectic2, simplectic3, simplectic4};

mod gradient;
mod math;
mod seed;
mod simplectic;
