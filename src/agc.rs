#[derive(Clone,Debug)]
pub struct AGC{
    set_point:f32,
    gain:f32,
}

impl AGC{
    // FIXME: Change to exponential decay that resets upon slider change
    // Logistic function
    const DELTA_SCALE:f32 = 0.001;
    const _EXP_SCALE:f32 = 0.001;
    const GAIN_MAX:f32 = 5.0;
    pub fn new(set_point:f32) -> AGC
    {
        AGC
        {
            set_point,
            gain: 0.1,
        }
    }

    pub fn gain(&self) -> f32
    {
        self.gain
    }

    pub fn update(&mut self, rms:f32)
    {
        //let delta = (self.set_point - rms)* Self::EXP_SCALE; 
        //self.gain *= 1.0 - (1.0 - delta.exp());
        //
        let delta = self.set_point - rms;
        self.gain *= 1.0 + (delta * Self::DELTA_SCALE);
        if self.gain > Self::GAIN_MAX
        {
            self.gain = Self::GAIN_MAX;
        }
    }
}
