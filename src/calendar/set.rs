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

use super::{Calendar, CalendarRights, SetArguments};
use crate::{core::set::SetObject, Get, Set};
use ahash::AHashMap;

impl Calendar<Set> {
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.name = Some(name.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn color(&mut self, color: impl Into<String>) -> &mut Self {
        self.color = Some(color.into());
        self
    }

    pub fn is_subscribed(&mut self, is_subscribed: bool) -> &mut Self {
        self.is_subscribed = Some(is_subscribed);
        self
    }

    pub fn share_with(
        &mut self,
        account_id: impl Into<String>,
        rights: CalendarRights,
    ) -> &mut Self {
        self.share_with
            .get_or_insert_with(AHashMap::new)
            .insert(account_id.into(), rights);
        self
    }
}

impl SetObject for Calendar<Set> {
    type SetArguments = SetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        Calendar {
            _create_id,
            _state: Default::default(),
            id: None,
            name: None,
            description: None,
            color: None,
            is_subscribed: None,
            my_rights: None,
            share_with: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for Calendar<Get> {
    type SetArguments = SetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}

impl SetArguments {
    pub fn on_destroy_remove_events(&mut self, value: bool) -> &mut Self {
        self.on_destroy_remove_events = Some(value);
        self
    }
}
