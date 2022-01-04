enum Color {
    Red,
    Blue,
    Green,
}

fn main() {
    let maybe_user = Some("Xulho");
    if let Some(user) = maybe_user {
        println!("user={:?}", user);
    } else {
        println!("no user");
    }

    let red = Color::Red;
    if let Color::Red = red {
        println!("its red!");
    } else {
        println!("its not red");
    }
}
