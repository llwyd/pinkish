#[derive(Clone,Debug)]
pub struct RMS{
    alpha:f32,
    y:f32,
}


impl RMS{

    pub fn new(fc:f32, fs:f32) -> RMS
    {
        assert!(fs > 0.0);
        assert!(fc > 0.0);

        let ratio = fc / fs;
        let exp = -2.0 * std::f32::consts::PI * ratio;
        RMS
        {
            alpha: exp.exp(),
            y: 0.0,
        }
    }

    pub fn value(&self) -> f32
    {
        self.y
    }

    pub fn next(&mut self, x:f32) -> f32
    {
        let x_abs = x.abs();
        self.y = x_abs + (self.alpha * (self.y - x_abs));
        self.y
    }
}
