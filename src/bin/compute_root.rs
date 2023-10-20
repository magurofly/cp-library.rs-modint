use modint::modulus::*;

const P: u64 = 65537;

fn main() {
    let m = StaticModulus64::<P>;
    if m.is_prime() {
        println!("{} is a prime", P);
        println!("having a primitive root {}", StaticModulus64::<P>::primitive_root());
    } else {
        println!("{} is not a prime", P);
    }
}