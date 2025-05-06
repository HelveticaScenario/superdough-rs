
use std::collections::HashMap;

use lazy_static::lazy_static;
use web_audio_api::context::{AudioContext, BaseAudioContext};

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

struct HAP {
    s: Option<u8>,
    bank: Option<u8>,
    source: Option<u8>,
    gain: Option<u8>,
    post_gain: Option<u8>,
    density: Option<u8>,
    stretch: Option<f64>,
    duration: f64,
    // filters
    f_anchor: Option<u8>,
    drive: Option<u8>,
    // low pass
    cutoff: Option<u8>,
    lpenv: Option<u8>,
    lp_attack: Option<u8>,
    lp_decay: Option<u8>,
    lp_sustain: Option<u8>,
    lp_release: Option<u8>,
    resonance: Option<u8>,
    // high pass
    hp_env: Option<u8>,
    h_cutoff: Option<u8>,
    hp_attack: Option<u8>,
    hp_decay: Option<u8>,
    hp_sustain: Option<u8>,
    hp_release: Option<u8>,
    h_resonance: Option<u8>,
    // band pass
    bp_env: Option<u8>,
    band_f: Option<u8>,
    bp_attack: Option<u8>,
    bp_decay: Option<u8>,
    bp_sustain: Option<u8>,
    bprelease: Option<u8>,
    band_q: Option<u8>,
    channels: Option<u8>,
    //phaser
    phaser_rate: Option<u8>,
    phaser_depth : Option<u8>,
    phaser_sweep: Option<u8>,
    phaser_center: Option<u8>,
    //
    coarse: Option<u8>,
    crush: Option<u8>,
    shape: Option<u8>,
    shape_vol : Option<u8>,
    distort: Option<u8>,
    distort_vol : Option<u8>,
    pan: Option<u8>,
    vowel: Option<u8>,
    delay: Option<u8>,
    delayfeedback: Option<u8>,
    delaytime : Option<u8>,
    orbit : Option<u8>,
    room: Option<u8>,
    room_fade: Option<u8>,
    room_lp: Option<u8>,
    room_dim: Option<u8>,
    room_size: Option<u8>,
    ir: Option<u8>,
    i : Option<u8>,
    velocity : Option<u8>,
    analyze: Option<u8>, // analyser wet
    fft : Option<u8>, // fftSize 0 - 10
    compressor: Option<u8>,
    compressor_ratio: Option<u8>,
    compressor_knee: Option<u8>,
    compressor_attack: Option<u8>,
    compressor_release: Option<u8>
}

enum Time {
    Absolute(f64),
    Relative(f64)
}

pub struct Superdough {
    audio_context: AudioContext,
    max_polyphony: usize,
    audio_device: String,
    sound_map: HashMap<String, String>
}

impl Superdough {
    fn new() -> Self {
        let audio_context = AudioContext::default();
        Superdough { 
            audio_context,
            max_polyphony: MAX_POLYPHONY,
            audio_device: DEFAULT_AUDIO_DEVICE_NAME.to_string(),
            sound_map: HashMap::new()
        }
    }

    fn fire(&self, value: &HAP, t: Time) {
        
        let mut time = match t {
            Time::Absolute(t) => t,
            Time::Relative(t) =>  self.audio_context.current_time() + t
        };

        if let Some(_) = value.stretch {
            //account for phase vocoder latency
            time -= 0.04;
        }

        if (time < self.audio_context.current_time()) {
            println!("[superdough]: cannot schedule sounds in the past (target: {}, now: {})",time,self.audio_context.current_time());
            return;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
       
        assert_eq!(2, 2);
    }
}
