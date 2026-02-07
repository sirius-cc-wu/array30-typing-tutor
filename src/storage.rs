use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Storage;

    #[wasm_bindgen(js_namespace = window, js_name = localStorage)]
    static LOCAL_STORAGE: Storage;

    #[wasm_bindgen(method, getter)]
    fn length(this: &Storage) -> u32;

    #[wasm_bindgen(method)]
    fn getItem(this: &Storage, key: &str) -> Option<String>;

    #[wasm_bindgen(method, catch)]
    fn setItem(this: &Storage, key: &str, value: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    fn removeItem(this: &Storage, key: &str);

    #[wasm_bindgen(method)]
    fn clear(this: &Storage);
}

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
        if let Ok(json) = serde_json::to_string(&record) {
            let key = format!("session_{}", js_sys::Date::now());
            let _ = LOCAL_STORAGE.setItem(&key, &json);
            
            // Add to sessions list
            let mut sessions = Self::get_sessions_list();
            sessions.push(key);
            if let Ok(list_json) = serde_json::to_string(&sessions) {
                let _ = LOCAL_STORAGE.setItem(SESSIONS_LIST_KEY, &list_json);
            }
        }
    }

    fn get_sessions_list() -> Vec<String> {
        if let Some(data) = LOCAL_STORAGE.getItem(SESSIONS_LIST_KEY) {
            if let Ok(sessions) = serde_json::from_str::<Vec<String>>(&data) {
                return sessions;
            }
        }
        Vec::new()
    }

    pub fn get_statistics() -> Statistics {
        let mut all_records = Vec::new();
        let session_keys = Self::get_sessions_list();
        
        for key in session_keys {
            if let Some(data) = LOCAL_STORAGE.getItem(&key) {
                if let Ok(record) = serde_json::from_str::<SessionRecord>(&data) {
                    all_records.push(record);
                }
            }
        }

        Statistics::from_records(all_records)
    }

    pub fn clear_history() {
        let session_keys = Self::get_sessions_list();
        for key in session_keys {
            LOCAL_STORAGE.removeItem(&key);
        }
        LOCAL_STORAGE.removeItem(SESSIONS_LIST_KEY);
    }
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
        let average_accuracy = records.iter().map(|r| r.accuracy).sum::<f64>() / total_sessions as f64;
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
