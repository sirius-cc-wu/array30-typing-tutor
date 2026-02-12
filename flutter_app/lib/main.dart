import 'dart:math';

import 'package:flutter/material.dart';
import 'package:google_fonts/google_fonts.dart';

import 'array30_data.dart';
import 'models.dart';
import 'storage.dart';

void main() {
  runApp(const Array30App());
}

class Array30App extends StatelessWidget {
  const Array30App({super.key});

  @override
  Widget build(BuildContext context) {
    final baseTextTheme = GoogleFonts.plusJakartaSansTextTheme();
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Array30 Typing Tutor',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(
          seedColor: _AppColors.brandPrimary,
          brightness: Brightness.light,
        ),
        textTheme: baseTextTheme,
        scaffoldBackgroundColor: Colors.transparent,
        useMaterial3: true,
      ),
      home: const Array30Home(),
    );
  }
}

class Array30Home extends StatefulWidget {
  const Array30Home({super.key});

  @override
  State<Array30Home> createState() => _Array30HomeState();
}

class _Array30HomeState extends State<Array30Home> with SingleTickerProviderStateMixin {
  late final TabController _tabController;
  final TextEditingController _inputController = TextEditingController();
  final FocusNode _inputFocus = FocusNode();

  PracticeSession _session = PracticeSession.newSession();
  DateTime? _startTime;
  bool _showCompletion = false;
  bool _loadingStats = false;
  Statistics _statistics = Statistics.empty();

  @override
  void initState() {
    super.initState();
    _tabController = TabController(length: 2, vsync: this);
    _loadStatistics();
  }

  @override
  void dispose() {
    _tabController.dispose();
    _inputController.dispose();
    _inputFocus.dispose();
    super.dispose();
  }

  Future<void> _loadStatistics() async {
    setState(() {
      _loadingStats = true;
    });
    final stats = await HistoryManager.getStatistics();
    if (!mounted) {
      return;
    }
    setState(() {
      _statistics = stats;
      _loadingStats = false;
    });
  }

  void _handleInput(String value) {
    setState(() {
      if (!_session.started) {
        _session.start();
        _startTime = DateTime.now();
      }

      final start = _startTime;
      final elapsedMs = start == null
          ? 0
          : DateTime.now().millisecondsSinceEpoch - start.millisecondsSinceEpoch;

      _session.updateInput(value, elapsedMs);
      _showCompletion = _isCompleted(value, _session.targetText);
    });
  }

  bool _isCompleted(String input, String target) {
    final inputChars = input.runes.toList();
    final targetChars = target.runes.toList();
    if (inputChars.length != targetChars.length) {
      return false;
    }
    for (var i = 0; i < inputChars.length; i++) {
      if (inputChars[i] != targetChars[i]) {
        return false;
      }
    }
    return true;
  }

  void _handleReset() {
    setState(() {
      _session = PracticeSession.newSession();
      _inputController.clear();
      _startTime = null;
      _showCompletion = false;
    });
    _inputFocus.requestFocus();
  }

