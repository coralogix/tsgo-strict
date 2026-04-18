use std::time::Instant;

#[derive(Debug, Clone)]
pub struct TimerEntry {
    pub label: String,
    pub duration_ms: u128,
}

#[derive(Default)]
pub struct Timer {
    starts: Vec<(String, Instant)>,
    entries: Vec<TimerEntry>,
}

impl Timer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_entries(entries: Vec<TimerEntry>) -> Self {
        Self {
            starts: Vec::new(),
            entries,
        }
    }

    pub fn start(&mut self, label: impl Into<String>) {
        self.starts.push((label.into(), Instant::now()));
    }

    pub fn end(&mut self, label: &str) {
        if let Some(idx) = self.starts.iter().rposition(|(name, _)| name == label) {
            let (name, start) = self.starts.remove(idx);
            let duration_ms = start.elapsed().as_millis();
            self.entries.push(TimerEntry {
                label: name,
                duration_ms,
            });
        }
    }

    pub fn entries(&self) -> &[TimerEntry] {
        &self.entries
    }
}
