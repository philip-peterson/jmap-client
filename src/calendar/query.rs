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
use serde::Serialize;

use super::{Calendar, QueryArguments};

#[derive(Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Filter {
    Name {
        #[serde(rename = "name")]
        value: String,
    },
    IsSubscribed {
        #[serde(rename = "isSubscribed")]
        value: bool,
    },
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "property")]
pub enum Comparator {
    #[serde(rename = "name")]
    Name,
}

impl Filter {
    pub fn name(value: impl Into<String>) -> Self {
        Filter::Name { value: value.into() }
    }

    pub fn is_subscribed(value: bool) -> Self {
        Filter::IsSubscribed { value }
    }
}

impl Comparator {
    pub fn name() -> query::Comparator<Comparator> {
        query::Comparator::new(Comparator::Name)
    }
}

impl QueryObject for Calendar<Set> {
    type QueryArguments = QueryArguments;
    type Filter = Filter;
    type Sort = Comparator;
}
