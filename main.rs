use std::io::{self, Read};

fn main() {
    let mut buffer = [0u8; 1]; // 创建一个字节的缓冲区
    print!("Enter the size of map, (and good luck): ");

    loop {
        io::stdin()
            .read(&mut buffer)
            .expect("Failed to read character");

        // 将字节转换为字符
        let input_char = buffer[0] as char;

        // 打印输入的字符
        print!("{}\r", input_char);

        // 检查是否是要退出的字符
        if input_char == 'q' {
            break; // 退出循环
        }
    }
}