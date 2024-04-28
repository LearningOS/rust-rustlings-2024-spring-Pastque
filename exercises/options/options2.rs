// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        // 创建一个if let语句，其值为“Some”类型
        // word = optional_target {
        //     assert_eq!(word, target);
        // }
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
        // 等同于下列
        match optional_target {
            Some(word) => {
                assert_eq!(word, target);
            }
            None => (),
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
        // 让这个while let声明-记住矢量pop还添加了另一层选项<T>。
        //  你可以将`Option<T> `堆叠到while let和if let中。
        // integer = optional_integers.pop() {
        //     assert_eq!(integer, cursor);
        //     cursor -= 1;
        // }

        // 使用 while let 来连续解包 Option<Vec<Option<i8>>> 中的值
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
        // while let 等同下列
        loop {
            match optional_integers.pop() {
                Some(Some(integer)) => {
                    assert_eq!(integer, cursor);
                    cursor -= 1;
                    // if cursor == 0 { break; } //本身
                },
                _ => break, // 当optional_integers.pop()返回None时退出循环
            }
        }

        assert_eq!(cursor, 0);
    }
}
