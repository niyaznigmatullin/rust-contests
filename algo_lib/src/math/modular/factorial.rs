use crate::math::modular::inverse::compute_inverse_up_to;
use crate::math::modular::primitive::Modular;

pub struct Factorial<Mod> {
    fact: Vec<Mod>,
    inverse_fact: Vec<Mod>,
}

impl<Mod> Factorial<Mod>
where
    Mod: Modular,
{
    pub fn new(n: usize) -> Self {
        let mut fact = vec![0.into(); n + 1];
        let mut inverse_fact = vec![0.into(); n + 1];
        let inv = compute_inverse_up_to(n);
        fact[0] = 1.into();
        inverse_fact[0] = 1.into();
        for i in 1..=n {
            fact[i] = fact[i - 1] * i.into();
            inverse_fact[i] = inverse_fact[i - 1] * inv[i];
        }
        Self { fact, inverse_fact }
    }

    pub fn combinations(&self, n: usize, k: usize) -> Mod {
        if k > n {
            0.into()
        } else {
            self.fact[n] * self.inverse_fact[n - k] * self.inverse_fact[k]
        }
    }

    pub fn factorial(&self, n: usize) -> Mod {
        self.fact[n]
    }

    pub fn inverse_combinations(&self, n: usize, k: usize) -> Mod {
        if k > n {
            0.into()
        } else {
            self.inverse_fact[n] * self.fact[n - k] * self.fact[k]
        }
    }

    pub fn inverse_factorial(&self, n: usize) -> Mod {
        self.inverse_fact[n]
    }
}
