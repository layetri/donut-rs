import 'package:donut/providers/dispatch.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class PlaybackState {
  bool isPlaying = false;
  int position = 0;

  PlaybackState clone() {
    var value = PlaybackState();
    value.isPlaying = isPlaying;
    value.position = position;

    return value;
  }
}

class MusicalTime {
  int bar = 0;
  int beat = 0;
  int tick = 0;

  MusicalTime.fromTicks(int ticks) {
    bar = 1 + (ticks ~/ 192);
    beat = 1 + ((ticks % 192) ~/ 48);
    tick = (ticks % 192) % 48;
  }
}

class PlaybackProvider extends StateNotifier<PlaybackState> {
  PlaybackProvider(super.state) {
    Dispatcher.listen("engine.position", (data) {
      var _state = state.clone();
      _state.position = data["value"];
      state = _state;
    });
  }

  MusicalTime getMusicalPosition() {
    final position = state.position;

    return MusicalTime.fromTicks(position);
  }
}

final playbackProvider = StateNotifierProvider<PlaybackProvider, PlaybackState>(
  (ref) => PlaybackProvider(PlaybackState()),
);

