use derive_builder::Builder;
use std::collections::HashMap;

use lazy_static::lazy_static;
use web_audio_api::{
    context::{AudioContext, BaseAudioContext},
    node::AudioNode,
};

mod helpers;

const DEFAULT_MAX_POLYPHONY: usize = 128;
const DEFAULT_AUDIO_DEVICE_NAME: &str = "System Standard";

// TODO: make this updatable
const MAX_POLYPHONY: usize = DEFAULT_MAX_POLYPHONY;

lazy_static! {
    static ref SOUND_MAP: HashMap<String, String> = HashMap::new();
}

/*
  let {
    s = getDefaultValue('s'),
    bank,
    source,
    gain = getDefaultValue('gain'),
    postgain = getDefaultValue('postgain'),
    density = getDefaultValue('density'),
    // filters
    fanchor = getDefaultValue('fanchor'),
    drive = 0.69,
    // low pass
    cutoff,
    lpenv,
    lpattack,
    lpdecay,
    lpsustain,
    lprelease,
    resonance = getDefaultValue('resonance'),
    // high pass
    hpenv,
    hcutoff,
    hpattack,
    hpdecay,
    hpsustain,
    hprelease,
    hresonance = getDefaultValue('hresonance'),
    // band pass
    bpenv,
    bandf,
    bpattack,
    bpdecay,
    bpsustain,
    bprelease,
    bandq = getDefaultValue('bandq'),
    channels = getDefaultValue('channels'),
    //phaser
    phaserrate: phaser,
    phaserdepth = getDefaultValue('phaserdepth'),
    phasersweep,
    phasercenter,
    //
    coarse,
    crush,
    shape,
    shapevol = getDefaultValue('shapevol'),
    distort,
    distortvol = getDefaultValue('distortvol'),
    pan,
    vowel,
    delay = getDefaultValue('delay'),
    delayfeedback = getDefaultValue('delayfeedback'),
    delaytime = getDefaultValue('delaytime'),
    orbit = getDefaultValue('orbit'),
    room,
    roomfade,
    roomlp,
    roomdim,
    roomsize,
    ir,
    i = getDefaultValue('i'),
    velocity = getDefaultValue('velocity'),
    analyze, // analyser wet
    fft = getDefaultValue('fft'), // fftSize 0 - 10
    compressor: compressorThreshold,
    compressorRatio,
    compressorKnee,
    compressorAttack,
    compressorRelease,
  } = value;


*/

#[derive(Default, Clone, Copy, Debug)]
enum FilterType {
    #[default]
    TwelveDB,
    Ladder,
    TwentyFourDB,
}

