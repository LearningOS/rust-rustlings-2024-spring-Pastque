// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Clippy 工具是一组用于分析代码的 lint 程序，以便您可以捕获常见错误并改进Rust代码。
// 对于这些练习，当有clippy警告时，代码将无法编译。请从输出中检查clippy的建议来解决练习。
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.



use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
