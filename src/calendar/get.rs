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

use super::{Calendar, CalendarRights};
use crate::{core::get::GetObject, Get, Set};
use ahash::AHashMap;

impl Calendar<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn color(&self) -> Option<&str> {
        self.color.as_deref()
    }

    pub fn is_subscribed(&self) -> bool {
        self.is_subscribed.unwrap_or(false)
    }

    pub fn my_rights(&self) -> Option<&CalendarRights> {
        self.my_rights.as_ref()
    }

    pub fn share_with(&self) -> Option<&AHashMap<String, CalendarRights>> {
        self.share_with.as_ref()
    }
}

impl GetObject for Calendar<Set> {
    type GetArguments = ();
}

impl GetObject for Calendar<Get> {
    type GetArguments = ();
}
