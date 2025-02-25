fn next_language<'a>(language: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in language {
        if found {
            return lang;
        }
        if lang.to_lowercase() == current.to_lowercase() {
            found = true;
        }
    }
    language.last().unwrap()
}

fn last_language<'a>(language: &'a [String]) -> &'a str {
    language.last().unwrap()
}

fn longest_language<'a>(language1: &'a str, language2: &'a str) -> &'a str {
    if language1.len() > language2.len() {
        language1
    } else {
        language2
    }
}

fn main() {
    let languages = vec![
        String::from("Rust"),
        String::from("Go"),
        String::from("Typescript"),
        String::from("Javascript"),
    ];

    let result = next_language(&languages, "go");
    println!("Next language is {}", result);

    let last = last_language(&languages);
    println!("Last language is {}", last);

    let longest = longest_language("Rust", "Lol");
    println!("Longest language is {}", longest);
}
