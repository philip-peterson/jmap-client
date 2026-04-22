/*
 * Copyright Stalwart Labs LLC See the COPYING
 * file at the top-level directory of this distribution.
 *
 * Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
 * https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
 * <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
 * option. This file may not be copied, modified, or distributed
 * except according to those terms.
 */

pub mod get;
pub mod helpers;
pub mod query;
pub mod set;

use crate::core::changes::ChangesObject;
use crate::core::Object;
use crate::{Get, Set};
use ahash::AHashMap;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SetArguments {
    #[serde(rename = "onDestroyRemoveEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_destroy_remove_events: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryArguments {}

#[derive(Debug, Deserialize, Default)]
pub struct ChangesResponse {
    #[serde(rename = "updatedProperties")]
    updated_properties: Option<Vec<Property>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calendar<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,

    #[serde(rename = "isSubscribed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    is_subscribed: Option<bool>,

    #[serde(rename = "myRights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    my_rights: Option<CalendarRights>,

    #[serde(rename = "shareWith")]
    #[serde(skip_serializing_if = "Option::is_none")]
    share_with: Option<AHashMap<String, CalendarRights>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarRights {
    #[serde(rename = "mayReadFreeBusy")]
    #[serde(default)]
    pub may_read_free_busy: bool,

    #[serde(rename = "mayReadItems")]
    #[serde(default)]
    pub may_read_items: bool,

    #[serde(rename = "mayWriteAll")]
    #[serde(default)]
    pub may_write_all: bool,

    #[serde(rename = "mayWriteOwn")]
    #[serde(default)]
    pub may_write_own: bool,

    #[serde(rename = "mayUpdatePrivate")]
    #[serde(default)]
    pub may_update_private: bool,

    #[serde(rename = "mayRSVP")]
    #[serde(default)]
    pub may_rsvp: bool,

    #[serde(rename = "mayDelete")]
    #[serde(default)]
    pub may_delete: bool,

    #[serde(rename = "mayAdmin")]
    #[serde(default)]
    pub may_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "color")]
    Color,
    #[serde(rename = "isSubscribed")]
    IsSubscribed,
    #[serde(rename = "myRights")]
    MyRights,
    #[serde(rename = "shareWith")]
    ShareWith,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Name => write!(f, "name"),
            Property::Description => write!(f, "description"),
            Property::Color => write!(f, "color"),
            Property::IsSubscribed => write!(f, "isSubscribed"),
            Property::MyRights => write!(f, "myRights"),
            Property::ShareWith => write!(f, "shareWith"),
        }
    }
}

impl ChangesResponse {
    pub fn updated_properties(&self) -> Option<&[Property]> {
        self.updated_properties.as_deref()
    }
}

impl Object for Calendar<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for Calendar<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for Calendar<Set> {
    type ChangesResponse = ChangesResponse;
}

impl ChangesObject for Calendar<Get> {
    type ChangesResponse = ChangesResponse;
}
