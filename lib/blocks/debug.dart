import 'package:flutter/cupertino.dart';
import 'package:flutter/src/widgets/framework.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/debug.dart';

class DebugInfoComponent extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final debugInfo = ref.watch(debugProvider);
    return Container(
      width: 200,
      child: Column(
        children: [
          const Text("Debug info", style: TextStyle(
            fontSize: 20,
            fontWeight: FontWeight.bold
          )),

          Text("Average cycle time: ${debugInfo.averageCycleTime}"),
          Text("Maximum cycle time: ${debugInfo.maximumCycleTime}"),
          Text("Block size: ${debugInfo.blockSize}"),
          Text("Sample rate: ${debugInfo.sampleRate}"),
          Text("Allowed cycle time: ${debugInfo.allowedCycleTime}"),
        ],
      )
    );
  }
}