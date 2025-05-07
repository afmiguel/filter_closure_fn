fn odd_filter(v: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in v {
        if i % 2 != 0 {
            result.push(*i);
        }
    }
    result
}

fn even_filter(v: &Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for i in v {
        if i % 2 == 0 {
            result.push(*i);
        }
    }
    result
}


fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = odd_filter(&v);
    println!("Odd numbers = {:?}", result); // Output: [1, 3, 5]

    let result = even_filter(&v);
    println!("Even numbers = {:?}", result); // Output: [2, 4]
}
