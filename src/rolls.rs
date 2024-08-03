use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Roll {
    Disaster(u8),
    Grim(u8),
    Messy(u8),
    Perfect(u8),
    Critical,
}
impl Roll {
    fn is_grim(self) -> bool {
        matches!(self, Roll::Grim(_))
    }
    fn is_perfect(self) -> bool {
        matches!(self, Roll::Perfect(_))
    }
}

pub struct Rolls {
    dist: Uniform<u8>,
}
impl Rolls {
    pub fn new() -> Self {
        Self {
            dist: Uniform::new_inclusive(1, 6),
        }
    }
    fn roll_n<'a, R: Rng + ?Sized>(
        &'a self,
        rng: &'a mut R,
        num_dice: usize,
    ) -> impl Iterator<Item = Roll> + 'a {
        self.sample_iter(rng)
            // .inspect(|r| {
            //     dbg!(r);
            // })
            .take(num_dice)
    }
    fn sample_pool<R: Rng + ?Sized>(&self, rng: &mut R, num_dice: usize) -> Roll {
        if num_dice == 0 {
            self.roll_n(rng, 2).min().expect("pool has two elements")
        } else {
            let mut current_max = Roll::Grim(1);
            for roll in self.roll_n(rng, num_dice) {
                if current_max.is_perfect() && roll.is_perfect() {
                    current_max = Roll::Critical;
                    break;
                } else {
                    current_max = current_max.max(roll);
                }
            }
            current_max
        }
    }
}
impl Distribution<Roll> for Rolls {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Roll {
        let roll = self.dist.sample(rng);
        match roll {
            r @ 1..=3 => Roll::Grim(r),
            r @ (4 | 5) => Roll::Messy(r),
            r @ 6 => Roll::Perfect(r),
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Pool {
    dice: u8,
}
impl Pool {
    pub fn new(dice: u8) -> Self {
        Self { dice }
    }
    pub fn dice(&self) -> u8 {
        self.dice
    }
    pub fn roll(&mut self, rolls: &Rolls) -> Vec<Roll> {
        let mut rng = thread_rng();
        self.roll_rng(&mut rng, rolls)
    }
    pub fn roll_rng<R: Rng + ?Sized>(&mut self, rng: &mut R, rolls: &Rolls) -> Vec<Roll> {
        let rolls: Vec<_> = rolls
            .roll_n(rng, self.dice as usize)
            .inspect(|roll| {
                if roll.is_grim() {
                    self.dice -= 1;
                }
            })
            .collect();
        rolls
    }
    // pub fn roll_until_depleted<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
    //     self.dist
    //         .sample_iter(rng)
    //         .take(self.dice)
    //         .max()
    //         .unwrap_or_default() as u8
    //         + 1
    // }
}
impl std::fmt::Display for Roll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roll::Disaster(r) => r.fmt(f),
            Roll::Grim(r) => r.fmt(f),
            Roll::Messy(r) => r.fmt(f),
            Roll::Perfect(r) => r.fmt(f),
            Roll::Critical => 6.fmt(f),
        }
    }
}
