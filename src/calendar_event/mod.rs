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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

// LocalDateTime string in RFC 8984 format: "YYYY-MM-DDTHH:MM:SS" (no timezone offset)
pub type LocalDateTime = String;

// ISO 8601 duration string, e.g. "PT1H30M", "P1D"
pub type Duration = String;

#[derive(Debug, Clone, Serialize, Default)]
pub struct SetArguments {}

#[derive(Debug, Clone, Serialize, Default)]
pub struct QueryArguments {}

#[derive(Debug, Deserialize, Default)]
pub struct ChangesResponse {
    #[serde(rename = "updatedProperties")]
    updated_properties: Option<Vec<Property>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent<State = Get> {
    #[serde(skip)]
    _create_id: Option<usize>,

    #[serde(skip)]
    _state: std::marker::PhantomData<State>,

    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,

    #[serde(rename = "uid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<String>,

    #[serde(rename = "calendarIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_ids: Option<AHashMap<String, bool>>,

    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<LocalDateTime>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    time_zone: Option<String>,

    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Duration>,

    #[serde(rename = "showWithoutTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    show_without_time: Option<bool>,

    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<EventStatus>,

    #[serde(rename = "freeBusyStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    free_busy_status: Option<FreeBusyStatus>,

    #[serde(rename = "privacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    privacy: Option<Privacy>,

    #[serde(rename = "sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    sequence: Option<u32>,

    #[serde(rename = "created")]
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<DateTime<Utc>>,

    #[serde(rename = "updated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    updated: Option<DateTime<Utc>>,

    #[serde(rename = "participants")]
    #[serde(skip_serializing_if = "Option::is_none")]
    participants: Option<AHashMap<String, Participant>>,

    #[serde(rename = "alerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    alerts: Option<AHashMap<String, Alert>>,

    #[serde(rename = "recurrenceRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    recurrence_rules: Option<Vec<RecurrenceRule>>,

    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    locations: Option<AHashMap<String, Location>>,

    #[serde(rename = "keywords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<AHashMap<String, bool>>,

    #[serde(rename = "useDefaultAlerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    use_default_alerts: Option<bool>,
}

// --- Supporting types ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "kind")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<ParticipantKind>,

    #[serde(rename = "roles")]
    pub roles: AHashMap<String, bool>,

    #[serde(rename = "participationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participation_status: Option<ParticipationStatus>,

    #[serde(rename = "expectReply")]
    #[serde(default)]
    pub expect_reply: bool,

    #[serde(rename = "sendTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_to: Option<AHashMap<String, String>>,

    #[serde(rename = "language")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(rename = "locationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,

    #[serde(rename = "scheduleStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_status: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    #[serde(rename = "trigger")]
    pub trigger: AlertTrigger,

    #[serde(rename = "acknowledged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<DateTime<Utc>>,

    #[serde(rename = "action")]
    pub action: AlertAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "@type")]
pub enum AlertTrigger {
    OffsetTrigger {
        offset: String,
        #[serde(rename = "relativeTo")]
        #[serde(skip_serializing_if = "Option::is_none")]
        relative_to: Option<TriggerRelation>,
    },
    AbsoluteTrigger {
        when: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,

    #[serde(rename = "coordinates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coordinates: Option<String>,

    #[serde(rename = "linkIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_ids: Option<AHashMap<String, bool>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurrenceRule {
    #[serde(rename = "frequency")]
    pub frequency: RecurrenceFrequency,

    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,

    #[serde(rename = "firstDayOfWeek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_day_of_week: Option<String>,

    #[serde(rename = "byDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_day: Option<Vec<NDay>>,

    #[serde(rename = "byMonthDay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month_day: Option<Vec<i32>>,

    #[serde(rename = "byMonth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_month: Option<Vec<String>>,

    #[serde(rename = "bySetPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_set_position: Option<Vec<i32>>,

    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    #[serde(rename = "until")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<LocalDateTime>,

    #[serde(rename = "skip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<RecurrenceSkip>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NDay {
    #[serde(rename = "day")]
    pub day: String,

    #[serde(rename = "nthOfPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nth_of_period: Option<i32>,
}

// --- Enums ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EventStatus {
    Confirmed,
    Tentative,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FreeBusyStatus {
    Busy,
    Free,
    BusyUnavailable,
    BusyTentative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Privacy {
    Public,
    Private,
    Secret,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ParticipationStatus {
    NeedsAction,
    Accepted,
    Declined,
    Tentative,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ParticipantKind {
    Individual,
    Group,
    Resource,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AlertAction {
    Display,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecurrenceFrequency {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    Minutely,
    Secondly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecurrenceSkip {
    Omit,
    Backward,
    Forward,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TriggerRelation {
    Start,
    End,
}

// --- Property enum ---

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum Property {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "uid")]
    Uid,
    #[serde(rename = "calendarIds")]
    CalendarIds,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "timeZone")]
    TimeZone,
    #[serde(rename = "duration")]
    Duration,
    #[serde(rename = "showWithoutTime")]
    ShowWithoutTime,
    #[serde(rename = "status")]
    Status,
    #[serde(rename = "freeBusyStatus")]
    FreeBusyStatus,
    #[serde(rename = "privacy")]
    Privacy,
    #[serde(rename = "sequence")]
    Sequence,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "updated")]
    Updated,
    #[serde(rename = "participants")]
    Participants,
    #[serde(rename = "alerts")]
    Alerts,
    #[serde(rename = "recurrenceRules")]
    RecurrenceRules,
    #[serde(rename = "locations")]
    Locations,
    #[serde(rename = "keywords")]
    Keywords,
    #[serde(rename = "useDefaultAlerts")]
    UseDefaultAlerts,
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::Id => write!(f, "id"),
            Property::Uid => write!(f, "uid"),
            Property::CalendarIds => write!(f, "calendarIds"),
            Property::Title => write!(f, "title"),
            Property::Description => write!(f, "description"),
            Property::Start => write!(f, "start"),
            Property::TimeZone => write!(f, "timeZone"),
            Property::Duration => write!(f, "duration"),
            Property::ShowWithoutTime => write!(f, "showWithoutTime"),
            Property::Status => write!(f, "status"),
            Property::FreeBusyStatus => write!(f, "freeBusyStatus"),
            Property::Privacy => write!(f, "privacy"),
            Property::Sequence => write!(f, "sequence"),
            Property::Created => write!(f, "created"),
            Property::Updated => write!(f, "updated"),
            Property::Participants => write!(f, "participants"),
            Property::Alerts => write!(f, "alerts"),
            Property::RecurrenceRules => write!(f, "recurrenceRules"),
            Property::Locations => write!(f, "locations"),
            Property::Keywords => write!(f, "keywords"),
            Property::UseDefaultAlerts => write!(f, "useDefaultAlerts"),
        }
    }
}

impl ChangesResponse {
    pub fn updated_properties(&self) -> Option<&[Property]> {
        self.updated_properties.as_deref()
    }
}

impl Object for CalendarEvent<Set> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl Object for CalendarEvent<Get> {
    type Property = Property;

    fn requires_account_id() -> bool {
        true
    }
}

impl ChangesObject for CalendarEvent<Set> {
    type ChangesResponse = ChangesResponse;
}

impl ChangesObject for CalendarEvent<Get> {
    type ChangesResponse = ChangesResponse;
}
