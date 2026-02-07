use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct TypingStats {
    pub characters_typed: usize,
    pub errors: usize,
    pub total_typed: usize,
    pub elapsed_seconds: u64,
}

impl Default for TypingStats {
    fn default() -> Self {
        Self {
            characters_typed: 0,
            errors: 0,
            total_typed: 0,
            elapsed_seconds: 0,
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
        self.stats.total_typed = input.len();

        // Count correctly typed characters
        let target_chars: Vec<char> = self.target_text.chars().collect();
        let input_chars: Vec<char> = input.chars().collect();
        
        let mut correct = 0;
        for (i, &c) in input_chars.iter().enumerate() {
            if i < target_chars.len() && c == target_chars[i] {
                correct += 1;
            }
        }

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
        let exercises = vec![
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
