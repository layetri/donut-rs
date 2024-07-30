import 'package:flutter_riverpod/flutter_riverpod.dart';

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
    
    note_on(key);
  }

  void lift(int key) {
    state = KeyboardState()..down = state.down.where((k) => k != key).toList();
    
    note_off(key);
  }
}

final keyboardProvider = StateNotifierProvider<KeyboardProvider, KeyboardState>((ref) {
  return KeyboardProvider();
});