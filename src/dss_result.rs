// Copyright 2024 Open Energy Solutions Inc.
// 
// Licensed under the Apache License, Version 2.0 (the License);
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{ffi::NulError, num::TryFromIntError, str::Utf8Error};
pub type Result<T> = std::result::Result<T, DssError>;

#[derive(Debug)]
pub enum DssError {
    InvalidUtf8(Utf8Error),
    NullCPtr,
    CallFail,
    TryFromInt,
    TryFromUint,
}

impl From<Utf8Error> for DssError {
    fn from(err: Utf8Error) -> Self {
        DssError::InvalidUtf8(err)
    }
}

impl From<NulError> for DssError {
    fn from(_: NulError) -> Self {
        DssError::NullCPtr
    }
}

impl From<TryFromIntError> for DssError {
    fn from(_: TryFromIntError) -> Self {
        DssError::TryFromInt
    }
}
