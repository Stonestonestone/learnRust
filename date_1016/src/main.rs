fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            remaining -= 1;
            
            if count == 2 {
                break 'counting_up;
            }
        }

        count += 1;
    }
    println!("End count = {}", count);
}
