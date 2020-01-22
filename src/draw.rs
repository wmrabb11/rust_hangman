pub fn draw_hangman(remaining: i32) {
    match remaining {
        6 => println!("
  +---+
  |   |
      |
      |
      |
      |
=========
                      "),
        5 => println!("
  +---+
  |   |
  O   |
      |
      |
      |
=========
                      "),
        4 => println!("
  +---+
  |   |
  O   |
  |   |
      |
      |
=========
                      "),
        3 => println!("
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========
                      "),
        2 => println!("
  +---+
  |   |
  O   |
 /|\\  |
      |
      |
=========
                      "),
        1 => println!("
  +---+
  |   |
  O   |
 /|\\  |
 /    |
      |
=========
                      "),
        0 => println!("
  +---+
  |   |
  O   |
 /|\\  |
 / \\  |
      |
=========
                      "),
        _ => println!("[-] Invalid remaining guesses. Must be between 0-6"),
    }
}
