use crate::util::vector::Vec3;

pub struct Random {
    seed: u32
}

impl Random {
    pub fn new(seed: u32) -> Random {
        Random { seed }
    }

    pub fn next(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(747796405).wrapping_add(2891336453);

        let result = (self
            .seed
            .wrapping_shr(self.seed.wrapping_shr(28).wrapping_add(4))
            ^ self.seed)
            .wrapping_mul(747796405);
        let result = result.wrapping_shr(22) ^ result;

        result as f64 / u32::MAX as f64
    }

    pub fn randint(&mut self, low: u32, high: u32) -> u32 {
        self.seed = self.seed.wrapping_mul(747796405).wrapping_add(2891336453);

        let result = (self
            .seed
            .wrapping_shr(self.seed.wrapping_shr(28).wrapping_add(4))
            ^ self.seed)
            .wrapping_mul(747796405);
        let result = result.wrapping_shr(22) ^ result;

        low + result % (high - low)
    }

    pub fn random_dir(&mut self) -> Vec3 {
        loop {
            let p = Vec3::new(
                [self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0]
            );

            if p * p < 1.0 {
                let v = p.normalized();

                return v;
            }
        }
    }

    pub fn randon_hemisphere_dir(&mut self, normal: Vec3) -> Vec3 {
        loop {
            let p = Vec3::new(
                [self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0]
            );

            if p * p < 1.0 {
                let v = p.normalized();

                if v * normal > 0.0 {
                    return v;
                } else {
                    return v * -1.0;
                }
            }
        }
    }
}