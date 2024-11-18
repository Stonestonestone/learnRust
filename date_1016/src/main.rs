fn main() {
    let n = 10; // 可以更改为需要的斐波那契数列的长度
    let mut fib_values = vec![0, 1]; // 初始化前两个斐波那契数

    println!("Fibonacci series up to {}: ", n);

    // 生成斐波那契数列
    for i in 2..=n {
        let next_value = fib_values[i - 1] + fib_values[i - 2]; // 计算下一个斐波那契数
        fib_values.push(next_value); // 将计算出的值添加到向量中
    }

    // 打印每一行的斐波那契数列
    for i in 0..=n {
        println!("{:?}", &fib_values[0..=i]); // 打印从0到当前索引的所有值
    }
}