import 'package:donut/src/rust/api/simple.dart';
import 'package:donut/src/rust/system/parameter.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class Parameter {
  ParameterID id;
  String name;
  double value;
  
  double min = 0.0;
  double max = 1.0;

  Parameter(this.id, this.name, this.value, {
    this.min = 0.0,
    this.max = 1.0
  });
}

class ParameterProvider extends StateNotifier<List<Parameter>> {
  ParameterProvider() : super([
    Parameter(ParameterID.ws1Detune, "Detune", 440.0, min: 350.0, max: 500.0),
    Parameter(ParameterID.ws1Harmonics, "Harmonics", 0.0, min: -1.0, max: 1.0),
    
    Parameter(ParameterID.wt1Shape, "Shape", 0.0, min: 0.0, max: 1.0),
    Parameter(ParameterID.wt1Detune, "Detune", 440.0, min: 350.0, max: 500.0),
    Parameter(ParameterID.wt1Transpose, "Transpose", 0.0, min: -12.0, max: 12.0),
    
    Parameter(ParameterID.adsr1Attack, "Attack", 20.0, min: 0.0, max: 1000.0),
    Parameter(ParameterID.adsr1Decay, "Decay", 20.0, min: 0.0, max: 1000.0),
    Parameter(ParameterID.adsr1Sustain, "Sustain", 0.8, min: 0.0, max: 1.0),
    Parameter(ParameterID.adsr1Release, "Release", 100.0, min: 0.0, max: 1000.0),
    
    Parameter(ParameterID.ksCutoff, "Cutoff", 10000.0, min: 1.0, max: 16000.0),
    Parameter(ParameterID.ksFeedback, "Feedback", 0.9999, min: 0.1, max: 0.99999999),
    
    Parameter(ParameterID.fxSaturatorAlpha, "Alpha", 0.0, min: 0.0, max: 1.0),
    
    Parameter(ParameterID.wt1Amount, "WaveTable 1", 0.0, min: 0.0, max: 1.0),
    Parameter(ParameterID.wt2Amount, "WaveTable 2", 0.0, min: 0.0, max: 1.0),
    Parameter(ParameterID.ws1Amount, "WaveShaper 1", 0.0, min: 0.0, max: 1.0),
    Parameter(ParameterID.ws2Amount, "WaveShaper 2", 0.0, min: 0.0, max: 1.0),
    Parameter(ParameterID.ksAmount, "Tensions", 1.0, min: 0.0, max: 1.0),
    
    Parameter(ParameterID.fxSaturatorAmount, "Saturator", 0.0, min: 0.0, max: 1.0),
  ]) {
    // Dispatcher.listen("parameters", (data) {
    //   final parameters = List<Map<String, dynamic>>.from(data["parameters"]);
    //   final _parameters = parameters.map((parameter) {
    //     return Parameter(
    //       ParameterID(parameter["id"]),
    //       parameter["name"],
    //       parameter["value"],
    //     );
    //   }).toList();
    //   state = _parameters;
    // });
  }

  void set(ParameterID id, double value) {
    final _state = state.map((parameter) {
      if (parameter.id == id) {
        return Parameter(id, parameter.name, value);
      }
      return parameter;
    }).toList();
    state = _state;

    setParameter(id: id, value: value);
  }
}

final parameterProvider = StateNotifierProvider<ParameterProvider, List<Parameter>>((ref) {
  return ParameterProvider();
});