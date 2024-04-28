// vecs2.rs
//// 给定一个包含偶数的 Vec，你的任务是完成循环，使得 Vec 中的每个数都被乘以 2。

// 请确保代码能通过测试！
//
// 若要获得提示，请执行 `rustlings hint vecs2` 或者使用 `hint` 观察子命令。



fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    // 待办事项：在此填充代码，使 Vec `v` 中的每个元素都被乘以 2。
    
    for  element in v.iter_mut() {
        *element = *element * 2;
    }

    // 此时，`v` 应该等于 [4, 8, 12, 16, 20]。
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    // 待办事项：与上方相同的操作，但无需修改 Vec，只需返回新的数值即可。
    
    v.iter().map(|element| {
        element * 2
    }).collect()
}

// 使用 cfg 测试宏
#[cfg(test)]
mod tests {
    use super::*; // 引入父模块中的所有项目

    #[test]
    fn test_vec_loop() {
        // 创建一个包含前5个偶数的 Vec
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // 调用 vec_loop 函数并对原 Vec 进行深拷贝
        let ans = vec_loop(v.clone());

        // 断言经过 vec_loop 函数处理后的 Vec 与原 Vec 的每个元素乘以 2 后的结果一致
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        // 创建同样的 Vec
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        // 调用 vec_map 函数，传入 Vec 的引用
        let ans = vec_map(&v);

        // 断言经过 vec_map 函数处理后的 Vec 与原 Vec 的每个元素乘以 2 后的结果一致
        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}