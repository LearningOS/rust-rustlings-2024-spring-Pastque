// options1.rs
//
// Execute `rustlings hint options1` or use the `hint` watch subcommand for a
// hint.

// This function returns how much icecream there is left in the fridge.
// If it's before 10PM, there's 5 pieces left. At 10PM, someone eats them
// all, so there'll be no more left :(

// 这个函数返回冰箱中还剩下多少冰淇淋。
// 如果是晚上10点前，还剩5块。晚上10点，有人把它们吃光了，所以就没有剩下了:(
fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    // We use the 24-hour system here, so 10PM is a value of 22 and 12AM is a
    // value of 0 The Option output should gracefully handle cases where
    // time_of_day > 23.
    // TODO: Complete the function body - remember to return an Option!
    // 我们在这里使用24小时制，因此10PM的值为22, 12AM的值为0。
    // 选项输出应该优雅地处理time_of_day > 23的情况。
    // 待办事项:完成函数体——记得返回一个选项!

    if time_of_day <= 24 {
        Some(if time_of_day >= 22 { 0 } else { 5 })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the
        // Option?
        // 待办事项:修改这个测试。如何得到选项中包含的值?
        let icecreams = maybe_icecream(12);
        // let icecreams = maybe_icecream(12).unwrap();
        //   或者
        // assert_eq!(icecreams, 5);;
        assert_eq!(icecreams, Some(5));
    }
}