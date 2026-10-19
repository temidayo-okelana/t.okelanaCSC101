fn main() {
    let v:Vec<i64> = Vec::new(); // first way fo creating vectors

    println!("\nThe length of Vec::new is: {}",v.len());

    let v = vec!["Grace","Effiong","Basil","Kareem","Susan"]; // second way of creating vectors

    println!("\nThe length of vec macro is: {}",v.len());
}
