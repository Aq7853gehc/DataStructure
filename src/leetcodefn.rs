#[allow(dead_code)]
pub fn remove_elements(num: &mut Vec<i32>, val: i32) -> i32 {
    let mut result: Vec<i32> = vec![];
    for &i in num.iter() {
        result.push(i);
    }
    for _i in 0..num.len() {
        num.pop();
    }
    for j in 0..result.len() {
        if result[j] != val {
            num.push(result[j]);
        }
    }
    let k = num.len() as i32;
    k
}

#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let mut index: Option<i32> = None;

    for (i, num) in nums.iter().enumerate() {
        if *num == target {
            index = Some(i as i32);
            break;
        }

        if index.is_none() && *num > target {
            index = Some(i as i32);
            break;
        }
    }

    if index.is_none() {
        index = Some(nums.len() as i32);
    }

    index.unwrap()
}

#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut result = digits.clone(); // Create a clone to avoid modifying the original vector
    let mut carry = 1; // Start with a carry of 1 to add

    for i in (0..result.len()).rev() {
        let new_val = result[i] + carry;
        if new_val == 10 {
            result[i] = 0;
            carry = 1; // Carry over the 1
        } else {
            result[i] = new_val;
            carry = 0; // No more carry, so break early
            break;
        }
    }

    if carry == 1 {
        result.insert(0, 1); // If there's still a carry, insert 1 at the front
    }

    result
}

#[allow(dead_code)]
pub fn merge(num1: &mut Vec<i32>, m: i32, num2: &mut Vec<i32>, n: i32) {
    
}
