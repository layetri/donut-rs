import 'package:donut/providers/synth.dart';
import 'package:donut/src/rust/system/parameter.dart';
import 'package:donut/ui/knob.dart';
import 'package:donut/ui/parameter.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class Saturator extends ConsumerWidget {
  const Saturator({Key? key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Container(
      padding: const EdgeInsets.all(20),
      margin: const EdgeInsets.all(10),
      color: Colors.grey.shade800,
      child: Column(children: [
        const Text("Saturator", style: TextStyle(color: Colors.white)),
        const SizedBox(height: 10),

        SizedBox(
          height: 60,
          child: Row(children: [
            Expanded(
              child: ParameterWidget(
                id: ParameterID.fxSaturatorAlpha,
                title: "Drive",
              ),
            ),
            Expanded(
              child: ParameterWidget(
                id: ParameterID.fxSaturatorAmount,
                title: "Amount",
              ),
            ),
          ],
          ),
        )
      ]),
    );
  }
}