// import time
// import math
// def is_prime(n:int)->bool:
//     if n<2:
//         return False
//     for i in range(2,n//2+1):
//         if n%i==0:
//             return False
//     return True

// def gen_primes(gen:int):
//     counter=0
//     n=0
//     while counter<gen:
//         if is_prime(n):
//             counter+=1
//         n+=1

// def main(cum:int):
//     n_time=[]
//     for i in range(cum):
//         start = time.perf_counter_ns()
//         gen_primes(i)
//         end = time.perf_counter_ns()
//         n_time.append(f"{i},{math.log10(end-start)}\n")
//     with open("./benchmark_python","w") as file:
//         file.writelines(n_time)

// if __name__=="__main__":
//     main(1000)
use std::time::Instant;
// use std::
fn is_prime(n:u16)->bool{
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

fn gen_primes(gen:u16)->Option<u16>{
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

const N:usize=4500;
fn main() {
    let mut n_time:Vec<String>=vec![];
    for i in 1..N+1{
        let last_p:Option<u16>;
        let start = Instant::now();
        last_p = gen_primes(i as u16);
        let dur = Instant::now()-start;
        let val = last_p.map_or("None".to_string(), |x| format!("{}",x));
        let op = format!("{},{},{}",i,val,(dur.as_nanos() as f32).log10());
        println!("{}",op);
        n_time.push(op);
    }
    std::fs::write("./benchmark_rust",n_time.join("\n")).unwrap();
}
