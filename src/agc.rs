#[derive(Clone,Debug)]
pub struct AGC{
    set_point:f32,
    gain:f32,
}

impl AGC{
    const DELTA_SCALE:f32 = 0.001;
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
        let delta = self.set_point - rms;
        self.gain *= 1.0 + (delta * Self::DELTA_SCALE);
    }
}
