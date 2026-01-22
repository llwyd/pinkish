use crate::audio_magic::*;

#[derive(Copy, Clone)]
pub struct Gain{
    gain:f32,
    prev:f32,
}

impl Gain{
    const GAIN_INCREMENT:f32 = 0.01;
    const GAIN_MAX:f32 = VOLUME_SLIDER_MAX; 
    const GAIN_INIT:f32 = 0.0;
    pub fn new() -> Gain{
        Gain{
            gain: Self::GAIN_INIT,
            prev: Self::GAIN_INIT,
        }
    }
    pub fn value(&self) -> f32{
        self.gain
    }

    pub fn ptr(&mut self) -> &mut f32{
        &mut self.gain
    }

    pub fn increment(&mut self)
    {
        self.gain += Self::GAIN_INCREMENT;
        if self.gain > Self::GAIN_MAX
        {
            self.gain = Self::GAIN_MAX;
        }
    }
    
    pub fn decrement(&mut self)
    {
        self.gain -= Self::GAIN_INCREMENT;
        if self.gain < 0.0
        {
            self.gain = 0.0;
        }
    }

#[allow(dead_code)]    
    pub fn silence(&mut self)
    {
        if self.gain > 0.0
        {
            self.prev = self.gain;
            self.gain = 0.0;
        }
    }
#[allow(dead_code)]    
    pub fn resume(&mut self)
    {
        self.gain = self.prev;
    }
}
