//fn main() {
//    println!("Hello, world!");
//}
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    //定义一个不可变变量，整数型，从随机数1-101之间随机生成一个数
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //定义一个可变变量，为空的字符串。用来存储每次猜测值
    let mut guess = String::new();
    //使用循环，可以通过状态反复猜测随机数
    loop{
        println!("Please input your guess:");
        //从键盘读取输入， read_line 将用户输入附加到传递给它的字符串中，赋值给guess变量，返回Result
        // Result实例拥有 expect 方法，实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        //将guess的string转为32 位无符号数字 。trim 去除字符串开头和结尾的空白字符，parse将字符串解成数字
        //添加错误处理，Result返回包含结果的Ok，返回数字。处理错误返回Err，_为通配符
        let guess:u32 = match guess.trim().parse() {
         Ok(num)=>num,
         Err(_)=>continue,
        };
        println!("You guessed: {}", guess);
        //match的cmp方法比较两个值
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //如果相等，从循环跳出
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    println!("The secret number is: {}", secret_number);
}