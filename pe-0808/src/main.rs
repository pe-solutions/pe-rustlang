// Reversible Prime Squares

 fn is_prime(u: u64) -> bool {
     if u <= 1 {
         return false;
     }

     if u <= 3 {
         return true;
     }

     if u % 2 == 0 || u % 3 == 0 {
         return false;
     }
 
     let mut i = 5;

     while i * i <= u {
         if u % i == 0 || u % (i + 2) == 0 {
             return false;
         }
         
         i += 6;
     }
 
     true
 }
 
 fn reverse(n: u64) -> u64 {
     let mut r = 0;
     let mut n = n;
     while n > 0 {
         r *= 10;
         r += n % 10;
         n /= 10;
     }
     r
 }
 
 fn sum_of_rev_prime(mut c: u32) -> u64 {
     let mut s;
     let mut rev;
     let mut z = 0;
     let mut p10 = 10;
     let mut p100 = 100;
     let mut q;
     let mut i = 0;
     let mut r;
     let mut n = 13;
 
     while c > 0 {
         if n > p100 {
             p10 *= 10;
             p100 *= 10;
         }
 
         q = n / p10;
         if q == 1 || q == 3 {
             if is_prime(n) {
                 s = (n as u64) * (n as u64);
                 rev = reverse(s);
 
                 if rev != s {
                     r = (rev as f64).sqrt() as u64;
 
                     if r * r == rev && r % 2 > 0 && r % 3 > 0 && is_prime(r) {
                         c -= 1;
                         z += s;
                     }
                 }
             }
         }
 
         n += 0x2A8A >> ((i & 3) << 2) & 15;
         i += 1;
     }
 
     z
 }
 
 fn main() {
     let start = std::time::Instant::now();
 
     let answer = sum_of_rev_prime(50);
 
    println!("\nProject Euler #808\nAnswer: {}", answer);
 
    let duration = start.elapsed();
    println!("Elapsed time: {} milliseconds.\n", duration.as_millis()); 

}
 