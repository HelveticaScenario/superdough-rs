use web_audio_api::node::GainOptions;

use web_audio_api::node::GainNode;

use web_audio_api::context::AudioContext;

pub(crate) fn gain_node(audio_context: &AudioContext, value: f32) -> GainNode {
    let mut gain_options = GainOptions::default();
    gain_options.gain = value;
    GainNode::new(audio_context, gain_options)
}

pub(crate) fn get_slope(y1: f32, y2: f32, x1: f32, x2: f32) -> f32 {
    let denom = x2 - x1;
    if denom == 0.0 {
        return 0.0;
    }
    (y2 - y1) / (x2 - x1)
}

// pub(crate) fn get_worklet(audio_context: &AudioContext) {
//     let options = AudioWorkletNodeOptions::default();
//     let node = AudioWorkletNode::new(audio_context, options)
// }
