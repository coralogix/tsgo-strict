// Copyright 2026 Coralogix Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
