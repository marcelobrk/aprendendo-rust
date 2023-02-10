fn main() {
    let mut s1 = String::from("hello");

    println!("Original value of s1: {s1}");
    receive_ownership(&mut s1);
    println!("Value of s1 after change by receive_ownershipt: {s1}");

    receive_and_return_ownership(s1.clone());

    println!("Value of s1 after passed to receive_and_return_ownership: {s1}");
}

fn receive_ownership(some_string: &mut String) {
    some_string.push_str(", world");
}

fn receive_and_return_ownership(some_string: String) {}
