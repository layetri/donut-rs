import 'dart:collection';
import 'dart:convert';

import 'package:donut/src/rust/api/ui.dart';
import 'package:flutter/foundation.dart';
import 'package:donut/src/rust/api/simple.dart';
import 'package:donut/src/rust/frb_generated.dart';

class Dispatcher {
  static final _eventCallbacks = HashMap<String, HashMap<int, Function(Map<String, dynamic>)>>();
  static int _nextCallbackId = 0;

  static void init() {
    runHandlerThread().listen((event) {
      final listeners = _eventCallbacks[event.destination];

      if (kDebugMode) {
        // print("Event: ${event.destination} ${event.content}");
      }

      // if(event.destination == "engine.phrase_progress") {
      //   print(event.content);
      // }

      if (listeners != null) {
        Map<String, dynamic> data = {};
        if (event.destination == "audio.samplerate") {
          data = {"samplerate": int.parse(event.content)};
        } else if (event.content.isNotEmpty) {
          data = jsonDecode(event.content);
        }

        for (final li in listeners.values) {
          li(data);
        }
      }
    });
  }

  // static void raise(EngineStatus status) {
  //   final listeners = _eventCallbacks["engine.error"];

  //   if (listeners != null) {
  //     Map<String, dynamic> data = status.toJson();

  //     for (final li in listeners.values) {
  //       li(data);
  //     }
  //   }
  // }

  static void removeListener(int id) {
    for (var map in _eventCallbacks.values) {
      if (map.containsKey(id)) {
        map.remove(id);
      }
    }
  }

  static int listen(String eventName, Function(Map<String, dynamic>) callback) {
    if (_eventCallbacks[eventName] == null) {
      _eventCallbacks[eventName] = HashMap();
    }

    final callbacks = _eventCallbacks[eventName]!;
    ++_nextCallbackId;
    final id = _nextCallbackId;
    callbacks[id] = callback;
    return id;
  }
}
