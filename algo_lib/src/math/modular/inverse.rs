use crate::math::modular::primitive::Modular;

pub fn compute_inverse_up_to<Mod>(n: usize) -> Vec<Mod>
where
    Mod: Modular,
{
    let mut inv: Vec<Mod> = vec![0.into(); n + 1];
    inv[1] = 1.into();
    for i in 2..=n {
        inv[i] = inv[Mod::MODULO as usize % i] * (Mod::MODULO as usize / i).into();
        inv[i] = -inv[i];
    }
    inv
}
