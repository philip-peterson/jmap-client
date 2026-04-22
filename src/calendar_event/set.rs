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
    Privacy, RecurrenceRule, SetArguments,
};
use crate::{core::set::SetObject, Get, Set};
use ahash::AHashMap;
use chrono::{DateTime, Utc};

impl CalendarEvent<Set> {
    pub fn uid(&mut self, uid: impl Into<String>) -> &mut Self {
        self.uid = Some(uid.into());
        self
    }

    pub fn calendar_id(&mut self, calendar_id: impl Into<String>) -> &mut Self {
        self.calendar_ids
            .get_or_insert_with(AHashMap::new)
            .insert(calendar_id.into(), true);
        self
    }

    pub fn title(&mut self, title: impl Into<String>) -> &mut Self {
        self.title = Some(title.into());
        self
    }

    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        self.description = Some(description.into());
        self
    }

    pub fn start(&mut self, start: impl Into<LocalDateTime>) -> &mut Self {
        self.start = Some(start.into());
        self
    }

    pub fn time_zone(&mut self, time_zone: impl Into<String>) -> &mut Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    pub fn duration(&mut self, duration: impl Into<String>) -> &mut Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn show_without_time(&mut self, show_without_time: bool) -> &mut Self {
        self.show_without_time = Some(show_without_time);
        self
    }

    pub fn status(&mut self, status: EventStatus) -> &mut Self {
        self.status = Some(status);
        self
    }

    pub fn free_busy_status(&mut self, free_busy_status: FreeBusyStatus) -> &mut Self {
        self.free_busy_status = Some(free_busy_status);
        self
    }

    pub fn privacy(&mut self, privacy: Privacy) -> &mut Self {
        self.privacy = Some(privacy);
        self
    }

    pub fn created(&mut self, created: DateTime<Utc>) -> &mut Self {
        self.created = Some(created);
        self
    }

    pub fn updated(&mut self, updated: DateTime<Utc>) -> &mut Self {
        self.updated = Some(updated);
        self
    }

    pub fn participant(
        &mut self,
        id: impl Into<String>,
        participant: Participant,
    ) -> &mut Self {
        self.participants
            .get_or_insert_with(AHashMap::new)
            .insert(id.into(), participant);
        self
    }

    pub fn alert(&mut self, id: impl Into<String>, alert: Alert) -> &mut Self {
        self.alerts
            .get_or_insert_with(AHashMap::new)
            .insert(id.into(), alert);
        self
    }

    pub fn recurrence_rule(&mut self, rule: RecurrenceRule) -> &mut Self {
        self.recurrence_rules.get_or_insert_with(Vec::new).push(rule);
        self
    }

    pub fn location(&mut self, id: impl Into<String>, location: Location) -> &mut Self {
        self.locations
            .get_or_insert_with(AHashMap::new)
            .insert(id.into(), location);
        self
    }

    pub fn keyword(&mut self, keyword: impl Into<String>) -> &mut Self {
        self.keywords
            .get_or_insert_with(AHashMap::new)
            .insert(keyword.into(), true);
        self
    }

    pub fn use_default_alerts(&mut self, use_default_alerts: bool) -> &mut Self {
        self.use_default_alerts = Some(use_default_alerts);
        self
    }

}

impl SetObject for CalendarEvent<Set> {
    type SetArguments = SetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        CalendarEvent {
            _create_id,
            _state: Default::default(),
            id: None,
            uid: None,
            calendar_ids: None,
            title: None,
            description: None,
            start: None,
            time_zone: None,
            duration: None,
            show_without_time: None,
            status: None,
            free_busy_status: None,
            privacy: None,
            sequence: None,
            created: None,
            updated: None,
            participants: None,
            alerts: None,
            recurrence_rules: None,
            locations: None,
            keywords: None,
            use_default_alerts: None,
        }
    }

    fn create_id(&self) -> Option<String> {
        self._create_id.map(|id| format!("c{}", id))
    }
}

impl SetObject for CalendarEvent<Get> {
    type SetArguments = SetArguments;

    fn new(_create_id: Option<usize>) -> Self {
        unimplemented!()
    }

    fn create_id(&self) -> Option<String> {
        None
    }
}
