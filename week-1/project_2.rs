fn main() {
    let t:f64 = 2.0 * 450000.0;
    let m:f64 = 1500000.0;
    let h:f64 = 3.0 * 750000.0;
    let d:f64 = 3.0 * 2850000.0;
    let a:f64 = 250000.0;
    let n:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
    
    // Average
    let s = t + m + h + d + a + n;
    println!("Sum is {}", s);
    let av = s / n;
    println!("avarage is {}", av);

}