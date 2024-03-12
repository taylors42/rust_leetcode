fn main() {
    
}


pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    for i in 1..nums.len(){
    let index: i32 = i.try_into().unwrap();
    if nums[i - 1] + nums[i] == target{
        output.push(index - 1);
        output.push(i.try_into().unwrap());
        }
    }
    output
}
