// errors5.rs
//
// This program uses an altered version of the code from errors4.
//
// This exercise uses some concepts that we won't get to until later in the
// course, like `Box` and the `From` trait. It's not important to understand
// them in detail right now, but you can read ahead if you like. For now, think
// of the `Box<dyn ???>` type as an "I want anything that does ???" type, which,
// given Rust's usual standards for runtime safety, should strike you as
// somewhat lenient!
//
// In short, this particular use case for boxes is for when you want to own a
// value and you care only that it is a type which implements a particular
// trait. To do so, The Box is declared as of type Box<dyn Trait> where Trait is
// the trait the compiler looks for on any value used in that context. For this
// exercise, that context is the potential errors which can be returned in a
// Result.
//
// What can we use to describe both errors? In other words, is there a trait
// which both errors implement?
//
//
// 这个程序使用了errors4中的代码的修改版本。
//
// 这个练习使用了一些我们在本课程后面才会讲到的概念，
// 比如`Box `和`From` trait。现在详细理解它们并不重要，但如果你愿意，你可以继续阅读。
// 现在，考虑` Box<dyn ??> `类型作为“我想要任何做??”的类型，考虑到Rust的运行时安全通常标准，
// 这应该会让你感到有点宽松!
//
// 简而言之，box的这种特殊用例用于当您想拥有一个值，并且您只关心它是实现特定特征的类型。
// 为此，Box被声明为Box<dyn Trait>类型，其中Trait是编译器在该上下文中使用的任何值上寻找的Trait。
// 对于这个练习来说，上下文就是结果中可能返回的潜在错误。我们可以用什么来描述这两种错误?
// 换句话说，是否存在两个错误都实现的trait ?
//
// 我们可以用什么来描述这两种错误?换句话说，是否存在两个错误都实现的trait ?
//
// Execute `rustlings hint errors5` or use the `hint` watch subcommand for a
// hint.



use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
// fn main() -> Result<(), Box<dyn ???>> {
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
