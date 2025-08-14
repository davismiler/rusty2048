use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Game random number generator with seed support
#[derive(Debug, Clone)]
pub struct GameRng {
    rng: StdRng,
}

impl GameRng {
    /// Create a new RNG with optional seed
    pub fn new(seed: Option<u64>) -> Self {
        let rng = if let Some(seed) = seed {
            StdRng::seed_from_u64(seed)
        } else {
            StdRng::from_entropy()
        };
        
        Self { rng }
    }
    
    /// Generate a random value between 0 and max (exclusive)
    pub fn gen_range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        self.rng.gen_range(0..max)
    }
    
    /// Generate a random boolean with given probability
    pub fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }
    
    /// Generate a random tile value (2 or 4 with 90/10 probability)
    pub fn gen_tile_value(&mut self) -> u32 {
        if self.gen_bool(0.9) {
            2
        } else {
            4
        }
    }
    
    /// Get the current seed (if available)
    pub fn get_seed(&self) -> Option<u64> {
        // Note: This is a simplified implementation
        // In a real implementation, you'd need to store the seed separately
        None
    }
}

impl Default for GameRng {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rng_with_seed() {
        let mut rng1 = GameRng::new(Some(42));
        let mut rng2 = GameRng::new(Some(42));
        
        // Same seed should produce same sequence
        for _ in 0..10 {
            assert_eq!(rng1.gen_range(100), rng2.gen_range(100));
        }
    }
    
    #[test]
    fn test_tile_value_generation() {
        let mut rng = GameRng::new(Some(123));
        let mut twos = 0;
        let mut fours = 0;
        
        for _ in 0..1000 {
            let value = rng.gen_tile_value();
            match value {
                2 => twos += 1,
                4 => fours += 1,
                _ => panic!("Unexpected tile value: {}", value),
            }
        }
        
        // Should be roughly 90% twos and 10% fours
        assert!(twos > 800 && twos < 950);
        assert!(fours > 50 && fours < 200);
    }
}
