// hashmaps2.rs
//
// We're collecting different fruits to bake a delicious fruit cake. For this,
// we have a basket, which we'll represent in the form of a hash map. The key
// represents the name of each fruit we collect and the value represents how
// many of that particular fruit we have collected. Three types of fruits -
// Apple (4), Mango (2) and Lychee (5) are already in the basket hash map. You
// must add fruit to the basket so that there is at least one of each kind and
// more than 11 in total - we have a lot of mouths to feed. You are not allowed
// to insert any more of these fruits!
//
// 我们正在收集不同的水果来烤一个美味的水果蛋糕。为此，我们有一个篮子，我们将以散列映射的形式表示它。
// 键表示我们收集的每个水果的名称，值表示我们收集了多少个特定的水果。
// 篮子散列图中已经有三种水果:苹果(4)、芒果(2)和荔枝(5)。
// 你必须把水果放进篮子里，这样每一种水果至少有一种，总数不超过11种——我们要养活很多张嘴。
// 你不能再用insert()把这些水果塞进去了!
//
// Make me pass the tests!
//
// Execute `rustlings hint hashmaps2` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;

#[derive(Hash, PartialEq, Eq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];

    for fruit in fruit_kinds {
        // TODO: Insert new fruits if they are not already present in the
        // basket. Note that you are not allowed to put any type of fruit that's
        // already present!
        // 待办事项:如果篮子里没有水果，就放入新的水果。注意，你不能把任何已经存在的水果放进去!
        basket.entry(fruit).or_insert(1);
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Don't modify this function!
    fn get_fruit_basket() -> HashMap<Fruit, u32> {
        let mut basket = HashMap::<Fruit, u32>::new();
        basket.insert(Fruit::Apple, 4);
        basket.insert(Fruit::Mango, 2);
        basket.insert(Fruit::Lychee, 5);

        basket
    }

    #[test]
    fn test_given_fruits_are_not_modified() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        assert_eq!(*basket.get(&Fruit::Apple).unwrap(), 4);
        assert_eq!(*basket.get(&Fruit::Mango).unwrap(), 2);
        assert_eq!(*basket.get(&Fruit::Lychee).unwrap(), 5);
    }

    #[test]
    fn at_least_five_types_of_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count_fruit_kinds = basket.len();
        assert!(count_fruit_kinds >= 5);
    }

    #[test]
    fn greater_than_eleven_fruits() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        let count = basket.values().sum::<u32>();
        assert!(count > 11);
    }
    
    #[test]
    fn all_fruit_types_in_basket() {
        let mut basket = get_fruit_basket();
        fruit_basket(&mut basket);
        for amount in basket.values() {
            assert_ne!(amount, &0);
        }
    }
}
