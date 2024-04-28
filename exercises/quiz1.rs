// quiz1.rs
//
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given the
// quantity bought. No hints this time!
//
// No hints this time ;)



// Put your function here!
// fn calculate_price_of_apples {

// Don't modify this function!

// This is a quiz for the following sections:VariablesFunctionsIfMary is buying apples. 
// 这是一个关于以下部分的测验:变量函数玛丽正在买苹果。
// The price of an apple is calculated as follows: An apple costs 2 rustbucks.
// 一个苹果的价格计算方法如下:一个苹果2卢比。
// If Mary buys more than 40 apples, each apple only costs 1 rustbuck! 
// 如果玛丽买了40多个苹果，每个苹果只花了1卢比!
// Write a function that calculates the price of an order of apples given the quantity bought. 
// 编写一个函数，计算给定购买量的苹果订单的价格。


#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}

fn calculate_price_of_apples(x:i32) -> i32{
    if x <= 40{
        x*2
    }else{
        x
    }
}



