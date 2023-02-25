//

pub fn main() {
    let mut v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];// infer
    v.push(5);
    v.push(6);
    // let third: &i32 = &v[2]; // insecure
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
    v.push(6);
    for midnigth in &mut v {
        *midnigth += 10;
        print!("{midnigth} ");
    }
    println!("{:?}", v);
    let vec = vec![1, 2, 3, 4];
    let new_vec: Vec<i32> = vec.iter().map(|c| c + 1).collect();
    // let new_vec: Vec<i32> = vec.into_iter().filter(|c| c % 2 == 0).collect();
    println!("{:?}", new_vec);
}