  Future<void> _handleNext() async {
    if (_showCompletion) {
      await _saveCurrentSession();
      if (!mounted) {
        return;
      }
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('Session saved. Loading next challenge.'),
          backgroundColor: _AppColors.brandPrimary,
        ),
      );
    }

    setState(() {
      _session.nextExercise();
      _inputController.clear();
      _startTime = null;
      _showCompletion = false;
    });
    _inputFocus.requestFocus();
  }

  Future<void> _saveCurrentSession() async {
    final stats = _session.stats;
    final record = SessionRecord(
      wpm: stats.wpm,
      accuracy: stats.accuracy,
      timestamp: _formatTimestamp(DateTime.now()),
      elapsedSeconds: stats.elapsedSeconds,
      exerciseText: _session.targetText,
    );

    await HistoryManager.saveSession(record);
    await _loadStatistics();
  }

  Future<void> _confirmResetProgress() async {
    final shouldReset = await showDialog<bool>(
      context: context,
      builder: (context) {
        return AlertDialog(
          title: const Text('Reset all progress?'),
          content: const Text(
            'This will permanently remove all saved practice sessions and statistics.',
          ),
          actions: [
            TextButton(
              onPressed: () => Navigator.of(context).pop(false),
              child: const Text('Cancel'),
            ),
            TextButton(
              onPressed: () => Navigator.of(context).pop(true),
              child: const Text('Reset Everything'),
            ),
          ],
        );
      },
    );

    if (shouldReset == true) {
      await HistoryManager.clearHistory();
      if (!mounted) {
        return;
      }
      await _loadStatistics();
      setState(() {
        _session = PracticeSession.newSession();
        _inputController.clear();
        _startTime = null;
        _showCompletion = false;
      });
      if (!mounted) {
        return;
      }
      _tabController.index = 0;
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(
          content: Text('All progress has been reset.'),
          backgroundColor: _AppColors.brandWarm,
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    final targetChars = _session.targetText.runes.toList();
    final inputChars = _inputController.text.runes.toList();
    final progress = targetChars.isEmpty
        ? 0.0
        : min(1.0, inputChars.length / targetChars.length);
    final nextChar = inputChars.length < targetChars.length
        ? String.fromCharCode(targetChars[inputChars.length])
        : null;
    final nextCode = nextChar == null ? null : array30Codes[nextChar];

    return Container(
      decoration: const BoxDecoration(
        gradient: LinearGradient(
          begin: Alignment.topLeft,
          end: Alignment.bottomRight,
          colors: [
            Color(0xFFF5F3FF),
            Color(0xFFEFF6FF),
            Color(0xFFFFF7ED),
          ],
        ),
      ),
      child: SafeArea(
        child: Scaffold(
          backgroundColor: Colors.transparent,
          body: Center(
            child: Container(
              constraints: const BoxConstraints(maxWidth: 1200),
              margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 24),
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: _AppColors.surfaceBase,
                borderRadius: BorderRadius.circular(32),
                boxShadow: const [
                  BoxShadow(
                    color: Color(0x33000000),
                    offset: Offset(6, 6),
                    blurRadius: 0,
                  ),
                ],
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.stretch,
                children: [
                  _buildHeader(),
                  const SizedBox(height: 20),
                  Divider(color: _AppColors.brandPrimary.withOpacity(0.2), thickness: 2),
                  const SizedBox(height: 20),
                  TabBar(
                    controller: _tabController,
                    labelColor: _AppColors.contentStrong,
                    indicatorColor: _AppColors.brandPrimary,
                    labelStyle: const TextStyle(fontWeight: FontWeight.w700),
                    tabs: const [
                      Tab(text: 'Practice'),
                      Tab(text: 'Statistics'),
                    ],
                  ),
                  const SizedBox(height: 20),
                  Expanded(
                    child: TabBarView(
                      controller: _tabController,
                      children: [
                        _buildPracticeTab(progress, targetChars, inputChars, nextChar, nextCode),
                        _buildStatisticsTab(),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildHeader() {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      crossAxisAlignment: CrossAxisAlignment.end,
      children: [
        Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'Array30',
              style: GoogleFonts.plusJakartaSans(
                fontSize: 44,
                fontWeight: FontWeight.w800,
                color: _AppColors.contentStrong,
              ),
            ),
            const SizedBox(height: 6),
            Text(
              'Master the art of typing',
              style: TextStyle(
                fontSize: 16,
                fontWeight: FontWeight.w600,
                color: _AppColors.contentMuted,
              ),
            ),
          ],
        ),
        Container(
          padding: const EdgeInsets.symmetric(horizontal: 14, vertical: 8),
          decoration: BoxDecoration(
            color: _AppColors.brandAccent.withOpacity(0.15),
            borderRadius: BorderRadius.circular(999),
          ),
          child: const Text(
            'Array30 Coach',
            style: TextStyle(fontWeight: FontWeight.w700),
          ),
        ),
      ],
    );
  }

  Widget _buildPracticeTab(
    double progress,
    List<int> targetChars,
    List<int> inputChars,
    String? nextChar,
    String? nextCode,
  ) {
    final stats = _session.stats;

    return SingleChildScrollView(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Row(
            children: [
              Expanded(
                child: _MetricCard(label: 'WPM', value: stats.wpm.toStringAsFixed(0)),
              ),
              const SizedBox(width: 12),
              Expanded(
                child: _MetricCard(label: 'Accuracy', value: '${stats.accuracy.toStringAsFixed(0)}%'),
              ),
              const SizedBox(width: 12),
              const Expanded(
                child: _MetricCard(label: 'Level', value: '4/10'),
              ),
            ],
          ),
          const SizedBox(height: 20),
          _SurfaceCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          'Current Exercise',
                          style: TextStyle(
                            fontSize: 22,
                            fontWeight: FontWeight.w700,
                            color: _AppColors.contentStrong,
                          ),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          '${inputChars.length} / ${targetChars.length}',
                          style: TextStyle(color: _AppColors.contentMuted, fontWeight: FontWeight.w600),
                        ),
                      ],
                    ),
                    Row(
                      children: [
                        _Badge(
                          label: _showCompletion ? 'Ready to save' : 'In progress',
                          variant: _showCompletion ? BadgeVariant.primary : BadgeVariant.secondary,
                        ),
                        if (stats.errors > 0) ...[
                          const SizedBox(width: 8),
                          _Badge(
                            label: '${stats.errors} errors',
                            variant: BadgeVariant.destructive,
                          ),
                        ],
                      ],
                    ),
                  ],
                ),
                const SizedBox(height: 16),
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(16),
                    border: Border.all(color: _AppColors.brandPrimary.withOpacity(0.1), width: 2),
                  ),
                  child: RichText(
                    text: TextSpan(
                      style: GoogleFonts.plusJakartaSans(fontSize: 22, height: 1.5),
                      children: _buildTargetSpans(targetChars, inputChars),
                    ),
                  ),
                ),
                const SizedBox(height: 16),
                ClipRRect(
                  borderRadius: BorderRadius.circular(999),
                  child: LinearProgressIndicator(
                    value: progress,
                    minHeight: 10,
                    backgroundColor: _AppColors.brandPrimary.withOpacity(0.12),
                    valueColor: const AlwaysStoppedAnimation(_AppColors.brandPrimary),
                  ),
                ),
                const SizedBox(height: 16),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 12),
                  decoration: BoxDecoration(
                    color: Colors.white,
                    borderRadius: BorderRadius.circular(16),
                  ),
                  child: _buildCodeHint(nextChar, nextCode),
                ),
                const SizedBox(height: 16),
                TextField(
                  controller: _inputController,
                  focusNode: _inputFocus,
                  onChanged: _handleInput,
                  autofocus: true,
                  decoration: InputDecoration(
                    hintText: 'Focus here and start typing...',
                    filled: true,
                    fillColor: Colors.white,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                      borderSide: BorderSide(color: _AppColors.brandPrimary.withOpacity(0.2), width: 2),
                    ),
                    focusedBorder: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(16),
                      borderSide: const BorderSide(color: _AppColors.brandPrimary, width: 2),
                    ),
                  ),
                  style: const TextStyle(fontSize: 18, fontWeight: FontWeight.w600),
                ),
              ],
            ),
          ),
          const SizedBox(height: 20),
          Row(
            children: [
              Expanded(
                child: _AppButton(
                  label: _showCompletion ? 'Save & Next Lesson' : 'Skip to Next Lesson',
                  icon: Icons.play_arrow_rounded,
                  variant: ButtonVariant.primary,
                  onPressed: _handleNext,
                ),
              ),
              const SizedBox(width: 12),
              _AppIconButton(
                icon: Icons.refresh_rounded,
                onPressed: _handleReset,
              ),
            ],
          ),
          const SizedBox(height: 16),
          if (_session.started && !_showCompletion)
            Align(
              alignment: Alignment.centerLeft,
              child: _Badge(label: 'Recording session...', variant: BadgeVariant.secondary),
            ),
          if (_showCompletion) ...[
            const SizedBox(height: 12),
            Container(
              padding: const EdgeInsets.all(16),
              decoration: BoxDecoration(
                color: _AppColors.brandSecondary.withOpacity(0.2),
                borderRadius: BorderRadius.circular(16),
              ),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Excellent Accuracy!',
                    style: TextStyle(fontSize: 18, fontWeight: FontWeight.w700, color: _AppColors.contentStrong),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    'You\'ve mastered this exercise. Save your progress to continue.',
                    style: TextStyle(color: _AppColors.contentMuted),
                  ),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }

  List<TextSpan> _buildTargetSpans(List<int> target, List<int> input) {
    final spans = <TextSpan>[];
    for (var i = 0; i < target.length; i++) {
      final char = String.fromCharCode(target[i]);
      Color color;
      if (i < input.length) {
        color = input[i] == target[i] ? _AppColors.brandAccent : _AppColors.brandWarm;
      } else {
        color = _AppColors.contentMuted.withOpacity(0.6);
      }
      spans.add(TextSpan(text: char, style: TextStyle(color: color, fontWeight: FontWeight.w600)));
    }
    return spans;
  }

  Widget _buildCodeHint(String? nextChar, String? nextCode) {
    if (nextChar == null) {
      return Text(
        'Exercise Complete',
        textAlign: TextAlign.center,
        style: TextStyle(color: _AppColors.contentMuted, fontWeight: FontWeight.w700),
      );
    }

    if (nextCode == null) {
      return Row(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Row(
            children: [
              Text(
                'Next: ',
                style: TextStyle(fontWeight: FontWeight.w700, color: _AppColors.contentMuted),
              ),
              Text(
                nextChar,
                style: TextStyle(fontWeight: FontWeight.w800, color: _AppColors.contentStrong),
              ),
            ],
          ),
          _Badge(label: 'No code hint', variant: BadgeVariant.outline),
        ],
      );
    }

    final codes = nextCode.split('|');
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(
          'Next: Array30 Code for $nextChar',
          style: TextStyle(fontWeight: FontWeight.w700, color: _AppColors.contentStrong),
        ),
        Wrap(
          spacing: 6,
          children: codes
              .map(
                (code) => _Badge(
                  label: code,
                  variant: BadgeVariant.secondary,
                  fontFamily: GoogleFonts.firaCode().fontFamily,
                ),
              )
              .toList(),
        ),
      ],
    );
  }

  Widget _buildStatisticsTab() {
    if (_loadingStats) {
      return const Center(child: CircularProgressIndicator());
    }

    if (_statistics.totalSessions <= 0) {
      return _SurfaceCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              'No data yet',
              style: TextStyle(fontSize: 22, fontWeight: FontWeight.w700, color: _AppColors.contentStrong),
            ),
            const SizedBox(height: 8),
            Text(
              'Start your first practice session to see your typing statistics and track your progress over time.',
              style: TextStyle(color: _AppColors.contentMuted),
            ),
            const SizedBox(height: 16),
            _Badge(label: 'Waiting for first session', variant: BadgeVariant.outline),
          ],
        ),
      );
    }

    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Text(
          'Performance Overview',
          style: TextStyle(fontSize: 22, fontWeight: FontWeight.w700, color: _AppColors.contentStrong),
        ),
        const SizedBox(height: 16),
        Wrap(
          spacing: 12,
          runSpacing: 12,
          children: [
            _StatCard(
              label: 'Total Sessions',
              value: _statistics.totalSessions.toString(),
              subtext: 'Sessions completed',
            ),
            _StatCard(
              label: 'Best Speed',
              value: '${_statistics.bestWpm.toStringAsFixed(1)} WPM',
              subtext: 'Your all-time peak',
            ),
            _StatCard(
              label: 'Avg Speed',
              value: '${_statistics.averageWpm.toStringAsFixed(1)} WPM',
              subtext: 'Overall average',
            ),
            _StatCard(
              label: 'Max Accuracy',
              value: '${_statistics.bestAccuracy.toStringAsFixed(1)}%',
              subtext: 'Highest precision',
            ),
            _StatCard(
              label: 'Avg Accuracy',
              value: '${_statistics.averageAccuracy.toStringAsFixed(1)}%',
              subtext: 'Consistency score',
            ),
            _StatCard(
              label: 'Total Practice',
              value: _formatTime(_statistics.totalPracticeTime),
              subtext: 'Time on keys',
            ),
          ],
        ),
        const SizedBox(height: 20),
        Row(
          children: [
            _AppButton(
              label: 'Back to Practice',
              icon: Icons.arrow_back_rounded,
              variant: ButtonVariant.secondary,
              onPressed: () => _tabController.index = 0,
            ),
            const SizedBox(width: 12),
            _AppButton(
              label: 'Reset All Progress',
              icon: Icons.delete_outline_rounded,
              variant: ButtonVariant.primary,
              onPressed: _confirmResetProgress,
            ),
          ],
        ),
      ],
    );
  }
}