#[derive(Default, Builder, Debug)]
#[builder(setter(into), default, pattern = "owned")]
struct HAP {
    #[builder(default = "Some(\"triangle\".into())")]
    s: Option<String>,
    bank: Option<String>,
    #[builder(default = "Some(0.8)")]
    gain: Option<f32>,
    #[builder(default = "Some(1.0)")]
    post_gain: Option<f32>,
    #[builder(default = "Some(0.03)")]
    density: Option<f32>,
    stretch: Option<f32>,
    #[builder(default = "0.0")]
    duration: f64,
    // filters
    #[builder(default = "Some(0.0)")]
    f_anchor: Option<f32>,
    drive: Option<f32>,
    #[builder(default = "Some(FilterType::TwelveDB)")]
    ftype: Option<FilterType>,
    // low pass
    cutoff: Option<f32>,
    lpenv: Option<f32>,
    lp_attack: Option<f32>,
    lp_decay: Option<f32>,
    lp_sustain: Option<f32>,
    lp_release: Option<f32>,
    #[builder(default = "Some(1.0)")]
    resonance: Option<f32>,
    // high pass
    hp_env: Option<f32>,
    h_cutoff: Option<f32>,
    hp_attack: Option<f32>,
    hp_decay: Option<f32>,
    hp_sustain: Option<f32>,
    hp_release: Option<f32>,
    #[builder(default = "Some(1.0)")]
    h_resonance: Option<f32>,
    // band pass
    bp_env: Option<f32>,
    band_f: Option<f32>,
    bp_attack: Option<f32>,
    bp_decay: Option<f32>,
    bp_sustain: Option<f32>,
    bprelease: Option<f32>,
    #[builder(default = "Some(1.0)")]
    band_q: Option<f32>,
    #[builder(default = "Some(vec![1.0, 2.0])")]
    channels: Option<Vec<f32>>,
    //phaser
    phaser_rate: Option<f32>,
    #[builder(default = "Some(0.75)")]
    phaser_depth: Option<f32>,
    phaser_sweep: Option<f32>,
    phaser_center: Option<f32>,
    //
    coarse: Option<f32>,
    crush: Option<f32>,
    shape: Option<f32>,
    #[builder(default = "Some(1.0)")]
    shape_vol: Option<f32>,
    distort: Option<f32>,
    #[builder(default = "Some(1.0)")]
    distort_vol: Option<f32>,
    pan: Option<f32>,
    vowel: Option<f32>,
    #[builder(default = "Some(0.0)")]
    delay: Option<f32>,
    #[builder(default = "Some(0.5)")]
    delayfeedback: Option<f32>,
    #[builder(default = "Some(0.25)")]
    delaytime: Option<f32>,
    #[builder(default = "Some(1.0)")]
    orbit: Option<f32>,
    room: Option<f32>,
    room_fade: Option<f32>,
    room_lp: Option<f32>,
    room_dim: Option<f32>,
    room_size: Option<f32>,
    ir: Option<f32>,
    #[builder(default = "Some(1.0)")]
    i: Option<f32>,
    #[builder(default = "Some(1.0)")]
    velocity: Option<f32>,
    analyze: Option<f32>, // analyser wet
    #[builder(default = "Some(8.0)")]
    fft: Option<f32>, // fftSize 0 - 10
    compressor: Option<f32>,
    compressor_ratio: Option<f32>,
    compressor_knee: Option<f32>,
    compressor_attack: Option<f32>,
    compressor_release: Option<f32>,
}

#[derive(Clone, Copy, Debug)]
enum Time {
    Absolute(f64),
    Relative(f64),
}

#[derive(Debug)]

pub struct Superdough {
    audio_context: AudioContext,
    max_polyphony: usize,
    audio_device: String,
    sound_map: HashMap<String, String>,
}

impl Superdough {
    fn new() -> Self {
        let audio_context = AudioContext::default();
        Superdough {
            audio_context,
            max_polyphony: MAX_POLYPHONY,
            audio_device: DEFAULT_AUDIO_DEVICE_NAME.to_string(),
            sound_map: HashMap::new(),
        }
    }

    fn fire(&self, value: &HAP, t: Time) {
        self.fire_with_custom_source(value, t, None);
    }

    fn fire_with_custom_source(&self, value: &HAP, t: Time, source: Option<Box<dyn AudioNode>>) {
        let mut time = match t {
            Time::Absolute(t) => t,
            Time::Relative(t) => self.audio_context.current_time() + t,
        };

        if let Some(_) = value.stretch {
            //account for phase vocoder latency
            time -= 0.04;
        }

        if time < self.audio_context.current_time() {
            println!(
                "[superdough]: cannot schedule sounds in the past (target: {}, now: {})",
                time,
                self.audio_context.current_time()
            );
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HAPBuilder, Superdough, Time};

    #[test]
    fn it_works() {
        let superdough = Superdough::new();

        let hap = HAPBuilder::default()
            .s("sine".to_string())
            .gain(None)
            .post_gain(1.0)
            .density(0.03)
            .duration(1.0)
            .build()
            .unwrap();

        println!("{:?}", hap);

        superdough.fire(&hap, Time::Relative(0.4));
    }
}
