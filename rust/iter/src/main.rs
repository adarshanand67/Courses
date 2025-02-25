// Iterators
#![allow(unused)]
fn print_vector(v: &[String]) {
    // for elem in v.iter() {
    //     println!("{}", elem);
    // }
    // iterator consumer
    v.iter().for_each(|elem| println!("{}", elem));

    // iterator adapter
    // v.iter()
    //     .map(|elem| elem.to_uppercase())
    //     .for_each(|elem| println!("{}", elem));
}

fn shorten_strings(v: &[String]) {
    v.iter().for_each(|elem| {
        let first_char = elem.chars().next();
        match first_char {
            Some(c) => println!("{}", c),
            None => println!("No first character found"),
        }
    });
    // mutable Iterators
}

fn to_uppercase(v: &[String]) -> Vec<String> {
    v.iter()
        .map(|elem| elem.to_uppercase())
        .collect::<Vec<String>>()
}

fn main() {
    let colors = Vec::from([
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ]);

    // let mut colors_iter = colors.iter();
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());
    // println!("{:#?}", colors_iter.next());

    // print_vector(&colors);
    // print_vector(&colors[1..3]);

    // shorten_strings(&colors);

    let upper_colors = to_uppercase(&colors);
    print_vector(&upper_colors);
}
