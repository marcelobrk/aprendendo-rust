fn main() {
    const PONTOS_MAXIMOS: u32 = 100_000;
    let mut x = 5;
    println!("O valor de x é: {}", x);
    x = 6;
    println!("O valor de x é: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("O valor de x é: {}", x);

    let guess: u32 = "42".parse().expect("Não é um número!");

    // Tuplas
    let tup = (500, 6.4, 1);

    /*
      x = 500,
      y 6.4
      z = 1
    */

    // Desestruturando
    let (x, y, z) = tup;

    println!("O valor de x é: {x}");

    // Acessando através do index
    println!("O valor do index 0 de tup é: {}", tup.0);

    //Arrays

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    //Cinco elementos do tipo i32
    let mut a: [i32; 5] = [1, 2, 3, 4, 5];

    //Cinco elementos do tipo i32
    let a = [3; 5]; é o mesmo que let a = [3, 3, 3, 3, 3];

    println!("Valor de a é: {}", a[1])
}
