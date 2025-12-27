use std::sync::{Arc, RwLock};

pub struct Biquad{
    a:[f32;3],
    b:[f32;2],
    g:Arc<RwLock<f32>>,
    s:[f32;2],
}

impl Biquad{
    pub fn new(a:[f32;3],
        b:[f32;2],
        gain:Arc<RwLock<f32>>) -> Biquad{
        
        Biquad
        {
            a: a,
            b: b,
            g: gain.clone(),
            s: [0.0, 0.0],
        }
    }

    pub fn next(&mut self, x:f32) -> f32
    {
        let y = x * self.a[0] + self.s[0];
        self.s[0] = self.s[1] + (x * self.a[1]) - (self.b[0] * y);
        self.s[1] = (x * self.a[2]) - (y * self.b[1]);
        y * *self.g.read().unwrap()
    }
}
