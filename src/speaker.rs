use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};

pub struct Speaker{
    device: AudioDevice<SquareWave>
}

impl Speaker{
    pub fn new(sdl: sdl2::Sdl) -> Speaker{
        let audio_subsystem = sdl.audio().unwrap();
        let desired_spec = AudioSpecDesired{
            freq: Some(44100),
            channels: Some(1),
            samples: None
        };
        let dev = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave{
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.25
            }
        }).unwrap();
        Speaker{
            device: dev
        }
    }

    pub fn start(&self){
        self.device.resume();
    }
    pub fn stop(&self){
        self.device.pause();
    }
}


//from rust docs
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = match self.phase {
                0.0...0.5 => self.volume,
                _ => -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}