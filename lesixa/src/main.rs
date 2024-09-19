fn main() {
    //let (s1 ,len) = cal_len(String::from("hello"));
    let s1 = String::from("hello");
    let len= cal_len(&s1);
    println!("the len is '{}' of {}",s1,len);
}

fn cal_len(s: &String) -> usize {
    s.len()
}
