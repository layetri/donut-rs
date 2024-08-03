import 'package:donut/providers/midi.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class MidiDeviceSelector extends ConsumerWidget {
  const MidiDeviceSelector({Key? key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final data = ref.watch(midiDeviceProvider);

    return DropdownButton<String>(
      value: data.devices.isEmpty ? null : data.selected,
      onChanged: (String? device) {
        if (device != null) {
          // select(device);
          ref.read(midiDeviceProvider.notifier).setDevice(device);
        }
      },
      items: data.devices.map((String device) {
        return DropdownMenuItem<String>(
          value: device,
          child: Text(device),
        );
      }).toList(),
    );
  }
}