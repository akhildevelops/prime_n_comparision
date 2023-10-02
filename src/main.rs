use std::time::Instant;
use std::env;
fn is_prime(n:u32)->bool{
    if n<2{
        return false
    }
    for i in 2..n/2+1 {
        if n%i==0{
            return false
        }
    }
    true
}

fn gen_primes(gen:u32)->Option<u32>{
    let mut counter=0;
    let mut n=0;
    let mut last_p = None;
    while counter<gen {
        if is_prime(n){
            counter+=1;
            last_p=Some(n);
        }
        n+=1
    }
    last_p
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();
    let n = args.next().unwrap().parse::<usize>().unwrap();
    let buckets = args.next().unwrap().parse::<usize>().unwrap();
    let mut n_time:Vec<String>=vec![];
    for i in (1..n+n/buckets+1).step_by((n/buckets).into()){
        let last_p:Option<u32>;
        let start = Instant::now();
        last_p = gen_primes(i as u32);
        let dur = Instant::now()-start;
        let val = last_p.map_or("None".to_string(), |x| format!("{}",x));
        let op = format!("{},{},{}",i,val,(dur.as_nanos() as f32).log10());
        println!("{}",op);
        n_time.push(op);
    }
    std::fs::write("./benchmark_rust",n_time.join("\n")).unwrap();
}
