pub fn two_sum(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    for (i, n) in nums.iter().enumerate() {
        for (j, m) in nums.iter().enumerate() {
            if i != j && n + m == target {
                return vec![i as i32, j as i32];
            }
        }
    }

    return vec![0, 0];
}

pub fn two_sum_2(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;
    let mut contents: HashMap<i32, i32> = HashMap::new();

    for (i, n) in nums.iter().enumerate() {
        contents.insert(*n as i32, i as i32);
    }

    for (i, n) in nums.iter().enumerate() {
        let tgt = target - n;
        if contents.contains_key(&tgt) {
            let j = contents[&tgt];
            if i != j as usize {
                return vec![i as i32, j as i32];
            }
        }
    }

    vec![0 as i32, 0 as i32]
}

pub fn two_sum_ultimate(nums: &Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut m: HashMap<i32, i32> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        match m.get(&(target - *v)) {
            Some(&i2) => return vec![i as i32, i2],
            None => m.insert(*v, i as i32),
        };
    }
    vec![]
}

#[test]
fn test_it() {
    let nums = vec![2, 7, 11, 15];
    const TARGET: i32 = 9;

    let result2 = two_sum(&nums, TARGET);
    assert_eq!(result2, vec![0, 1]);

    let result2 = two_sum_2(&nums, TARGET);
    assert_eq!(result2, vec![0, 1]);

    let result3 = two_sum_ultimate(&nums, TARGET);
    assert_eq!(result3, vec![1, 0]);
}
