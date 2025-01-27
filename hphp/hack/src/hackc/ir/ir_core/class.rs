// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the "hack" directory of this source tree.

use crate::Attr;
use crate::Attribute;
use crate::ClassName;
use crate::Constant;
use crate::CtxConstant;
use crate::Method;
use crate::PropName;
use crate::Property;
use crate::Requirement;
use crate::Span;
use crate::TypeConstant;
use crate::TypeInfo;
use crate::UpperBound;

/// This represents a Hack class or enum in IR.
#[derive(Debug)]
pub struct Class {
    /// Class attributes.
    pub attributes: Vec<Attribute>,

    /// Base class.
    pub base: Option<ClassName>,

    /// Class constants.
    pub constants: Vec<Constant>,

    // TODO: (doc coeffect constants)
    pub ctx_constants: Vec<CtxConstant>,

    /// Doc comment for the class.
    pub doc_comment: Option<Vec<u8>>,

    /// In an enum this is the enum_type:
    /// ```
    /// enum A: int as int
    ///                ^^^
    /// ```
    pub enum_type: Option<TypeInfo>,
    pub enum_includes: Vec<ClassName>,

    pub flags: Attr,

    /// The implemented interfaces.
    pub implements: Vec<ClassName>,

    /// The methods defined in this class.
    pub methods: Vec<Method>,

    pub name: ClassName,
    pub properties: Vec<Property>,
    pub requirements: Vec<Requirement>,
    pub span: Span,
    pub type_constants: Vec<TypeConstant>,

    /// For class generics the upper bounds of each generic.
    pub upper_bounds: Vec<UpperBound>,

    pub uses: Vec<ClassName>,
}

impl Class {
    pub fn get_prop_by_pid(&self, pid: PropName) -> Option<&Property> {
        self.properties.iter().find(|prop| prop.name == pid)
    }

    pub fn is_trait(&self) -> bool {
        (self.flags & Attr::AttrTrait) == Attr::AttrTrait
    }
}
