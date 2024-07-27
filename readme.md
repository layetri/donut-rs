# Donut 2.0
Donut is back, bigger and better than ever! I've decided to port the entire synthesis engine to Rust, implementing all the features I could previously only dream of. This is all backed by the research I've been doing over the past 2 years while working on [Mikoto Studio](https://mikoto.studio). This time around, the focus is to fully develop its potential as a stage synthesizer, keyboard and all.

### Updated feature list
- It's 100% more Rust now. Synthesis and most audio processing is rebuilt from the ground up in Rust, making it more reliable and work cross-platform without any external audio engine dependencies (looking at you JACK)
- I'm still doing the **everything synth**. Donut's 4/4/4 config will be back, featuring the 4 sound sources, 4 effects, and 4 modulators (for a start).
- Fully modular Audio Graph. Building on the implementation from Mikoto Studio, Donut 2.0 will feature a more reliable audio processing backend (maybe).
- User-friendly UI. I'm still on the fence about the exact framework to choose, but this iteration of Donut will finally feature a usable graphics interface (or Dear ImGui again).

### Credits
- The default piano sample was generously provided by Björn Colin.
