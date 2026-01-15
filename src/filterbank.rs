use std::sync::{Arc, RwLock};

use crate::biquad::Biquad;

#[allow(dead_code)]
pub struct FilterBank{
    biquad:Vec<Biquad>,
    g:Arc<RwLock<f32>>,
}

impl FilterBank{
    #[allow(dead_code)]
    pub fn new(filters: Vec<Biquad>,
            gain:Arc<RwLock<f32>>) -> FilterBank{
        FilterBank{
            biquad: filters.clone(),
            g: gain.clone(),
        }
    }
    #[allow(dead_code)]
    pub fn next(&mut self, x:f32) -> f32
    {
        let mut y = x;
        for filter in &mut self.biquad
        {
            y = filter.next(y);
        }
        y * *self.g.read().unwrap()
    }
}
