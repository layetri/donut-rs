// The original content is temporarily commented out to allow generating a self-contained demo - feel free to uncomment later.

import 'package:donut/blocks/controls.dart';
import 'package:donut/blocks/debug.dart';
import 'package:donut/blocks/keyboard.dart';
import 'package:donut/blocks/midi_select.dart';
import 'package:donut/providers/dispatch.dart';
import 'package:donut/providers/playback.dart';
import 'package:flutter/material.dart';
import 'package:donut/src/rust/api/simple.dart';
import 'package:donut/src/rust/frb_generated.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'package:google_fonts/google_fonts.dart';

Future<void> main() async {
  await RustLib.init();

  Dispatcher.init();
  runApp(const ProviderScope(child: Donut()));
}

class Donut extends ConsumerWidget {
  const Donut({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final position = ref.watch(playbackProvider).position;
    final musicalTime = MusicalTime.fromTicks(position);

    return MaterialApp(
      theme: _buildTheme(Brightness.dark),
      home: Scaffold(
        body: Column(
          children: [
            Container(
              height: 50,
              padding: const EdgeInsets.all(10),
              child: Row(
                children: [
                  const Spacer(),

                  const MidiDeviceSelector(),

                  const Spacer(),

                  SizedBox(
                    height: 100,
                    width: 300,
                    child: Row(
                      children: [
                        Text("Bar: ${musicalTime.bar}"),
                        const SizedBox(width: 20),
                        Text("Beat: ${musicalTime.beat}"),
                        const SizedBox(width: 20),
                        Text("Tick: ${musicalTime.tick}"),
                      ],
                    ),
                  ),

                  const Spacer(),

                  TextButton(
                    child: const Text("Play"),
                    onPressed: () {
                      play();
                    },
                  ),

                  const Spacer()
                ],
              ),
            ),
            const Expanded(
              child: Controls()
            ),

            const Text("Donut", style: TextStyle(fontSize: 26, fontWeight: FontWeight.w900, color: Colors.white)),
            Text("An instrument by ExpressiveLabs", style: TextStyle(fontSize: 12, color: Colors.grey.shade200)),

            const SizedBox(height: 20),

            SizedBox(
              height: 55,
              child: KeyboardComponent()
            )
          ],
        ),
      ),
    );
  }
}

ThemeData _buildTheme(brightness) {
  var baseTheme = ThemeData(brightness: brightness);

  return baseTheme.copyWith(
    textTheme: GoogleFonts.hindTextTheme(baseTheme.textTheme),
  );
}


