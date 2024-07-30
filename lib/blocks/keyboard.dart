import 'package:donut/providers/keyboard.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:donut/src/rust/api/simple.dart';
import 'package:donut/src/rust/frb_generated.dart';


class KeyboardPainter extends CustomPainter {
  int startFrom = 36;
  int endAt = 84;

  final List<int> activeKeys;

  KeyboardPainter({required this.activeKeys});

  @override
  void paint(Canvas canvas, Size size) {
    final nKeys = endAt - startFrom;
    final keyWidth = size.width / nKeys;
    final keyHeight = size.height;

    final linePaint = Paint()
      ..color = Colors.grey.shade800
      ..style = PaintingStyle.stroke
      ..strokeWidth = 1;

    final blackKeyPaint = Paint()
      ..color = Colors.black
      ..style = PaintingStyle.fill;

    final whiteKeyPaint = Paint()
      ..color = Colors.white
      ..style = PaintingStyle.fill;

    for (var i = startFrom; i < endAt; i++) {
      final keyRect = Rect.fromLTWH(
        (i - startFrom) * keyWidth,
        0,
        keyWidth,
        keyHeight
      );

      if (activeKeys.contains(i)) {
        canvas.drawRect(keyRect, Paint()..color = Colors.indigo);
      } else if ([1, 3, 6, 8, 10].contains(i % 12)) {
        canvas.drawRect(keyRect, blackKeyPaint);
      } else {
        canvas.drawRect(keyRect, whiteKeyPaint);
      }

      canvas.drawLine(
        Offset((i - startFrom) * keyWidth, 0),
        Offset((i - startFrom) * keyWidth, keyHeight),
        linePaint
      );
    }
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return false;
  }
}

class KeyboardComponent extends ConsumerStatefulWidget {
  @override
  _KeyboardComponentState createState() => _KeyboardComponentState();
}

class _KeyboardComponentState extends ConsumerState<KeyboardComponent> {
  List<int> keysDown = [];

  @override
  Widget build(BuildContext context) {
    return Container(
      child: Row(
        children: [
          Container(
            width: 50,
            color: Colors.black54,
          ),
          Expanded(
            child: GestureDetector(
              onTapDown: (details) {
                final keyWidth = (MediaQuery.of(context).size.width - 100) / 48;
                final key = (details.localPosition.dx / keyWidth).floor() + 36;

                ref.read(keyboardProvider.notifier).strike(key);
              },
              onTapUp: (details) {
                final keyWidth = (MediaQuery.of(context).size.width - 100) / 48;
                final key = (details.localPosition.dx / keyWidth).floor() + 36;

                ref.read(keyboardProvider.notifier).lift(key);
              },
              child: CustomPaint(
                painter: KeyboardPainter(
                  activeKeys: ref.watch(keyboardProvider).down
                ),
                child: const SizedBox(
                  height: 55
                ),
              ),
            ),
          ),
          Container(
            width: 50,
            color: Colors.black54,
          )
        ],
      ),
    );
  }
}