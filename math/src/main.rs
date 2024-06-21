use primefactor::primefactor_gcd;
use prime_factorization::Factorization;

fn main() {
    let factor = Factorization::<u32>::run(30030);

    println!("{:?}",factor);

    println!("{}",2*3*5*7*11*13);
}
