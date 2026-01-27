#[derive(Clone,Debug)]
pub struct Clipper{
    max:f32,
}

impl Clipper{
    pub fn new(max:f32) -> Clipper{
        assert!(max > 0.0);
        Clipper{
            max,
        }
    }

    pub fn next(&self, x:f32) -> f32
    {
        x.clamp(-self.max, self.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;    
    
    #[test]
    fn zero() {
        let c = Clipper::new(1.0);

        let x = 0.0;
        let y = c.next(x);
        assert_eq!(y, 0.0);
    }
    
    #[test]
    fn positive_within_range() {
        let c = Clipper::new(1.0);

        let x = 0.5;
        let y = c.next(x);
        assert_eq!(y, 0.5);
    }
    
    #[test]
    fn negative_within_range() {
        let c = Clipper::new(1.0);

        let x = -0.5;
        let y = c.next(x);
        assert_eq!(y, -0.5);
    }
    
    #[test]
    fn positive_above_range() {
        let c = Clipper::new(1.0);

        let x = 1.001;
        let y = c.next(x);
        assert_eq!(y, 1.0);
    }
    
    #[test]
    fn negative_above_range() {
        let c = Clipper::new(1.0);

        let x = -1.001;
        let y = c.next(x);
        assert_eq!(y, -1.0);
    }
    
    #[test]
    fn positive_exact() {
        let c = Clipper::new(1.0);

        let x = 1.0;
        let y = c.next(x);
        assert_eq!(y, 1.00);
    }
    
    #[test]
    fn negative_exact() {
        let c = Clipper::new(1.0);

        let x = -1.0;
        let y = c.next(x);
        assert_eq!(y, -1.0);
    }
}
