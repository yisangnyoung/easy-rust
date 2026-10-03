use std::io;

fn main() {
    println!("Please type something, or x to escape: ");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        // 이 부분이 제대로 동작하지 않습니다.

        // 먼저 문자열을 지웁니다. 그렇지 않으면 계속 추가합니다.
        input_string.clear();

        // 사용자에게서 표준 입력(stdin)을 가져와서 read_string에 넣습니다.
        io::stdin().read_line(&mut input_string).unwrap();

        println!("You wrote {}", input_string);
    }
    println!("See you later!");
}
