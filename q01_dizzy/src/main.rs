// Question: https://github.com/jadijadi/Practical-Programming-Practices/tree/main/strings/tfcctf2023-dizzy


fn main() {
    let encrypted = "T4 l16 _36 510 _27 s26 _11 320 414 {6 }39 C2 T0 m28 317 y35 d31 F1 m22 g19 d38 z34 423 l15 329 c12 ;37 19 h13 _30 F5 t7 C3 325 z33 _21 h8 n18 132 k24";
    let separated_items: Vec<&str> = encrypted.split(' ').collect();

    let mut code = ['\0'; 100];
    for item in separated_items {
        let character: char = item.chars().next().unwrap();
        let index: usize = item[1..].parse().unwrap();
        code[index] = character;
    }

    let decrypted: String = code.iter().collect();
    println!("{}", decrypted);
}
