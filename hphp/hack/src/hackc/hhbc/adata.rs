// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use hash::IndexSet;

use crate::AdataId;
use crate::TypedValue;

#[derive(Debug, Default)]
pub struct AdataState {
    shared: IndexSet<TypedValue>,
}

impl AdataState {
    pub fn intern(&mut self, tv: TypedValue) -> AdataId {
        let (i, _) = self.shared.insert_full(tv);
        AdataId::new(i)
    }

    pub fn finish(self) -> Vec<TypedValue> {
        self.shared.into_iter().collect()
    }
}
