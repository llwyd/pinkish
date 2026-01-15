use crate::biquad::Biquad;

#[derive(Clone,Debug)]
pub struct CrossoverBiquads{
    pub lpf:Vec<Biquad>,
    pub hpf:Vec<Biquad>,
}

#[derive(Clone)]
pub struct Crossover{
    lpf:Vec<Biquad>,
    hpf:Vec<Biquad>,
}

impl Crossover{
    pub fn new(lpf: Vec<Biquad>,
                hpf: Vec<Biquad>) -> Crossover{
        Crossover{
            lpf: lpf.clone(),
            hpf: hpf.clone(),
        }
    }
    pub fn next(&mut self, x:f32) -> (f32,f32)
    {
        let mut low = x;
        let mut high = x;
        
        for filter in &mut self.lpf
        {
            low = filter.next(low);
        }
        
        for filter in &mut self.hpf
        {
            high = filter.next(high);
        }

        (low, high)
    }
}
