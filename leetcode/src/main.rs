use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap: HashMap<i32, i32> = HashMap::new();
        for i in 0..nums.len() {
            let complement = target - nums[i];
            if hashmap.contains_key(&complement) {
                return vec![i as i32, *hashmap.get_key_value(&complement).unwrap().1];
            }
            hashmap.insert(nums[i], i as i32);
        }
        vec![]
    }
}
