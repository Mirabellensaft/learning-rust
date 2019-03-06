fn main() {
    //no variable is in scope
    let variable_a = gives_ownership(); // variable_a comes into scope
    let variable_b = String::from("Huhu"); // variable_b comes into scope
    let mut variable_c = takes_and_gives_back(variable_b); // variable_c comes into scope, variable_b is moved into variable_c and comes out of scope
    let variable_d = calculate_length(&variable_c);

    let r1 = &variable_c;
    let r2 = &mut variable_c;


    change (&mut variable_c);
    println!("{}, {}", r1, r2);
}

fn gives_ownership () -> String {

    let just_a_string = String::from("Hello");
    just_a_string
}

fn takes_and_gives_back (another_string: String) -> String {

    another_string
}

fn calculate_length (s: &String) -> usize {
    let length = s.len();
    length
}

fn change(a_string: &mut String) -> &mut String {
    a_string.push_str(", world.");

    a_string
}
