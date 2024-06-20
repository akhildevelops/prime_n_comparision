use std::env;
use std::io::{BufWriter, Write};
use std::time::Instant;

fn is_prime(n: u32) -> bool {
    if n < 2 {
        false
    } else if n < 4 {
        true
    } else {
        let factor = (n as f32).sqrt() as u32;
        !(2..factor + 1).any(|x| n % x == 0)
    }
}

fn gen_primes<F>(gen: u32, mut callback: F)
where
    F: FnMut(u32, u32),
{
    let mut counter = 0;
    let mut n = 0;
    while counter < gen {
        if is_prime(n) {
            counter += 1;
            callback(counter, n);
        }
        n += 1
    }
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let mut file_writer = BufWriter::new(
        std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open("./benchmark_rust")
            .unwrap(),
    );
    let n = args.next().unwrap().parse::<u32>().unwrap();
    let buckets = args.next().unwrap().parse::<u32>().unwrap();
    let start = Instant::now();
    gen_primes(n as u32, |counter: u32, prime_val: u32| {
        if counter % buckets == 0 || counter == n {
            let end_time = Instant::now();
            write!(
                file_writer,
                "{},{},{}\n",
                counter,
                prime_val,
                (end_time - start).as_nanos()
            )
            .unwrap();
        }
    });
    file_writer.flush().unwrap();
}
