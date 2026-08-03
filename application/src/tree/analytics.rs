use automerge::ReadDoc;
use chrono::{Datelike, Timelike};

#[derive(Debug, Default, Copy, Clone)]
pub struct Snapshot {
    pub completed: u32,
    pub total: u32,
}

#[derive(Debug, Default)]
pub struct Analytics {
    timeline: std::collections::BTreeMap<chrono::NaiveDateTime, Snapshot>,
    last_heads: Vec<automerge::ChangeHash>,
}

#[derive(Debug, Copy, Clone)]
pub enum TimeScale {
    Hour,
    Day,
    Week,
    Month,
}

impl TimeScale {
    fn truncate(self, date_time: chrono::NaiveDateTime) -> Option<chrono::NaiveDateTime> {
        let date = date_time.date();
        match self {
            Self::Hour => date.and_hms_opt(date_time.hour(), 0, 0),
            Self::Day => date.and_hms_opt(0, 0, 0),
            Self::Week => {
                let days_from_monday = date_time.weekday().num_days_from_monday();
                date.checked_sub_signed(chrono::Duration::days(i64::from(days_from_monday)))
                    .and_then(|d| d.and_hms_opt(0, 0, 0))
            }
            Self::Month => chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1)
                .and_then(|d| d.and_hms_opt(0, 0, 0)),
        }
    }
}

impl Analytics {
    pub(super) fn new(document: &automerge::Automerge) -> Self {
        let mut analytics = Self::default();

        let start_time = document
            .get_changes(&[])
            .first()
            .and_then(|change| chrono::DateTime::from_timestamp_millis(change.timestamp()))
            .and_then(|dt| TimeScale::Hour.truncate(dt.naive_utc()));

        if let Some(start_time) = start_time {
            analytics.rebuild_from_time(document, start_time);
        }

        analytics
    }

    pub(super) fn update(&mut self, document: &automerge::Automerge) {
        let new_changes = document.get_changes(&self.last_heads);
        if new_changes.is_empty() {
            return;
        }

        let time_of_earliest_change = new_changes
            .iter()
            .filter_map(|change| chrono::DateTime::from_timestamp_millis(change.timestamp()))
            .filter_map(|date_time| TimeScale::Hour.truncate(date_time.naive_utc()))
            .min();

        if let Some(start_time) = time_of_earliest_change {
            self.rebuild_from_time(document, start_time);
        } else {
            self.last_heads = document.get_heads();
        }
    }

    fn rebuild_from_time(
        &mut self,
        document: &automerge::Automerge,
        start_time: chrono::NaiveDateTime,
    ) {
        let last_hash_per_hour: std::collections::BTreeMap<_, _> = document
            .get_changes(&[])
            .iter()
            .filter_map(|change| {
                let dt = chrono::DateTime::from_timestamp_millis(change.timestamp())?;
                let hour = TimeScale::Hour.truncate(dt.naive_utc())?;
                (hour >= start_time).then_some((hour, change.hash())) // <-- filter
            })
            .collect();

        self.timeline.split_off(&start_time);

        for (hour, hash) in last_hash_per_hour {
            if let Ok(forked) = document.fork_at(&[hash]) {
                let (completed, total) = Self::fast_sum_progress(&forked);
                self.timeline.insert(hour, Snapshot { completed, total });
            }
        }
        self.last_heads = document.get_heads();
    }

    fn fast_sum_progress(document: &automerge::Automerge) -> (u32, u32) {
        let mut completed = 0;
        let mut total = 0;
        let mut stack = vec![automerge::ObjId::Root];

        while let Some(id) = stack.pop() {
            if let Ok(Some((automerge::Value::Scalar(scalar), _))) =
                document.get(&id, super::NODE_TASK_COMPLETED)
                && let automerge::ScalarValue::Counter(counter) = scalar.as_ref()
            {
                let val = i64::from(counter).max(0);
                completed += u32::try_from(val).unwrap_or(0);
            }

            if let Ok(Some((automerge::Value::Scalar(scalar), _))) =
                document.get(&id, super::NODE_TASK_TOTAL)
            {
                total += match scalar.as_ref() {
                    automerge::ScalarValue::Uint(u) => u32::try_from(*u).unwrap_or(0),
                    automerge::ScalarValue::Int(i) => u32::try_from((*i).max(0)).unwrap_or(0),
                    _ => 0,
                };
            }

            if let Ok(Some((_, list_id))) = document.get(&id, super::CHILDREN) {
                stack.extend((0..document.length(&list_id)).filter_map(|idx| {
                    match document.get(&list_id, idx) {
                        Ok(Some((automerge::Value::Object(automerge::ObjType::Map), child_id))) => {
                            Some(child_id)
                        }
                        _ => None,
                    }
                }));
            }
        }

        (completed, total)
    }
}

impl Analytics {
    fn snapshots_by_scale(
        &self,
        scale: TimeScale,
    ) -> std::collections::BTreeMap<chrono::NaiveDateTime, Snapshot> {
        self.timeline
            .iter()
            .filter_map(|(date_time, snapshot)| {
                scale
                    .truncate(*date_time)
                    .map(|truncated| (truncated, *snapshot))
            })
            .collect()
    }

    #[must_use]
    pub fn completed_over_time(&self, scale: TimeScale) -> Vec<(chrono::NaiveDateTime, u32)> {
        self.snapshots_by_scale(scale)
            .into_iter()
            .map(|(date_time, snapshot)| (date_time, snapshot.completed))
            .collect()
    }

    #[must_use]
    pub fn total_over_time(&self, scale: TimeScale) -> Vec<(chrono::NaiveDateTime, u32)> {
        self.snapshots_by_scale(scale)
            .into_iter()
            .map(|(date_time, snapshot)| (date_time, snapshot.total))
            .collect()
    }

    #[must_use]
    pub fn change_over_time(&self, scale: TimeScale) -> Vec<(chrono::NaiveDateTime, i32)> {
        let mut previous_completed = 0;
        self.snapshots_by_scale(scale)
            .into_iter()
            .map(|(date_time, snapshot)| {
                let delta = snapshot.completed.cast_signed() - previous_completed;
                previous_completed = snapshot.completed.cast_signed();
                (date_time, delta)
            })
            .collect()
    }
}
