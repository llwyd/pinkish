use std::sync::{Arc, RwLock};

use crate::crossover::Crossover;

pub struct Equaliser{
    cross: Vec<Crossover>,
    g:Arc<RwLock<Vec<f32>>>,
}

impl Equaliser{
    pub fn new(crossovers: Vec<Crossover>, band_gain:Arc<RwLock<Vec<f32>>>
        ) -> Equaliser{

        let num_bands = crossovers.len() + 1;
        assert!(crossovers.len() > 0);
        assert!(num_bands > 0);
        println!("Initialising EQ with {} LR Crossover filters and {} bands", crossovers.len(), num_bands);
        
        Equaliser{
            cross: crossovers.clone(),
            g: band_gain.clone(),
        }
    }
    pub fn gain(&self) -> Arc<RwLock<Vec<f32>>>
    {
        self.g.clone()
    }
    pub fn next(&mut self, x:f32) -> f32
    {
        let mut low;
        let mut high = x;

        let mut out = 0.0;
        for (idx, cross) in self.cross.iter_mut().enumerate()
        {
            (low, high) = cross.next(high);
            out += low * self.g.read().unwrap()[idx];
        }
        out += high * self.g.read().unwrap().last().unwrap();
        out
    }
}
