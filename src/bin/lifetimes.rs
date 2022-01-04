#[derive(Debug)]
enum Answer {
    Yes,
    No,
}

#[derive(Debug)]
struct Form<'a> {
    question: &'a Answer,
}

fn main() {
    let answer = Answer::Yes;

    let form = Form { question: &answer };

    println!("{:?}", form);
}

#[derive(Debug)]
struct Quiz {
    question: Answer,
}

fn get_first_quiz<'a>(quis_1: &'a Quiz, quis_2: &Quiz, quis_3: &Quiz) -> &'a Answer {
    &quis_1.question
}
