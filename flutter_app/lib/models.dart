import 'dart:convert';

class TypingStats {
  TypingStats({
    this.charactersTyped = 0,
    this.errors = 0,
    this.totalTyped = 0,
    this.elapsedSeconds = 0,
  });

  int charactersTyped;
  int errors;
  int totalTyped;
  int elapsedSeconds;

  double get wpm {
    if (elapsedSeconds <= 0) {
      return 0;
    }
    return (charactersTyped / 5) / (elapsedSeconds / 60);
  }

  double get accuracy {
    if (totalTyped <= 0) {
      return 100;
    }
    return ((totalTyped - errors) / totalTyped) * 100;
  }
}

class PracticeSession {
  PracticeSession({
    required this.targetText,
    required this.userInput,
    required this.stats,
    required this.started,
    required this.exerciseIndex,
  });

  factory PracticeSession.newSession() {
    return PracticeSession(
      targetText: _getExercise(0),
      userInput: '',
      stats: TypingStats(),
      started: false,
      exerciseIndex: 0,
    );
  }

  String targetText;
  String userInput;
  TypingStats stats;
  bool started;
  int exerciseIndex;

  void start() {
    started = true;
  }

  void updateInput(String input, int elapsedMs) {
    userInput = input;

    final targetChars = targetText.runes.toList();
    final inputChars = input.runes.toList();
    var correct = 0;
    for (var i = 0; i < inputChars.length; i++) {
      if (i < targetChars.length && inputChars[i] == targetChars[i]) {
        correct += 1;
      }
    }

    stats.totalTyped = inputChars.length;
    stats.charactersTyped = correct;
    stats.errors = stats.totalTyped - correct;
    stats.elapsedSeconds = elapsedMs ~/ 1000;
  }

  void nextExercise() {
    exerciseIndex += 1;
    targetText = _getExercise(exerciseIndex);
    userInput = '';
    stats = TypingStats();
    started = false;
  }

  static String _getExercise(int index) {
    const exercises = [
      'Array30是一個高效率的漢字輸入法',
      '熟能生巧，經過練習可以提高打字速度',
      '這個打字教練使用Rust和Dioxus開發',
      'Array30採用最優化的按鍵配置設計',
      '持續練習將幫助您改善打字的技能',
      '中文輸入法有很多不同的方式和系統',
      '提高打字速度需要長期的努力和堅持',
      'Array30提供快速和準確的輸入體驗',
    ];

    return exercises[index % exercises.length];
  }
}

class SessionRecord {
  SessionRecord({
    required this.wpm,
    required this.accuracy,
    required this.timestamp,
    required this.elapsedSeconds,
    required this.exerciseText,
  });

  final double wpm;
  final double accuracy;
  final String timestamp;
  final int elapsedSeconds;
  final String exerciseText;

  Map<String, dynamic> toJson() {
    return {
      'wpm': wpm,
      'accuracy': accuracy,
      'timestamp': timestamp,
      'elapsed_seconds': elapsedSeconds,
      'exercise_text': exerciseText,
    };
  }

  factory SessionRecord.fromJson(Map<String, dynamic> json) {
    return SessionRecord(
      wpm: (json['wpm'] as num).toDouble(),
      accuracy: (json['accuracy'] as num).toDouble(),
      timestamp: json['timestamp'] as String,
      elapsedSeconds: json['elapsed_seconds'] as int,
      exerciseText: json['exercise_text'] as String,
    );
  }

  static SessionRecord fromJsonString(String jsonStr) {
    return SessionRecord.fromJson(json.decode(jsonStr) as Map<String, dynamic>);
  }

  String toJsonString() => json.encode(toJson());
}

class Statistics {
  Statistics({
    required this.totalSessions,
    required this.bestWpm,
    required this.averageWpm,
    required this.bestAccuracy,
    required this.averageAccuracy,
    required this.totalPracticeTime,
  });

  factory Statistics.empty() {
    return Statistics(
      totalSessions: 0,
      bestWpm: 0,
      averageWpm: 0,
      bestAccuracy: 0,
      averageAccuracy: 0,
      totalPracticeTime: 0,
    );
  }

  final int totalSessions;
  final double bestWpm;
  final double averageWpm;
  final double bestAccuracy;
  final double averageAccuracy;
  final int totalPracticeTime;

  factory Statistics.fromRecords(List<SessionRecord> records) {
    if (records.isEmpty) {
      return Statistics.empty();
    }

    final totalSessions = records.length;
    final bestWpm = records.map((r) => r.wpm).reduce((a, b) => a > b ? a : b);
    final averageWpm = records.map((r) => r.wpm).reduce((a, b) => a + b) / totalSessions;
    final bestAccuracy = records.map((r) => r.accuracy).reduce((a, b) => a > b ? a : b);
    final averageAccuracy =
        records.map((r) => r.accuracy).reduce((a, b) => a + b) / totalSessions;
    final totalPracticeTime = records.map((r) => r.elapsedSeconds).reduce((a, b) => a + b);

    return Statistics(
      totalSessions: totalSessions,
      bestWpm: bestWpm,
      averageWpm: averageWpm,
      bestAccuracy: bestAccuracy,
      averageAccuracy: averageAccuracy,
      totalPracticeTime: totalPracticeTime,
    );
  }
}
