import 'dart:math';

import 'package:flutter/material.dart';

const PI = 3.141592653589793;

enum KnobTrackType {
  Full,
  Mirrored,
  Split
}

class KnobTrack extends CustomPainter {
  final double value;
  final bool hover;
  final double min;
  final double max;
  final Color trackColor;
  final KnobTrackType trackType;

  KnobTrack({
    required this.value,
    required this.trackColor,
    required this.trackType,
    required this.hover,
    required this.min,
    required this.max
  });

  double maxTrackAngle = 150;
  double minTrackAngle = -150;

  // Function to calculate position+angle from a value
  Offset calculatePosition(double value, double size) {
    final angle = calculateAngle(value);
    final rad = angle * (PI / 180);
    final x = size / 2 * (1 + cos(rad));
    final y = size / 2 * (1 + sin(rad));

    return Offset(x, y);
  }

  double calculateAngle(double value) {
    final angle = (value - min) / (max - min) * (maxTrackAngle - minTrackAngle) +
        minTrackAngle;
    return angle;
  }

  @override
  void paint(Canvas canvas, Size size) {
    final trackPaint = Paint()
      ..color = Colors.black38
      ..strokeWidth = 2.0
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round;

    final fillPaint = Paint()
      ..color = trackColor
      ..strokeWidth = 2.0
      ..style = PaintingStyle.stroke
      ..strokeCap = StrokeCap.round;

    Offset center = Offset(size.width / 2.0, size.height / 2.0);

    canvas.drawArc(
      Rect.fromCenter(
        center: center,
        width: size.width,
        height: size.height,
      ),
      (maxTrackAngle - 90) * (PI / 180),
      (minTrackAngle - maxTrackAngle) * (PI / 180),
      false,
      trackPaint,
    );

    double start = 0.0;
    double sweep = 0.0;

    // Based on track type and value, calculate the start and sweep angles
    if(trackType == KnobTrackType.Full) {
      // 0.0 -> 0deg, 1.0 -> full track
      start = (minTrackAngle - 90) * (PI / 180);
      sweep = (value - min) / (max - min) * (maxTrackAngle - minTrackAngle) * (PI / 180);
    } else if(trackType == KnobTrackType.Mirrored) {
      // start = (maxTrackAngle - 90) * (PI / 180);
      // sweep = (minTrackAngle - maxTrackAngle) * (PI / 180) / 2;
    } else if(trackType == KnobTrackType.Split) {
      // middle of range = 0deg, 0.0 -> -90deg, 1.0 -> 90deg
      final middle = (max - min) / 2 + min;
      if(value > middle) {
        start = 270 * (PI / 180);
        sweep = (value - middle) / (max - middle) * maxTrackAngle * (PI / 180);
      } else {
        start = 270 * (PI / 180) - (middle - value) / (middle - min) * maxTrackAngle * (PI / 180);
        sweep = (middle - value) / (middle - min) * maxTrackAngle * (PI / 180);
      }
    }

    Path oval = Path()
      ..addArc(Rect.fromCenter(
        center: center,
        width: size.width,
        height: size.height,
      ), start, sweep
      );

    Paint shadowPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2
      ..color = trackColor.withOpacity(0.9)
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 1);
    canvas.drawPath(oval, shadowPaint);

    canvas.drawArc(
      Rect.fromCenter(
        center: center,
        width: size.width,
        height: size.height,
      ),
      start,
      sweep,
      false,
      fillPaint,
    );
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}

class KnobPainter extends CustomPainter {
  final double value;
  final bool hover;
  final double min;
  final double max;

  double maxTrackAngle = 150;
  double minTrackAngle = -150;

  KnobPainter(
      {required this.value,
        required this.hover,
        required this.min,
        required this.max});

  // Function to calculate position+angle from a value
  Offset calculatePosition(double value, double size) {
    final angle = calculateAngle(value);
    final rad = angle * (PI / 180);
    final x = size / 2 * (1 + cos(rad));
    final y = size / 2 * (1 + sin(rad));

    return Offset(x, y);
  }

  double calculateAngle(double value) {
    final angle = (value - min) / (max - min) * (maxTrackAngle - minTrackAngle) +
        minTrackAngle;
    return angle;
  }

