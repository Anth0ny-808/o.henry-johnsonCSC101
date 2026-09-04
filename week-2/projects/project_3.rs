fn main() {
   let p = 210_000;
   let r = 5;
   let n = 3;
   let a = p as f64 * (1.0 - (r as f64 / 100.0)).powf(n as f64);
   println!("amount is {}",a);
   let de = a - p as f64;
   println!("the depriciation is {}", de)
}
