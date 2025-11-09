use std::io;
fn main() {
    
    let p = "P -- Poundo Yam & Edinkaiko soup";
    let f = "F -- Fried Rice & Chicken";
    let a = "A -- Amala & Ewedu Soup";
    let e = "E -- Eba & Egusi Soup";
    let w = "W -- White rice & Stew";

    let p1:f64 = 3_200.00;
    let f1:f64 = 3_300.00;
    let a1:f64 = 2_500.00;
    let e1:f64 = 2_000.00;
    let w1:f64 = 2_500.00;



    let menu = format!("{} {}
      {} {}
      {} {}
      {} {}
      {} {}",p,p1,f,f1,a,a1,e,e1,w,w1);

    println!("\n {}", menu);

    let mut ord = String::new();
    println!("Kindly take your order from the following:P,F,A,E,W...");
    io::stdin().read_line(&mut ord).expect("Not a valid input");

    let order = ord.trim().to_uppercase();


    let mut quantity = String::new();
    println!("Enter quantity");
    io::stdin().read_line(&mut quantity).expect("Failed to read input");
    let quantity:f64 = quantity.trim().parse().expect("Input not an interger");

    if order == "P" {
      let price1:f64 = p1 * quantity;
      println!("Total charges is {}", price1);
    }

    if order == "F" {
      let price2:f64 = f1 * quantity;
      println!("Total charges is {}", price2);
    }

    if order == "A" {
      let price3:f64 = a1 * quantity;
      println!("Total charges is {}", price3);
    }

    if order == "E" {
      let price4:f64 = e1 * quantity;
      println!("Total charges is {}", price4);
    }

    if order == "W" {
      let price5:f64 = w1 * quantity;
      println!("Total charges is {}", price5);
    }

    else {
      println!("Not a valid input. Try again");
    }




 let mut total_price:f64 = match order.as_str() {
      "P" => 3_200.00 * quantity,
      "F" => 3_300.00 * quantity,
      "A" => 2_500.00 * quantity,
      "E" => 2_000.00 * quantity,
      "W" => 2_500.00 * quantity,

      _ => {
        println!("Invalid item code entered.");
        0.00
        
      }
 };


 if total_price > 10_000.00 {
  let discount_rate = 0.95;
  let discount_amount = total_price * discount_rate;


  total_price -= discount_amount;

  println!("Congratulations! You are entitled to a 5% discount (discount_amount: {:.2}).", discount_amount);
 }

 if total_price > 0.0 {
  println!("The final total charge is {:.2}", total_price);
 }



}







