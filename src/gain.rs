
#[derive(Copy, Clone)]
pub struct Gain{
    gain:f32,
}

impl Gain{
    pub fn new() -> Gain{
        Gain{
            gain: 1.0,
        }
    }
    pub fn value(&self) -> f32{
        self.gain
    }

    pub fn ptr(&mut self) -> &mut f32{
        &mut self.gain
    }

    pub fn silence(&mut self)
    {
        self.gain = 0.0;
    }
}
