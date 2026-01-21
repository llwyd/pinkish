use crate::single_pole_lpf::SinglePoleLPF;

#[derive(Clone,Debug)]
pub struct RMS{
    lpf: SinglePoleLPF,
    rms: f32,
}


impl RMS{

    pub fn new(fc:f32, fs:f32) -> RMS
    {
        RMS
        {
            lpf: SinglePoleLPF::new(fc,fs),
            rms: 0.0,
        }
    }

#[allow(dead_code)]    
    pub fn value(&self) -> f32
    {
        self.rms
    }

    pub fn next(&mut self, x:f32) -> f32
    {
        self.rms = self.lpf.next(x.abs());
        self.rms
    }
}
