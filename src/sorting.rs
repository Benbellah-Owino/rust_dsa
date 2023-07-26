
pub fn bubble_sort(array: &mut Vec<i32>) {
    println!("{:?}", array);

    for number in (0..array.len()) {
        for i in 0..array.len() {
            if i + 1 != array.len() && array[i] > array[i + 1] {
                let temp: i32 = array[i + 1];
                array[i + 1] = array[i];
                array[i] = temp;
            }
        }
    }
    println!("{:?}\n\n", array);
}
