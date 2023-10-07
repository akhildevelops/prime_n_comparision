use std::time::Instant;
use std::env;
use std::io::{BufWriter, Write};
fn is_prime(n:u32)->bool{
    if n==2{
        return true;
    }else if n<2 || n%2==0{
        return false
    } else{
    let sqrt = (n as f32).sqrt() as u32;
    !(3..=sqrt).step_by(2).any(|i| n%i==0)
    }
}

fn gen_primes<F>(gen:u32,mut callback:F)
where F:FnMut(u32,u32,Instant){
    let mut counter=0;
    let mut n=0;
    while counter<gen {
        if is_prime(n){
            counter+=1;
            callback(counter,n,Instant::now())
        }
        n+=1
    }
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let mut file_writer = BufWriter::new(std::fs::File::options().create(true).write(true).truncate(true).open("./benchmark_rust").unwrap());
    let n = args.next().unwrap().parse::<u32>().unwrap();
    let buckets = args.next().unwrap().parse::<u32>().unwrap();
    let start = Instant::now();
    gen_primes(n as u32,|counter:u32,prime_val:u32,end_time:Instant|{
        if counter%buckets ==0 || counter==n{
        let f_str = format!("{},{},{}\n",counter,prime_val,(end_time-start).as_nanos());
        file_writer.write(f_str.as_bytes()).unwrap();
        }
    });
    file_writer.flush().unwrap();
}