class _MetricCard extends StatelessWidget {
  const _MetricCard({required this.label, required this.value});

  final String label;
  final String value;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(16),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(20),
        border: Border.all(color: _AppColors.brandPrimary.withOpacity(0.12), width: 2),
      ),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(label, style: TextStyle(color: _AppColors.contentMuted, fontWeight: FontWeight.w600)),
          const SizedBox(height: 6),
          Text(value, style: TextStyle(fontSize: 24, fontWeight: FontWeight.w800, color: _AppColors.contentStrong)),
        ],
      ),
    );
  }
}

class _StatCard extends StatelessWidget {
  const _StatCard({required this.label, required this.value, required this.subtext});

  final String label;
  final String value;
  final String subtext;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 260,
      child: _SurfaceCard(
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            _Badge(label: label, variant: BadgeVariant.secondary),
            const SizedBox(height: 12),
            Text(value, style: TextStyle(fontSize: 22, fontWeight: FontWeight.w800, color: _AppColors.contentStrong)),
            const SizedBox(height: 4),
            Text(subtext, style: TextStyle(color: _AppColors.contentMuted)),
          ],
        ),
      ),
    );
  }
}

class _SurfaceCard extends StatelessWidget {
  const _SurfaceCard({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      padding: const EdgeInsets.all(20),
      decoration: BoxDecoration(
        color: Colors.white,
        borderRadius: BorderRadius.circular(24),
        border: Border.all(color: _AppColors.brandPrimary.withOpacity(0.12), width: 2),
        boxShadow: const [
          BoxShadow(
            color: Color(0x22000000),
            offset: Offset(4, 4),
            blurRadius: 0,
          ),
        ],
      ),
      child: child,
    );
  }
}

