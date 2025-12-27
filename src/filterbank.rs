use std::sync::{Arc, RwLock};

use crate::biquad::Biquad;

pub struct FilterBank{
    biquad:Vec<Biquad>,
    g:Arc<RwLock<f32>>,
}

impl FilterBank{
    pub fn new(filters: &Vec<Biquad>,
            gain:Arc<RwLock<f32>>) -> FilterBank{
        FilterBank{
            biquad: filters.to_vec(),
            g: gain.clone(),
        }
    }
    pub fn next(&mut self, x:f32) -> f32
    {
        let mut y = x;
        for mut filter in &mut self.biquad
        {
            y = filter.next(y);
        }
        y * *self.g.read().unwrap()
    }
}
