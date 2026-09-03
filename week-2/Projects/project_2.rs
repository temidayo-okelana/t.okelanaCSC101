fn main() {
	let item_1_qty:f64 = 2.0;
	let item_1_amt:f64 = 450000.0;
	let _item_1_total:f64 = item_1_qty*item_1_amt;

	let item_2_qty:f64 = 1.0;
	let item_2_amt:f64 = 1500000.0;
	let _item_2_total:f64 = item_2_qty*item_2_amt;

	let item_3_qty:f64 = 3.0;
	let item_3_amt:f64 = 750000.0;
	let _item_3_total:f64 = item_3_qty*item_3_amt;

	let item_4_qty:f64 = 3.0;
	let item_4_amt:f64 = 2850000.0;
	let _item_4_total:f64 = item_4_qty*item_4_amt;

	let item_5_qty:f64 = 1.0;
	let item_5_amt:f64 = 250000.0;
	let _item_5_total:f64 = item_5_qty*item_5_amt;

	let qty = item_1_qty+item_2_qty+item_3_qty+item_4_qty+item_5_qty;
	let amt = item_1_amt+item_2_amt+item_3_amt+item_4_amt+item_5_amt;

	println!("The sum is {}", amt);
	println!("The average is {}", amt/qty);
}