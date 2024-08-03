import 'package:donut/src/rust/system/parameter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/synth.dart';
import 'knob.dart';

class ParameterWidget extends ConsumerWidget {
  ParameterID id;
  String title;

  ParameterWidget({required this.id, required this.title});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final parameter = ref.watch(parameterProvider.select((value) => value.firstWhere((element) => element.id == id)));

    return Knob(
      value: parameter.value,
      min: parameter.min,
      max: parameter.max,
      size: 30,
      title: title,
      onChange: (double value) {
        ref.read(parameterProvider.notifier).set(id, value);
      },
    );
  }
}