pub struct Resonator
{
    a1: f32,
    a2:f32,
    y:[f32;2],
}

impl Resonator{

    pub fn new() -> Resonator{
        
        Resonator{
            a1: 0.0,
            a2: 0.0,
            y: [0.0,0.0],
        }
    }
}
