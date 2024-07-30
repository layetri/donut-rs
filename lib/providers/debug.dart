import 'package:donut/providers/dispatch.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class DebugInfo {
  String averageCycleTime;
  String maximumCycleTime;
  int blockSize;
  double sampleRate;
  String allowedCycleTime;

  DebugInfo({
    this.averageCycleTime = "0ms",
    this.maximumCycleTime = "0ms",
    this.blockSize = 0,
    this.sampleRate = 0,
    this.allowedCycleTime = "0ms",
  });
}

class DebugProvider extends StateNotifier<DebugInfo> {
  DebugProvider(super.state) : super() {
    Dispatcher.listen("debug", (data) {
      var s = DebugInfo(
        averageCycleTime: data["avg_cycle_time"],
        maximumCycleTime: data["max_cycle_time"],
        blockSize: data["block_size"],
        sampleRate: data["sample_rate"],
        allowedCycleTime: data["allowed_cycle_time"],
      );

      state = s;
    });
  }

  void updateDebugInfo(DebugInfo info) {
    state = info;
  }
}

final debugProvider = StateNotifierProvider<DebugProvider, DebugInfo>(
  (ref) => DebugProvider(DebugInfo()),
);