mod chart;

use crate::tree::Tree;
use chrono::NaiveDate;

pub struct TasksChart {
    chart: chart::LineChart<NaiveDate, u32, 2>,
    last_heads: Vec<automerge::ChangeHash>,
}

impl TasksChart {
    pub fn new() -> Self {
        Self {
            // Start with empty arrays so it allocates nothing on startup
            chart: chart::LineChart::new([chart::line::Line::new([]), chart::line::Line::new([])]),
            last_heads: Vec::new(),
        }
    }

    pub fn sync(&mut self, tree: &Tree) {
        let current_heads = tree.document.get_heads();
        if self.last_heads == current_heads {
            return; // Cache hit, do nothing
        }

        let daily_progress = vec![(
            chrono::NaiveDate::from_epoch_days(12).unwrap(),
            crate::tree::node::Progress::default(),
        )];

        // Rebuild the chart with fresh data
        self.chart = chart::LineChart::new([
            chart::line::Line::new(daily_progress.iter().map(
                |&(date, ref prog): &(chrono::NaiveDate, crate::tree::node::Progress)| {
                    (date, prog.completed)
                },
            )),
            chart::line::Line::new(
                daily_progress
                    .iter()
                    .map(|&(date, ref prog)| (date, prog.total)),
            ),
        ]);

        self.last_heads = current_heads;
    }
}

pub struct WorkChart {
    chart: chart::LineChart<NaiveDate, f32, 1>,
    last_heads: Vec<automerge::ChangeHash>,
}

impl WorkChart {
    pub fn new() -> Self {
        Self {
            chart: chart::LineChart::new([chart::line::Line::new([])]),
            last_heads: Vec::new(),
        }
    }

    pub fn sync(&mut self, tree: &Tree) {
        let current_heads = tree.document.get_heads();
        if self.last_heads == current_heads {
            return;
        }

        let daily_progress: Vec<(chrono::NaiveDate, crate::tree::node::Progress)> = vec![];

        let mut work_series = Vec::with_capacity(daily_progress.len());
        let mut previous_completed = 0;

        for (date, progress) in daily_progress {
            let current_completed = progress.completed;
            let daily_work = (current_completed as f32) - (previous_completed as f32);
            work_series.push((date, daily_work));
            previous_completed = current_completed;
        }

        self.chart = chart::LineChart::new([chart::line::Line::new(work_series)]);
        self.last_heads = current_heads;
    }
}
