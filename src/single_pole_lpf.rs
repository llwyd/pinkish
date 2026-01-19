#[derive(Clone,Debug)]
pub struct SinglePoleLPF{
    alpha:f32,
    y:f32,
}

impl SinglePoleLPF{
    pub fn new(fc:f32, fs:f32) -> SinglePoleLPF
    {
        assert!(fs > 0.0);
        assert!(fc > 0.0);

        let ratio = fc / fs;
        let exp = -2.0 * std::f32::consts::PI * ratio;
        SinglePoleLPF
        {
            alpha: exp.exp(),
            y: 0.0,
        }
    }

    pub fn next(&mut self, x:f32) -> f32
    {
        self.y = x + (self.alpha * (self.y - x));
        self.y
    }
}
