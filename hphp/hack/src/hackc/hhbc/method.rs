// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use bitflags::bitflags;
use serde::Serialize;

use crate::Body;
use crate::MethodName;
use crate::Visibility;

#[derive(Debug, Serialize)]
#[repr(C)]
pub struct Method {
    pub visibility: Visibility,
    pub name: MethodName,
    pub body: Body,
    pub flags: MethodFlags,
}

bitflags! {
    #[derive(Default, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    #[repr(C)]
    pub struct MethodFlags: u16 {
        const IS_ASYNC = 1 << 0;
        const IS_GENERATOR = 1 << 1;
        const IS_PAIR_GENERATOR = 1 << 2;
        const IS_CLOSURE_BODY = 1 << 3;
    }
}

impl Method {
    pub fn is_closure_body(&self) -> bool {
        self.flags.contains(MethodFlags::IS_CLOSURE_BODY)
    }
}
