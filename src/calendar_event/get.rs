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

use super::{
    Alert, CalendarEvent, EventStatus, FreeBusyStatus, Location, LocalDateTime, Participant,
    Privacy, RecurrenceRule,
};
use crate::{core::get::GetObject, Get, Set};
use ahash::AHashMap;
use chrono::{DateTime, Utc};

impl CalendarEvent<Get> {
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn take_id(&mut self) -> String {
        self.id.take().unwrap_or_default()
    }

    pub fn uid(&self) -> Option<&str> {
        self.uid.as_deref()
    }

    pub fn calendar_ids(&self) -> Option<&AHashMap<String, bool>> {
        self.calendar_ids.as_ref()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn start(&self) -> Option<&LocalDateTime> {
        self.start.as_ref()
    }

    pub fn time_zone(&self) -> Option<&str> {
        self.time_zone.as_deref()
    }

    pub fn duration(&self) -> Option<&str> {
        self.duration.as_deref()
    }

    pub fn show_without_time(&self) -> bool {
        self.show_without_time.unwrap_or(false)
    }

    pub fn status(&self) -> Option<&EventStatus> {
        self.status.as_ref()
    }

    pub fn free_busy_status(&self) -> Option<&FreeBusyStatus> {
        self.free_busy_status.as_ref()
    }

    pub fn privacy(&self) -> Option<&Privacy> {
        self.privacy.as_ref()
    }

    pub fn sequence(&self) -> u32 {
        self.sequence.unwrap_or(0)
    }

    pub fn created(&self) -> Option<&DateTime<Utc>> {
        self.created.as_ref()
    }

    pub fn updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }

    pub fn participants(&self) -> Option<&AHashMap<String, Participant>> {
        self.participants.as_ref()
    }

    pub fn alerts(&self) -> Option<&AHashMap<String, Alert>> {
        self.alerts.as_ref()
    }

    pub fn recurrence_rules(&self) -> Option<&[RecurrenceRule]> {
        self.recurrence_rules.as_deref()
    }

    pub fn locations(&self) -> Option<&AHashMap<String, Location>> {
        self.locations.as_ref()
    }

    pub fn keywords(&self) -> Option<&AHashMap<String, bool>> {
        self.keywords.as_ref()
    }

    pub fn use_default_alerts(&self) -> bool {
        self.use_default_alerts.unwrap_or(false)
    }
}

impl GetObject for CalendarEvent<Set> {
    type GetArguments = ();
}

impl GetObject for CalendarEvent<Get> {
    type GetArguments = ();
}
