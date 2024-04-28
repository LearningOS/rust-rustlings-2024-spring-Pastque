// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.



pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.


    /*  递归1
    fn rec(num:u64, time:u64)->u64{
        if time == 0|| time==1{
            num
        }else{
            rec(num*time, time-1)
        }
    }
    rec(1,num)
    */

    /*  递归2
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
    */

    /* 奇怪的方法1
    */
    (1..=num).product()

    /* 奇怪的方法2
    (0..=num).fold(1,|num0, time|{
        match time{
            0 | 1 => num0,
            _ => num0 * time, 
        }
    })
    */


}
#[cfg(test)]
mod tests {
    use super::*;
 
    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
