import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api/simple.dart';

class KeyboardState {
  List<int> down = [];

  KeyboardState clone() {
    return KeyboardState()..down = List<int>.from(down);
  }
}

class KeyboardProvider extends StateNotifier<KeyboardState> {
  KeyboardProvider() : super(KeyboardState());

  void strike(int key) {
    state = KeyboardState()..down = [...state.down, key];
    
    noteOn(pitch: key, velocity: 1.0);
  }

  void lift(int key) {
    state = KeyboardState()..down = state.down.where((k) => k != key).toList();
    
    noteOff(pitch: key);
  }
}

final keyboardProvider = StateNotifierProvider<KeyboardProvider, KeyboardState>((ref) {
  return KeyboardProvider();
});