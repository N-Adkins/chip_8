extern crate sdl2;

use sdl2::audio::AudioSpecDesired;
use sdl2::audio::AudioDevice;

pub struct Audio {
    pub device: AudioDevice<SquareWave>,
}

impl Audio {
    
    pub fn new(sdl_context: &sdl2::Sdl, frequency: i32, sample_rate: u16) -> Audio {
        
        let audio_subsystem = sdl_context.audio().unwrap();

        let want = AudioSpecDesired {
            freq: Some(frequency),
            channels: Some(1),
            samples: Some(sample_rate),
        };
        
        let device = audio_subsystem.open_playback(None, &want, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: 0.1,
            }
        }).unwrap();
        
        Audio {
            device,
        }

    }

}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl sdl2::audio::AudioCallback for SquareWave {
    
    type Channel = f32;

    fn callback(&mut self, out: &mut [Self::Channel]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }

}
