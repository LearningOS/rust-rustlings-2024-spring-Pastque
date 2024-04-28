// errors2.rs
//
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the tokens. Since
// the player typed in the quantity, though, we get it as a string-- and they
// might have typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is: if we call
// the `parse` function on a string that is not a number, that function will
// return a `ParseIntError`, and in that case, we want to immediately return
// that error from our function and not try to multiply and add.
//
// There are at least two ways to implement this that are both correct-- but one
// is a lot shorter!
//
// 假设我们正在编写一个可以用代币购买道具的游戏。
// 所有项目都需要5个代币，每当您购买项目时，都需要支付1个代币的手续费。
// 游戏玩家将输入他们想要购买的物品数量，然后`total_cost`函数将计算token的总成本。
// 因为玩家输入了数量，我们得到的是一个字符串——他们可能输入了任何东西，而不仅仅是数字!
//
// 现在，这个函数根本没有处理错误的情况(也没有正确地处理成功的情况)。
// 我们要做的是:如果我们对一个不是数字的字符串调用`parse`函数，该函数将返回一个`ParseIntError `，
// 在这种情况下，我们希望立即从我们的函数中返回错误，而不是尝试乘法和加法。
//
// 至少有两种实现方法都是正确的——但其中一种方法要短得多!
//
// Execute `rustlings hint errors2` or use the `hint` watch subcommand for a
// hint.



use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    

    // let qty = item_quantity.parse::<i32>();
    
    // let qty = item_quantity.parse::<i32>()? ;
    // 等价于下列
    let qty = match item_quantity.parse::<i32>() {
        Ok(val) => return Ok(val * cost_per_item + processing_fee),
        Err(e) => return Err(e),
    };

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}
