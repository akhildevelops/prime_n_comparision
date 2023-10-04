use std::time::Instant;
use std::env;
use std::rc::Rc;
use std::cell::RefCell;
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

fn gen_primes<F>(gen:u32,callback:F)
where F:FnOnce(u32,u32,Instant)+Copy{
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
    let n = args.next().unwrap().parse::<usize>().unwrap() as u32;
    let buckets = args.next().unwrap().parse::<usize>().unwrap() as u32;
    let n_time:Rc<RefCell<Vec<String>>>=Rc::new(RefCell::new(vec![]));
    let start = Instant::now();
    gen_primes(n as u32,|counter:u32,prime_val:u32,end_time:Instant|{
        if counter%buckets ==0 || counter==n{
        n_time.borrow_mut().push(format!("{},{},{}",counter,prime_val,((end_time-start).as_nanos() as f32).log10()))
        }
    });
    std::fs::write("./benchmark_rust",n_time.borrow_mut().join("\n")).unwrap();
}
