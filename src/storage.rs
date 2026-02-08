use serde::{Deserialize, Serialize};
use web_sys::Storage;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct SessionRecord {
    pub wpm: f64,
    pub accuracy: f64,
    pub timestamp: String,
    pub elapsed_seconds: u64,
    pub exercise_text: String,
}

pub struct HistoryManager;

const SESSIONS_LIST_KEY: &str = "_array30_sessions_list";

impl HistoryManager {
    pub fn save_session(record: SessionRecord) {
        if let Some(storage) = local_storage() {
            if let Ok(json) = serde_json::to_string(&record) {
                let key = format!("session_{}", js_sys::Date::now());
                let _ = storage.set_item(&key, &json);

                // Add to sessions list
                let mut sessions = Self::get_sessions_list();
                sessions.push(key);
                if let Ok(list_json) = serde_json::to_string(&sessions) {
                    let _ = storage.set_item(SESSIONS_LIST_KEY, &list_json);
                }
            }
        }
    }

    fn get_sessions_list() -> Vec<String> {
        if let Some(storage) = local_storage() {
            if let Ok(Some(data)) = storage.get_item(SESSIONS_LIST_KEY) {
                if let Ok(sessions) = serde_json::from_str::<Vec<String>>(&data) {
                    return sessions;
                }
            }
        }
        Vec::new()
    }

    pub fn get_statistics() -> Statistics {
        let mut all_records = Vec::new();
        let session_keys = Self::get_sessions_list();

        if let Some(storage) = local_storage() {
            for key in session_keys {
                if let Ok(Some(data)) = storage.get_item(&key) {
                    if let Ok(record) = serde_json::from_str::<SessionRecord>(&data) {
                        all_records.push(record);
                    }
                }
            }
        }

        Statistics::from_records(all_records)
    }

    pub fn clear_history() {
        let session_keys = Self::get_sessions_list();
        if let Some(storage) = local_storage() {
            for key in session_keys {
                let _ = storage.remove_item(&key);
            }
            let _ = storage.remove_item(SESSIONS_LIST_KEY);
        }
    }
}

fn local_storage() -> Option<Storage> {
    web_sys::window().and_then(|window| window.local_storage().ok().flatten())
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Statistics {
    pub total_sessions: usize,
    pub best_wpm: f64,
    pub average_wpm: f64,
    pub best_accuracy: f64,
    pub average_accuracy: f64,
    pub total_practice_time: u64,
}

impl Statistics {
    pub fn from_records(records: Vec<SessionRecord>) -> Self {
        if records.is_empty() {
            return Statistics::default();
        }

        let total_sessions = records.len();
        let best_wpm = records.iter().map(|r| r.wpm).fold(0.0, f64::max);
        let average_wpm = records.iter().map(|r| r.wpm).sum::<f64>() / total_sessions as f64;
        let best_accuracy = records.iter().map(|r| r.accuracy).fold(0.0, f64::max);
        let average_accuracy =
            records.iter().map(|r| r.accuracy).sum::<f64>() / total_sessions as f64;
        let total_practice_time = records.iter().map(|r| r.elapsed_seconds).sum();

        Statistics {
            total_sessions,
            best_wpm,
            average_wpm,
            best_accuracy,
            average_accuracy,
            total_practice_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SessionRecord, Statistics};

    #[test]
    fn statistics_empty_records_returns_default() {
        let stats = Statistics::from_records(vec![]);
        assert_eq!(stats, Statistics::default());
    }

    #[test]
    fn statistics_from_records_aggregates_fields() {
        let records = vec![
            SessionRecord {
                wpm: 20.0,
                accuracy: 95.0,
                timestamp: "2026-01-01 10:00:00".to_string(),
                elapsed_seconds: 30,
                exercise_text: "練習一".to_string(),
            },
            SessionRecord {
                wpm: 40.0,
                accuracy: 90.0,
                timestamp: "2026-01-01 10:05:00".to_string(),
                elapsed_seconds: 45,
                exercise_text: "練習二".to_string(),
            },
        ];

        let stats = Statistics::from_records(records);

        assert_eq!(stats.total_sessions, 2);
        assert_eq!(stats.best_wpm, 40.0);
        assert_eq!(stats.best_accuracy, 95.0);
        assert_eq!(stats.total_practice_time, 75);
        assert!((stats.average_wpm - 30.0).abs() < f64::EPSILON);
        assert!((stats.average_accuracy - 92.5).abs() < f64::EPSILON);
    }
}
