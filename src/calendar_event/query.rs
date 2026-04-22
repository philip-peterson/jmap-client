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

use crate::{
    core::query::{self, QueryObject},
    Set,
};
use chrono::{DateTime, Utc};
use serde::Serialize;

use super::{CalendarEvent, LocalDateTime, QueryArguments};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    CalendarIds {
        #[serde(rename = "calendarIds")]
        value: Vec<String>,
    },
    Uid {
        #[serde(rename = "uid")]
        value: String,
    },
    Text {
        #[serde(rename = "text")]
        value: String,
    },
    Title {
        #[serde(rename = "title")]
        value: String,
    },
    Description {
        #[serde(rename = "description")]
        value: String,
    },
    Location {
        #[serde(rename = "location")]
        value: String,
    },
    HasKeyword {
        #[serde(rename = "hasKeyword")]
        value: String,
    },
    NotKeyword {
        #[serde(rename = "notKeyword")]
        value: String,
    },
    After {
        #[serde(rename = "after")]
        value: LocalDateTime,
    },
    Before {
        #[serde(rename = "before")]
        value: LocalDateTime,
    },
    AfterUtc {
        #[serde(rename = "after")]
        value: DateTime<Utc>,
    },
    BeforeUtc {
        #[serde(rename = "before")]
        value: DateTime<Utc>,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "start")]
    Start,
    #[serde(rename = "uid")]
    Uid,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "updated")]
    Updated,
}

impl Filter {
    pub fn calendar_ids(ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Filter::CalendarIds {
            value: ids.into_iter().map(Into::into).collect(),
        }
    }

    pub fn uid(value: impl Into<String>) -> Self {
        Filter::Uid { value: value.into() }
    }

    pub fn text(value: impl Into<String>) -> Self {
        Filter::Text { value: value.into() }
    }

    pub fn title(value: impl Into<String>) -> Self {
        Filter::Title { value: value.into() }
    }

    pub fn description(value: impl Into<String>) -> Self {
        Filter::Description { value: value.into() }
    }

    pub fn location(value: impl Into<String>) -> Self {
        Filter::Location { value: value.into() }
    }

    pub fn has_keyword(value: impl Into<String>) -> Self {
        Filter::HasKeyword { value: value.into() }
    }

    pub fn not_keyword(value: impl Into<String>) -> Self {
        Filter::NotKeyword { value: value.into() }
    }

    pub fn after(value: impl Into<LocalDateTime>) -> Self {
        Filter::After { value: value.into() }
    }

    pub fn before(value: impl Into<LocalDateTime>) -> Self {
        Filter::Before { value: value.into() }
    }

    pub fn after_utc(value: DateTime<Utc>) -> Self {
        Filter::AfterUtc { value }
    }

    pub fn before_utc(value: DateTime<Utc>) -> Self {
        Filter::BeforeUtc { value }
    }
}

impl Comparator {
    pub fn start() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Start)
    }

    pub fn uid() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Uid)
    }

    pub fn title() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Title)
    }

    pub fn updated() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Updated)
    }
}

impl QueryObject for CalendarEvent<Set> {
    type QueryArguments = QueryArguments;
    type Filter = Filter;
    type Sort = Comparator;
}
