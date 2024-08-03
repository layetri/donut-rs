import 'package:donut/providers/dispatch.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/simple.dart';

class MidiDeviceState {
  List<String> devices = [];
  String selected = "";

  MidiDeviceState clone() {
    return MidiDeviceState()
      ..devices = List<String>.from(devices)
      ..selected = selected;
  }
}

class MidiDeviceProvider extends StateNotifier<MidiDeviceState> {
  MidiDeviceProvider() : super(MidiDeviceState()) {
    Dispatcher.listen("midi.ports", (data) {
      final ports = List<String>.from(data["ports"]);

      state = MidiDeviceState()
        ..devices = ports
        ..selected = ports.isEmpty ? "" : ports.first;
    });
  }

  void addDevice(String device) {
    var _state = state.clone();
    _state.devices.add(device);
    state = _state;
  }

  void removeDevice(String device) {
    var _state = state.clone();
    _state.devices.remove(device);
    state = _state;
  }

  void setDevice(String device) {
    state = MidiDeviceState()
      ..devices = state.devices
      ..selected = device;

    setMidiInputPort(name: device);
  }
}

final midiDeviceProvider = StateNotifierProvider<MidiDeviceProvider, MidiDeviceState>((ref) {
  return MidiDeviceProvider();
});