enum BadgeVariant { primary, secondary, outline, destructive }

enum ButtonVariant { primary, secondary }

class _Badge extends StatelessWidget {
  const _Badge({
    required this.label,
    required this.variant,
    this.fontFamily,
  });

  final String label;
  final BadgeVariant variant;
  final String? fontFamily;

  @override
  Widget build(BuildContext context) {
    final colors = _badgeColors(variant);
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
      decoration: BoxDecoration(
        color: colors.background,
        borderRadius: BorderRadius.circular(999),
        border: colors.border != null ? Border.all(color: colors.border!, width: 1.5) : null,
      ),
      child: Text(
        label,
        style: TextStyle(
          fontWeight: FontWeight.w700,
          fontSize: 12,
          color: colors.text,
          fontFamily: fontFamily,
        ),
      ),
    );
  }
}

class _BadgeColors {
  const _BadgeColors({required this.background, required this.text, this.border});

  final Color background;
  final Color text;
  final Color? border;
}

_BadgeColors _badgeColors(BadgeVariant variant) {
  switch (variant) {
    case BadgeVariant.primary:
      return const _BadgeColors(background: _AppColors.brandPrimary, text: Colors.white);
    case BadgeVariant.secondary:
      return _BadgeColors(background: _AppColors.brandSecondary.withOpacity(0.2), text: _AppColors.brandPrimary);
    case BadgeVariant.outline:
      return _BadgeColors(background: Colors.transparent, text: _AppColors.contentMuted, border: _AppColors.contentMuted);
    case BadgeVariant.destructive:
      return const _BadgeColors(background: _AppColors.brandWarm, text: Colors.white);
  }
}

