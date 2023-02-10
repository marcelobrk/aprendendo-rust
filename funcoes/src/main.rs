fn main() {
    println!("Hello, world!");

    teste();
    teste2();
}

fn teste() -> i32 {
    let x = {
        let x = 3;
        x + 1
    };

    println!("O valor de x é: {}", x);
    return x;
}

fn teste2() {
    let number = 3;

    if number < 7 {
        println!("Condicao é true");
    } else {
        println!("Condicao é false");
    }
}