  @override
  void paint(Canvas canvas, Size size) {
    final thumbPaint = Paint()
      ..color = Colors.white
      ..strokeWidth = 1.0
      ..style = PaintingStyle.fill;

    final knobPaint = Paint()
      ..color = Colors.black38
      ..strokeWidth = 1.0
      ..style = PaintingStyle.fill;

    var circlePath = Path();
    circlePath.addOval(Rect.fromCircle(
      center: Offset(size.width / 2.0, size.height / 2.0),
      radius: (size.width - 9.0) / 2.0,
    ));

    Paint shadowPaint = Paint()
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2
      ..color = Colors.black54
      ..maskFilter = const MaskFilter.blur(BlurStyle.normal, 1);
    canvas.drawPath(circlePath, shadowPaint);

    canvas.drawCircle(
      Offset(size.width / 2.0, size.height / 2.0),
      (size.width - 6.0) / 2.0,
      knobPaint,
    );

    final indicatorHeight = size.width / 2.0 - 6.0;
    final indicatorWidth = 2.0;

    canvas.drawRRect(
      RRect.fromRectAndRadius(
        Rect.fromLTWH(size.width / 2.0 - indicatorWidth / 2, 4.0, indicatorWidth, indicatorHeight),
        Radius.circular(indicatorWidth / 2),
      ),
      thumbPaint,
    );
  }

  @override
  bool shouldRepaint(covariant CustomPainter oldDelegate) {
    return true;
  }
}

class Knob extends StatefulWidget {
  final double value;
  String? title;
  double min = 0.0;
  double max = 1.0;
  double? snap;

  final Color? color;

  double size;
  KnobTrackType trackType;

  final void Function(double)? onChange;
  final void Function(double)? onFinish;

  final String Function(double)? format;
  final bool showValue;

  Knob(
      {Key? key,
        required this.value,
        this.title,
        this.showValue = false,
        this.onChange,
        this.onFinish,
        this.format,
        this.trackType = KnobTrackType.Full,
        this.min = 0.0,
        this.max = 1.0,
        this.snap,
        this.size = 80.0,
        this.color})
      : super(key: key);

  @override
  _KnobState createState() => _KnobState();
}

class _KnobState extends State<Knob> {
  bool hover = false;

  double maxTrackAngle = 150;
  double minTrackAngle = -150;

  double internalValue = 0.0;

  @override
  Widget build(BuildContext context) {
    return MouseRegion(
        onEnter: (event) {
          setState(() {
            hover = true;
          });
        },
        onExit: (event) {
          setState(() {
            hover = false;
          });
        },
        child: Column(
          mainAxisAlignment: widget.title == null ? MainAxisAlignment.center : MainAxisAlignment.start,
          children: [
            SizedBox(
              width: widget.size,
              height: widget.size,
              child: GestureDetector(
                onDoubleTap: () {
                  if(widget.snap != null) {
                    setState(() {
                      internalValue = widget.snap!;
                    });
                    widget.onChange?.call(widget.snap!);
                  } else {
                    setState(() {
                      internalValue = widget.min;
                    });
                    widget.onChange?.call(widget.min);
                  }
                },
                onVerticalDragUpdate: (details) {
                  final delta = -1.0 * (details.delta.dy / 50.0);
                  final newValue = widget.value + delta;
                  if (newValue >= widget.min && newValue <= widget.max) {
                    setState(() {
                      internalValue = internalValue + delta;
                    });

                    if(widget.snap != null && (internalValue - widget.snap!).abs() < 0.15) {
                      widget.onChange?.call(widget.snap!);
                    } else {
                      widget.onChange?.call(newValue);
                    }
                  }
                },
                onVerticalDragEnd: (details) {
                  widget.onFinish?.call(widget.value);
                },
                child: Stack(
                  children: [
                    CustomPaint(
                        painter: KnobTrack(
                            value: widget.value,
                            trackColor: widget.color ?? Colors.pinkAccent,
                            trackType: widget.trackType,
                            hover: hover,
                            min: widget.min,
                            max: widget.max),
                        child: SizedBox(
                          width: widget.size,
                          height: widget.size,
                        )
                    ),
                    Transform.rotate(
                      angle: ((widget.value - widget.min) / (widget.max - widget.min) * (maxTrackAngle - minTrackAngle) + minTrackAngle) * (PI / 180),
                      child: CustomPaint(
                        painter: KnobPainter(
                            value: widget.value,
                            hover: hover,
                            min: widget.min,
                            max: widget.max),
                        child: SizedBox(
                          width: widget.size,
                          height: widget.size,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ),
            if(widget.title != null)
              Expanded(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.start,
                  crossAxisAlignment: CrossAxisAlignment.center,
                  children: [
                    Text(
                      widget.title!,
                      style: TextStyle(color: Colors.grey.shade100, fontSize: 10.0),
                    ),
                    if(widget.showValue)
                      Text(
                        widget.format?.call(widget.value) ?? widget.value.toStringAsFixed(2),
                        style: const TextStyle(color: Colors.white, fontSize: 9.0, fontWeight: FontWeight.bold),
                      )
                  ],
                ),
              )
          ],
        ),
      );
  }
}