class _AppButton extends StatelessWidget {
  const _AppButton({
    required this.label,
    required this.icon,
    required this.variant,
    required this.onPressed,
  });

  final String label;
  final IconData icon;
  final ButtonVariant variant;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    final colors = _buttonColors(variant);
    return ElevatedButton.icon(
      onPressed: onPressed,
      icon: Icon(icon, size: 20, color: colors.foreground),
      label: Text(label, style: TextStyle(color: colors.foreground, fontWeight: FontWeight.w700)),
      style: ElevatedButton.styleFrom(
        padding: const EdgeInsets.symmetric(horizontal: 18, vertical: 16),
        backgroundColor: colors.background,
        elevation: 0,
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      ),
    );
  }
}

class _AppIconButton extends StatelessWidget {
  const _AppIconButton({required this.icon, required this.onPressed});

  final IconData icon;
  final VoidCallback onPressed;

  @override
  Widget build(BuildContext context) {
    return IconButton(
      onPressed: onPressed,
      icon: Icon(icon, color: _AppColors.contentStrong),
      style: IconButton.styleFrom(
        backgroundColor: Colors.white,
        padding: const EdgeInsets.all(16),
        shape: RoundedRectangleBorder(borderRadius: BorderRadius.circular(16)),
      ),
    );
  }
}

