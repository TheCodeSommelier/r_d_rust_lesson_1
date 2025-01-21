use std::io;

fn main() {
    println!("Enter your name:");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim().to_string();

    init_chat_loop(name);
}

fn init_chat_loop(name: String) {
    let mut index = 0;
    let consent = get_consent();
    let mut answers = Vec::new();
    answers.push(name);

    while consent {
        println!("{}", ask_question(index));
        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        answers.push(answer.trim().to_string());

        if index >= 3 {
            print_answers(answers);
            break;
        }

        index += 1;
    }
}

fn get_consent() -> bool {
    println!("Would you like to give us more?");

    let mut consent = String::new();

    io::stdin()
        .read_line(&mut consent)
        .expect("Well that did not work...");

    let consent = consent.trim();

    if consent == "Yes" || consent == "Y" {
        return true;
    } else {
        return false;
    }
}

fn ask_question(index: usize) -> &'static str {
    if index < 3 {
        let questions = vec![
            "How old are you?",
            "What exactly is your take on google?",
            "Did you know, that an ostrich is not my favourite animal?",
        ];
        return questions[index];
    } else {
        return "Yea... I don't want to know anything else... Just come back later...";
    }
}

fn print_answers(answers: Vec<String>) {
    let mut index = 1;
    while index != answers.len() {
        println!("Answer no. {}: {}", index, answers[index]);
        index += 1;
    }
    println!("Not bad... 10/10")
}
