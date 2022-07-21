// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. One apple usually costs 2 Rustbucks, but if you buy
// more than 40 at once, each apple only costs 1! Write a function that calculates
// the price of an order of apples given the quantity bought. No hints this time!


// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(65, price3);
}

const NORMAL_COST: i32 = 2;
const DISCOUNTED_COST: i32 = 1;
const DISCOUNTED_NUMBER: i32 = 40;

fn calculate_price_of_apples(number: i32) -> i32 {
    if number > DISCOUNTED_NUMBER {
        return number * DISCOUNTED_COST
    }
    
    number * NORMAL_COST
}