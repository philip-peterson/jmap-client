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

#[cfg(feature = "async")]
use jmap_client::{
    calendar::query::Filter as CalendarFilter,
    calendar_event::{
        query::Comparator as EventComparator,
        query::Filter as EventFilter,
        Alert, AlertAction, AlertTrigger, EventStatus, Participant, ParticipationStatus,
        RecurrenceFrequency, RecurrenceRule, TriggerRelation,
    },
    client::Client,
    core::query::Filter,
    core::set::SetObject
};

#[cfg(feature = "async")]
async fn calendars() {
    // Connect to the JMAP server using Basic authentication
    let client = Client::new()
        .credentials(("john@example.org", "secret"))
        .connect("https://jmap.example.org")
        .await
        .unwrap();

    // Create a calendar
    let calendar_id = client
        .calendar_create("Work", Some("#3b82f6"), true)
        .await
        .unwrap()
        .take_id();

    println!("Created calendar: {}", calendar_id);

    // Rename the calendar
    client
        .calendar_update_name(&calendar_id, "Work Calendar")
        .await
        .unwrap();

    // Query all subscribed calendars
    let calendar_ids = client
        .calendar_query(
            CalendarFilter::is_subscribed(true).into(),
            None::<Vec<_>>,
        )
        .await
        .unwrap()
        .take_ids();

    println!("Subscribed calendars: {:?}", calendar_ids);

    // Fetch the calendar's details
    let calendar = client
        .calendar_get(&calendar_id, None::<Vec<_>>)
        .await
        .unwrap()
        .unwrap();

    println!(
        "Calendar: {} (color: {:?})",
        calendar.name().unwrap_or(""),
        calendar.color()
    );

    // Create a simple all-day event
    let event_id = client
        .calendar_event_create(
            &calendar_id,
            "Team Standup",
            "2024-06-10T09:00:00",
            "PT30M",
            Some("America/New_York"),
        )
        .await
        .unwrap()
        .take_id();

    println!("Created event: {}", event_id);

    // Create an event with participants, a recurrence rule, and an alert
    // using the low-level request builder
    let mut request = client.build();
    let create = request.set_calendar_event().create();

    create
        .calendar_id(&calendar_id)
        .title("Weekly Sync")
        .description("Team weekly sync meeting")
        .start("2024-06-10T10:00:00")
        .duration("PT1H")
        .time_zone("America/New_York")
        .status(EventStatus::Confirmed)
        .recurrence_rule(RecurrenceRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: Some(1),
            count: Some(10),
            until: None,
            first_day_of_week: None,
            by_day: None,
            by_month_day: None,
            by_month: None,
            by_set_position: None,
            skip: None,
        })
        .participant(
            "organizer",
            Participant {
                name: Some("John Doe".to_string()),
                email: Some("john@example.org".to_string()),
                roles: [("owner".to_string(), true), ("attendee".to_string(), true)]
                    .into_iter()
                    .collect(),
                participation_status: Some(ParticipationStatus::Accepted),
                expect_reply: false,
                kind: None,
                description: None,
                send_to: None,
                language: None,
                location_id: None,
                schedule_status: None,
            },
        )
        .participant(
            "attendee1",
            Participant {
                name: Some("Jane Smith".to_string()),
                email: Some("jane@example.org".to_string()),
                roles: [("attendee".to_string(), true)].into_iter().collect(),
                participation_status: Some(ParticipationStatus::NeedsAction),
                expect_reply: true,
                kind: None,
                description: None,
                send_to: None,
                language: None,
                location_id: None,
                schedule_status: None,
            },
        )
        .alert(
            "reminder",
            Alert {
                trigger: AlertTrigger::OffsetTrigger {
                    offset: "-PT15M".to_string(),
                    relative_to: Some(TriggerRelation::Start),
                },
                acknowledged: None,
                action: AlertAction::Display,
            },
        );

    let recurring_id = create.create_id().unwrap();

    let recurring_event = request
        .send_single::<jmap_client::core::response::CalendarEventSetResponse>()
        .await
        .unwrap()
        .created(&recurring_id)
        .unwrap();

    println!(
        "Created recurring event: {} (uid: {:?})",
        recurring_event.id().unwrap_or(""),
        recurring_event.uid()
    );

    // Query events in the calendar, sorted by start date
    let event_ids = client
        .calendar_event_query(
            Filter::and([EventFilter::calendar_ids([&calendar_id])]).into(),
            [EventComparator::start()].into(),
        )
        .await
        .unwrap()
        .take_ids();

    println!("Events in calendar: {:?}", event_ids);

    // Fetch an event
    let event = client
        .calendar_event_get(&event_id, None::<Vec<_>>)
        .await
        .unwrap()
        .unwrap();

    println!(
        "Event: {} at {} (duration: {:?})",
        event.title().unwrap_or(""),
        event.start().map(String::as_str).unwrap_or(""),
        event.duration()
    );

    // Update the event title
    client
        .calendar_event_update_title(&event_id, "Team Standup (Updated)")
        .await
        .unwrap();

    // Delete the events
    client.calendar_event_destroy(&event_id).await.unwrap();
    client
        .calendar_event_destroy(recurring_event.id().unwrap())
        .await
        .unwrap();

    // Delete the calendar
    client.calendar_destroy(&calendar_id).await.unwrap();
}

fn main() {
    #[cfg(feature = "async")]
    let _c = calendars();
}
