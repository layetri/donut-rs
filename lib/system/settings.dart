//
// import 'package:json_annotation/json_annotation.dart';
//
// @JsonSerializable()
// class Settings {
//   String midiDevice = "";
//   String audioDevice = "";
//  
//   Settings();
//  
//   static Future<void> set(String key, dynamic value) async {
//     final settings = await get();
//     settings[key] = value;
//     await _storage.write(key: "settings", value: settings);
//   }
// }