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
    client::Client,
    core::{
        changes::{ChangesRequest, ChangesResponse},
        get::GetRequest,
        query::{Comparator, Filter, QueryRequest, QueryResponse},
        query_changes::{QueryChangesRequest, QueryChangesResponse},
        request::{Arguments, Request},
        response::{CalendarEventGetResponse, CalendarEventSetResponse},
        set::{SetObject, SetRequest},
    },
    Get, Method, Set, URI,
};

use super::{CalendarEvent, LocalDateTime, Property};

impl Client {
    #[maybe_async::maybe_async]
    pub async fn calendar_event_create(
        &self,
        calendar_id: impl Into<String>,
        title: impl Into<String>,
        start: impl Into<LocalDateTime>,
        duration: impl Into<String>,
        time_zone: Option<impl Into<String>>,
    ) -> crate::Result<CalendarEvent> {
        let mut request = self.build();
        let create = request.set_calendar_event().create();
        create
            .calendar_id(calendar_id)
            .title(title)
            .start(start)
            .duration(duration);
        if let Some(tz) = time_zone {
            create.time_zone(tz);
        }
        let id = create.create_id().unwrap();
        request
            .send_single::<CalendarEventSetResponse>()
            .await?
            .created(&id)
    }

    #[maybe_async::maybe_async]
    pub async fn calendar_event_get(
        &self,
        id: &str,
        properties: Option<impl IntoIterator<Item = Property>>,
    ) -> crate::Result<Option<CalendarEvent>> {
        let mut request = self.build();
        let get_request = request.get_calendar_event().ids([id]);
        if let Some(properties) = properties {
            get_request.properties(properties.into_iter());
        }
        request
            .send_single::<CalendarEventGetResponse>()
            .await
            .map(|mut r| r.take_list().pop())
    }

    #[maybe_async::maybe_async]
    pub async fn calendar_event_destroy(&self, id: &str) -> crate::Result<()> {
        let mut request = self.build();
        request.set_calendar_event().destroy([id]);
        request
            .send_single::<CalendarEventSetResponse>()
            .await?
            .destroyed(id)
    }

    #[maybe_async::maybe_async]
    pub async fn calendar_event_update_title(
        &self,
        id: &str,
        title: impl Into<String>,
    ) -> crate::Result<Option<CalendarEvent>> {
        let mut request = self.build();
        request.set_calendar_event().update(id).title(title);
        request
            .send_single::<CalendarEventSetResponse>()
            .await?
            .updated(id)
    }

    #[maybe_async::maybe_async]
    pub async fn calendar_event_query(
        &self,
        filter: Option<impl Into<Filter<super::query::Filter>>>,
        sort: Option<impl IntoIterator<Item = Comparator<super::query::Comparator>>>,
    ) -> crate::Result<QueryResponse> {
        let mut request = self.build();
        let query_request = request.query_calendar_event();
        if let Some(filter) = filter {
            query_request.filter(filter);
        }
        if let Some(sort) = sort {
            query_request.sort(sort.into_iter());
        }
        request.send_single::<QueryResponse>().await
    }

    #[maybe_async::maybe_async]
    pub async fn calendar_event_changes(
        &self,
        since_state: impl Into<String>,
        max_changes: usize,
    ) -> crate::Result<ChangesResponse<CalendarEvent<Get>>> {
        let mut request = self.build();
        request
            .changes_calendar_event(since_state)
            .max_changes(max_changes);
        request.send_single().await
    }
}

impl Request<'_> {
    pub fn get_calendar_event(&mut self) -> &mut GetRequest<CalendarEvent<Set>> {
        self.add_capability(URI::Calendars);
        self.add_method_call(
            Method::GetCalendarEvent,
            Arguments::calendar_event_get(self.params(Method::GetCalendarEvent)),
        )
        .calendar_event_get_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_get_calendar_event(self) -> crate::Result<CalendarEventGetResponse> {
        self.send_single().await
    }

    pub fn changes_calendar_event(
        &mut self,
        since_state: impl Into<String>,
    ) -> &mut ChangesRequest {
        self.add_capability(URI::Calendars);
        self.add_method_call(
            Method::ChangesCalendarEvent,
            Arguments::changes(
                self.params(Method::ChangesCalendarEvent),
                since_state.into(),
            ),
        )
        .changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_changes_calendar_event(
        self,
    ) -> crate::Result<ChangesResponse<CalendarEvent<Get>>> {
        self.send_single().await
    }

    pub fn query_calendar_event(&mut self) -> &mut QueryRequest<CalendarEvent<Set>> {
        self.add_capability(URI::Calendars);
        self.add_method_call(
            Method::QueryCalendarEvent,
            Arguments::calendar_event_query(self.params(Method::QueryCalendarEvent)),
        )
        .calendar_event_query_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_calendar_event(self) -> crate::Result<QueryResponse> {
        self.send_single().await
    }

    pub fn query_calendar_event_changes(
        &mut self,
        since_query_state: impl Into<String>,
    ) -> &mut QueryChangesRequest<CalendarEvent<Set>> {
        self.add_capability(URI::Calendars);
        self.add_method_call(
            Method::QueryChangesCalendarEvent,
            Arguments::calendar_event_query_changes(
                self.params(Method::QueryChangesCalendarEvent),
                since_query_state.into(),
            ),
        )
        .calendar_event_query_changes_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_query_calendar_event_changes(self) -> crate::Result<QueryChangesResponse> {
        self.send_single().await
    }

    pub fn set_calendar_event(&mut self) -> &mut SetRequest<CalendarEvent<Set>> {
        self.add_capability(URI::Calendars);
        self.add_method_call(
            Method::SetCalendarEvent,
            Arguments::calendar_event_set(self.params(Method::SetCalendarEvent)),
        )
        .calendar_event_set_mut()
    }

    #[maybe_async::maybe_async]
    pub async fn send_set_calendar_event(self) -> crate::Result<CalendarEventSetResponse> {
        self.send_single().await
    }
}
