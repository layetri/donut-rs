import 'package:donut/modules/tensions.dart';
import 'package:donut/modules/waveshaper.dart';
import 'package:donut/modules/wavetable.dart';
import 'package:flutter/material.dart';

import '../modules/saturator.dart';

class Controls extends StatelessWidget {
  const Controls({Key? key}) : super(key: key);
  
  @override
  Widget build(BuildContext context) {
    return Container(
      height: 50,
      padding: const EdgeInsets.all(10),
      child: const Column(
        children: [
          Expanded(
            child: Row(
              children: [
                SizedBox(
                  width: 220,
                  child: WaveShaper()
                ),
                SizedBox(
                  width: 220,
                  child: Basic()
                ),
                SizedBox(
                  width: 220,
                  child: Tensions()
                ),

              ],
            )
          ),
          
          Expanded(
            child: Row(
              children: [
                Spacer(),
                SizedBox(
                  width: 180,
                  child: Saturator()
                )
              ],
            )
          )
        ],
      )
    );
  }
}