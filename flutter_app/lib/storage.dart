import 'package:shared_preferences/shared_preferences.dart';

import 'models.dart';

class HistoryManager {
  static const _sessionsKey = '_array30_sessions_v2';
  static const _legacyListKey = '_array30_sessions_list';

  static Future<void> saveSession(SessionRecord record) async {
    final prefs = await SharedPreferences.getInstance();

    final records = await _loadRecords(prefs);
    records.add(record);
    await _saveRecords(prefs, records);

    await _writeLegacyRecord(prefs, record);
  }

  static Future<Statistics> getStatistics() async {
    final prefs = await SharedPreferences.getInstance();
    final records = await _loadRecords(prefs);
    return Statistics.fromRecords(records);
  }

  static Future<void> clearHistory() async {
    final prefs = await SharedPreferences.getInstance();
    await prefs.remove(_sessionsKey);

    final legacyKeys = prefs.getStringList(_legacyListKey) ?? <String>[];
    for (final key in legacyKeys) {
      await prefs.remove(key);
    }
    await prefs.remove(_legacyListKey);
  }

  static Future<List<SessionRecord>> _loadRecords(SharedPreferences prefs) async {
    final stored = prefs.getStringList(_sessionsKey);
    if (stored != null && stored.isNotEmpty) {
      return stored.map(SessionRecord.fromJsonString).toList();
    }

    final legacy = _loadLegacyRecords(prefs);
    if (legacy.isNotEmpty) {
      await _saveRecords(prefs, legacy);
    }
    return legacy;
  }

  static Future<void> _saveRecords(SharedPreferences prefs, List<SessionRecord> records) async {
    final encoded = records.map((record) => record.toJsonString()).toList();
    await prefs.setStringList(_sessionsKey, encoded);
  }

  static List<SessionRecord> _loadLegacyRecords(SharedPreferences prefs) {
    final legacyKeys = prefs.getStringList(_legacyListKey) ?? <String>[];
    final records = <SessionRecord>[];
    for (final key in legacyKeys) {
      final jsonStr = prefs.getString(key);
      if (jsonStr == null) {
        continue;
      }
      try {
        records.add(SessionRecord.fromJsonString(jsonStr));
      } catch (_) {
        // Skip malformed legacy record.
      }
    }
    return records;
  }

  static Future<void> _writeLegacyRecord(SharedPreferences prefs, SessionRecord record) async {
    final timestamp = DateTime.now().millisecondsSinceEpoch;
    final key = 'session_$timestamp';
    await prefs.setString(key, record.toJsonString());

    final legacyKeys = prefs.getStringList(_legacyListKey) ?? <String>[];
    legacyKeys.add(key);
    await prefs.setStringList(_legacyListKey, legacyKeys);
  }
}
