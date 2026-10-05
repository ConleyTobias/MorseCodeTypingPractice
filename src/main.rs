use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::sync::LazyLock;
use std::time::Instant;
use inquire::Select;
use rand::RngExt;
use inquire::ui::{RenderConfig, Styled};

// Static Variables
static MORSE: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
        HashMap::from([
            // Letters
            (".-", "A"),
            ("-...", "B"),
            ("-.-.", "C"),
            ("-..", "D"),
            (".", "E"),
            ("..-.", "F"),
            ("--.", "G"),
            ("....", "H"),
            ("..", "I"),
            (".---", "J"),
            ("-.-", "K"),
            (".-..", "L"),
            ("--", "M"),
            ("-.", "N"),
            ("---", "O"),
            (".--.", "P"),
            ("--.-", "Q"),
            (".-.", "R"),
            ("...", "S"),
            ("-", "T"),
            ("..-", "U"),
            ("...-", "V"),
            (".--", "W"),
            ("-..-", "X"),
            ("-.--", "Y"),
            ("--..", "Z"),
            // Numbers
            ("-----", "0"),
            (".----", "1"),
            ("..---", "2"),
            ("...--", "3"),
            ("....-", "4"),
            (".....", "5"),
            ("-....", "6"),
            ("--...", "7"),
            ("---..", "8"),
            ("----.", "9"),
            // Punctuation
            (".-.-.-", "."),
            ("--..--", ","),
            ("..--..", "?"),
            (".----.", "'"),
            ("-.-.--", "!"),
            ("-..-.", "/"),
            ("-.--.", "("),
            ("-.--.-", ")"),
            (".-...", "&"),
            ("---...", ":"),
            ("-.-.-.", ";"),
            ("-...-", "="),
            (".-.-.", "+"),
            ("-....-", "-"),
            ("..--.-", "_"),
            (".-..-.", "\""),
            ("...-..-", "$"),
            (".--.-.", "@"),
        ])
    });

fn main() {
    // Variables
    let starting_words: u8 = 50;

    // Menu items
    let menu_logo = "
     _____                    _____       _        _____         _            _____         _
    |     |___ ___ ___ ___   |     |___ _| |___   |_   _|_ _ ___|_|___ ___   |_   _|___ ___| |_
    | | | | . |  _|_ -| -_|  |   --| . | . | -_|    | | | | | . | |   | . |    | | | -_|_ -|  _|
    |_|_|_|___|_| |___|___|  |_____|___|___|___|    |_| |_  |  _|_|_|_|_  |    |_| |___|___|_|
                                                        |___|_|       |___|
    ";
    let menu_options = vec!["Start", "Exit"];

    // Styling config
    let mut config = RenderConfig::default();
    config.highlighted_option_prefix = Styled::new("•");
    config.prompt_prefix = Styled::new("");

    // Create menu
    let selection = Select::new(menu_logo, menu_options)
        .with_render_config(config)
        .prompt()
        .unwrap();

    // Handle menus
    if selection == "Start" {
        start(starting_words);
    } else {
        return;
    }
}

fn start(starting_words: u8) {
    let line: String = "-----------------".to_string();
    clearscreen::clear().unwrap();
    let test = gen_test(starting_words);
    let mut input_text = String::new();
    let start = Instant::now();

    while start.elapsed().as_secs() < 60 {
        // print!("\x1B[2J\x1B[H");
        clearscreen::clear().unwrap();
        println!("{}", test);
        print!("{}", line);
        print!("{}", start.elapsed().as_secs());
        println!("{}", line);
        println!("{}", input_text);
        io::stdout().flush().unwrap(); // Ignore previous output

        io::stdin().read_line(&mut input_text).unwrap();
        input_text = input_text.trim_end().to_string();
        input_text = replace_keys_with_values(&*input_text, &*MORSE);
    }

    clearscreen::clear().unwrap();
    main();
}

fn replace_keys_with_values(input: &str, replacements: &HashMap<&str, &str>) -> String {
    let mut pairs: Vec<(&str, &str)> = replacements.iter().map(|(k, v)| (*k, *v)).collect();
    pairs.sort_by_key(|(k, _)| std::cmp::Reverse(k.len()));

    let mut result = input.to_string();
    for (key, value) in pairs {
        result = result.replace(key, value);
    }
    result
}
fn gen_test(words: u8) -> String {
    let mut string: String = String::new();

    let mut rng = rand::rng(); // initialize rng
    for _ in 1..words {
        string += &*rand_word(&mut rng);
        string += " ";
    }

    string
}

fn rand_word(rng: &mut impl RngExt) -> String {
    top_english_words::get_word(rng.random_range(0..100)).unwrap()
}
