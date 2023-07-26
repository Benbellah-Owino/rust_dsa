pub fn binary_search(number: i32, array: &Vec<i32>) -> i32 {
    let mut left: usize = 0;
    let mut right: usize = array.len() - 1;
    let mut mid_point: usize = (right + left) / 2;

    while left < right {
        mid_point = (right + left) / 2;
        if (number > array[mid_point]) {
            left = mid_point + 1;
        } else {
            right = mid_point;
        }
        if (number == array[left]) {
            return left as i32;
        } else if (array.len() == 0) {
            return -1;
        }
    }
    return -1;
}

pub fn linear_search(number: i32, array: &Vec<i32>) -> Option<i32> {
    let mut counter = 0;
    for i in array {
        if *i == number {
            return Some(counter);
        }
        counter += 1;
    }

    return None;
}