class _ButtonColors {
  const _ButtonColors(this.background, this.foreground);

  final Color background;
  final Color foreground;
}

_ButtonColors _buttonColors(ButtonVariant variant) {
  switch (variant) {
    case ButtonVariant.primary:
      return const _ButtonColors(_AppColors.brandPrimary, Colors.white);
    case ButtonVariant.secondary:
      return _ButtonColors(_AppColors.brandSecondary.withOpacity(0.2), _AppColors.brandPrimary);
  }
}

String _formatTimestamp(DateTime dateTime) {
  final year = dateTime.year.toString().padLeft(4, '0');
  final month = dateTime.month.toString().padLeft(2, '0');
  final day = dateTime.day.toString().padLeft(2, '0');
  final hour = dateTime.hour.toString().padLeft(2, '0');
  final minute = dateTime.minute.toString().padLeft(2, '0');
  final second = dateTime.second.toString().padLeft(2, '0');
  return '$year-$month-$day $hour:$minute:$second';
}

String _formatTime(int seconds) {
  final hours = seconds ~/ 3600;
  final minutes = (seconds % 3600) ~/ 60;
  final secs = seconds % 60;
  if (hours > 0) {
    return '${hours}h ${minutes}m';
  }
  if (minutes > 0) {
    return '${minutes}m ${secs}s';
  }
  return '${secs}s';
}

class _AppColors {
  static const brandPrimary = Color(0xFF4848E5);
  static const brandSecondary = Color(0xFF818CF8);
  static const brandAccent = Color(0xFF0D9488);
  static const brandWarm = Color(0xFFF97316);
  static const surfaceBase = Color(0xFFEEF2FF);
  static const contentStrong = Color(0xFF1E1B4B);
  static const contentMuted = Color(0xFF6B7280);
}
