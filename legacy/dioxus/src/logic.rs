use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct TypingStats {
    pub characters_typed: usize,
    pub errors: usize,
    pub total_typed: usize,
    pub elapsed_seconds: u64,
}

impl TypingStats {
    pub fn wpm(&self) -> f64 {
        if self.elapsed_seconds > 0 {
            (self.characters_typed as f64 / 5.0) / (self.elapsed_seconds as f64 / 60.0)
        } else {
            0.0
        }
    }

    pub fn accuracy(&self) -> f64 {
        if self.total_typed > 0 {
            ((self.total_typed - self.errors) as f64 / self.total_typed as f64) * 100.0
        } else {
            100.0
        }
    }
}

#[derive(Clone)]
pub struct PracticeSession {
    pub target_text: String,
    pub user_input: String,
    pub stats: TypingStats,
    pub started: bool,
    pub exercise_index: usize,
}

impl PracticeSession {
    pub fn new() -> Self {
        Self {
            target_text: Self::get_exercise(0),
            user_input: String::new(),
            stats: TypingStats::default(),
            started: false,
            exercise_index: 0,
        }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn update_input(&mut self, input: &str, elapsed_ms: u64) {
        self.user_input = input.to_string();

        // Count correctly typed characters
        let target_chars: Vec<char> = self.target_text.chars().collect();
        let input_chars: Vec<char> = input.chars().collect();

        let mut correct = 0;
        for (i, &c) in input_chars.iter().enumerate() {
            if i < target_chars.len() && c == target_chars[i] {
                correct += 1;
            }
        }

        self.stats.total_typed = input_chars.len(); // Use character count, not byte length
        self.stats.characters_typed = correct;
        self.stats.errors = self.stats.total_typed.saturating_sub(correct);
        self.stats.elapsed_seconds = elapsed_ms / 1000;
    }

    pub fn next_exercise(&mut self) {
        self.exercise_index += 1;
        self.target_text = Self::get_exercise(self.exercise_index);
        self.user_input.clear();
        self.stats = TypingStats::default();
        self.started = false;
    }

    fn get_exercise(index: usize) -> String {
        let exercises = [
            "Array30是一個高效率的漢字輸入法",
            "熟能生巧，經過練習可以提高打字速度",
            "這個打字教練使用Rust和Dioxus開發",
            "Array30採用最優化的按鍵配置設計",
            "持續練習將幫助您改善打字的技能",
            "中文輸入法有很多不同的方式和系統",
            "提高打字速度需要長期的努力和堅持",
            "Array30提供快速和準確的輸入體驗",
        ];

        exercises[index % exercises.len()].to_string()
    }
}

impl Default for PracticeSession {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::PracticeSession;

    #[test]
    fn update_input_counts_cjk_characters_not_bytes() {
        let mut session = PracticeSession::new();
        session.target_text = "漢字測試".to_string();

        session.update_input("漢字x試", 2500);

        assert_eq!(session.stats.total_typed, 4);
        assert_eq!(session.stats.characters_typed, 3);
        assert_eq!(session.stats.errors, 1);
        assert_eq!(session.stats.elapsed_seconds, 2);
    }

    #[test]
    fn next_exercise_resets_session_state() {
        let mut session = PracticeSession::new();
        session.start();
        session.update_input("abc", 3000);

        session.next_exercise();

        assert!(!session.started);
        assert_eq!(session.user_input, "");
        assert_eq!(session.stats.total_typed, 0);
        assert_eq!(session.stats.characters_typed, 0);
        assert_eq!(session.stats.errors, 0);
        assert_eq!(session.stats.elapsed_seconds, 0);
    }
}
