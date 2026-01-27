use std::sync::{Arc, RwLock};

use crate::crossover::Crossover;
use crate::single_pole_lpf::SinglePoleLPF;

pub struct Equaliser{
    cross: Vec<Crossover>,
    g:Arc<RwLock<Vec<f32>>>,
    gain_filter: Vec<SinglePoleLPF>,
    num_bands: usize,
}

impl Equaliser{
    const GAIN_CUTOFF:f32 = 2.0;
    pub fn new(crossovers: Vec<Crossover>,
        band_gain:Arc<RwLock<Vec<f32>>>,
        fs: f32,
        ) -> Equaliser{

        let num_bands = crossovers.len() + 1;
        assert!(crossovers.len() > 0);
        assert!(num_bands > 0);
        
        let mut e = Equaliser{
            cross: crossovers.clone(),
            g: band_gain.clone(),
            gain_filter: Vec::new(),
            num_bands,
        };
        
        for i in 0..num_bands
        {
            e.gain_filter.push(SinglePoleLPF::new(Self::GAIN_CUTOFF, fs, e.g.read().unwrap()[i] ));
        }

        e
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
            let g = self.g.read().unwrap()[idx];
            let gain = self.gain_filter[idx].next(g);
            (low, high) = cross.next(high);
            out += low * gain;
        }
        /* This is shite, FIXME */
        let g = self.g.read().unwrap()[self.num_bands -1];
        let gain = self.gain_filter[self.num_bands - 1].next(g);
        out += high * gain; 
        out
    }
}
