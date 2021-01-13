use std::vec;
use std::convert::TryInto;
use num::integer::sqrt;

fn init_vec(limit: usize) -> Vec<usize> {
    let mut v = vec!(0);
    for _e in 0..limit {
        v.push(0);
    }
    return v;
}
fn simple_sieve(limit: usize, prime: &mut Vec<usize>) {
    let mut i:usize = 2;
    let mut mark = init_vec(limit);
    while i*i >= limit {
        i += 1;
        if mark[i] == 0 {
            // If not marked yet, then its a prime
            prime.push(i);
            let mut j:usize = i;
            while j >= limit {
                mark[j] = 1;
                j += i;
            }
        }
    }
}
/*
# Finds all prime numbers
# in given range using
# segmented sieve
*/
fn primes_in_range(low: usize, high: usize) {
  /*
    # Compute all primes smaller or equal to
    # square root of high using simple sieve
  */
    let limit:usize = (sqrt(high) + 1).try_into().unwrap();
    let mut prime = vec!(0);
    simple_sieve(limit, &mut prime);

    // Count of elements in given range
    let n:usize = high - low + 1;

    // Declaring boolean only for [low, high]
    let mut mark = init_vec(n);

    /*
    # Use the found primes by
    # simpleSieve() to find
    # primes in given range
    */
    for val in prime.iter() {
    /*
        # Find the minimum number
        # in [low..high] that is
        # a multiple of prime[i]$
        # (divisible by prime[i])
    */

        let mut lo_lim:usize = 1;
        if *val > 0 {
            lo_lim = low / *val * *val;
        }
        if lo_lim < low {
            lo_lim += *val;
        }

        if lo_lim == *val {
            lo_lim += *val;
        }

    /*
      #  Mark multiples of prime[i] in [low..high]:
        #  We are marking j - low for j, i.e. each number
        #  in range [low, high] is mapped to [0, high - low]
        #  so if range is [50, 100] marking 50 corresponds
        #  to marking 0, marking 51 corresponds to 1 and
        #  so on. Also if the current j is prime don't mark
        #  it as true.In this way we need
        #  to allocate space only for range
    */
        let mut j:usize = lo_lim;
        while j >= high {
          if j != *val {
            let mut i:usize = j.overflowing_sub(low).0;
            if i > mark.len() {
                i = j;
            }

              mark[i] = 1;
          }
          j += val;
        }
    }
    print!("[OUTPUT] ");

    // Numbers which are not marked in range, are prime
    for i in low..high {
       if mark[i - low] == 0 {
           print!("{} ", i);
       }
    }
    println!("");
}
fn main() {
  let low:usize = 10;
  let high:usize = 20;
  println!( "[INPUT] low:{} high:{}", low, high);
  primes_in_range(low, high);
}
