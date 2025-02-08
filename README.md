# Hangman Game in Rust

## Overview

This project is a command-line and GUI-based Hangman game built using Rust and `eframe` (egui framework). The goal of the project is to enhance skills in Rust programming, GUI development, and state management while creating an engaging and interactive word-guessing game.

### Features

- **Command-line Mode:**
  - Players can choose from the menu if they wish to log in, create an account our continue as a guest.
  - Players can choose from the menu if they wish to play game, see leaderboard, setting, or quit the program.
  - The play game option trigger the GUI and the user is presented with it to play.
  
- **GUI Mode (eframe/egui):**
  - A graphical interface allows users to play Hangman interactively.
  - Players can guess individual letters or submit a full word guess.
  - The interface updates dynamically to show the hangman drawing and guessed word progress.
  - Score tracking based on remaining attempts and word length.
  
- **Scoring System:**
  - A correct final guess grants full points (10), while running out of attempts results in a score of 0.
  
## Development Environment

The project is built using:

- **Rust**: The main programming language for both CLI and GUI versions.
- **eframe/egui**: For building the graphical interface.
- **tokio**: To handle asynchronous operations in the main function.
- **std::collections::HashSet**: To manage guessed letters efficiently.

## How to Run

### Running in the Command-Line Version:
```sh
cargo run
```


## Future Enhancements

- **Word API Integration:** Fetch random words from an API instead of hardcoding them.
- **Difficulty Levels:** Implement easy, medium, and hard modes with varying word lengths and attempts.
- **Theming & Animations:** Improve GUI design with themes and animations for a better user experience.

This project serves as a learning experience in Rust, GUI development, and game logic implementation.
