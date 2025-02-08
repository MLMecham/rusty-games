use eframe::egui;
use std::collections::HashSet;
mod style;
struct HangmanGame {
    secret_word: String,
    guessed_letters: HashSet<char>,
    remaining_attempts: u8,
    final_guess_mode: bool,
    final_guess_input: String,
    final_score: i32,

}

impl HangmanGame {
    const HANGMAN_STAGES: [&'static str; 7] = [
        "\n        +---+\n        |   |\n            |\n            |\n            |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n            |\n            |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n        |   |\n            |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n       /|   |\n            |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n       /|\\  |\n            |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n       /|\\  |\n       /    |\n            |\n      =========",
        "\n        +---+\n        |   |\n        O   |\n       /|\\  |\n       / \\  |\n            |\n      =========",
    ];

    fn new(secret_word: &str) -> Self {
        HangmanGame {
            secret_word: secret_word.to_lowercase(),
            guessed_letters: HashSet::new(),
            remaining_attempts: 6,
            final_guess_mode: false,
            final_guess_input: String::new(),
            final_score: 0,
        }
    }

    fn display_progress(&self) -> String {
        self.secret_word
            .chars()
            .map(|c| if self.guessed_letters.contains(&c) { c } else { '_' })
            .collect()
    }

    fn guess(&mut self, c: char) -> bool {
        let c = c.to_ascii_lowercase();
        self.guessed_letters.insert(c);

        if !self.secret_word.contains(c) {
            self.remaining_attempts -= 1;
            false
        } else {
            true
        }
    }

    fn is_won(&self) -> bool {
        self.secret_word.chars().all(|c| self.guessed_letters.contains(&c))
    }

    fn is_lost(&self) -> bool {
        self.remaining_attempts == 0
    }

    fn display_hangman(&self) -> &'static str {
        HangmanGame::HANGMAN_STAGES[(6 - self.remaining_attempts) as usize]
    }

    fn process_final_guess(&mut self, guess: &str) {
        if guess.len() != self.secret_word.len() {
            self.remaining_attempts = 0;
            return;
        }

        if guess.to_lowercase() != self.secret_word {
            self.remaining_attempts = 0;
        } else {
            // Fill in all letters if correct
            for c in guess.chars() {
                self.guessed_letters.insert(c.to_ascii_lowercase());
            }
        }
    }

    // Returns the score based on the game result
    fn calculate_score(&self) -> i32 {
        if self.is_won() {
            10 // Award 10 points if the user wins
        } else {
            0 // No points if the user loses
        }
    }

    // Updates the score during the game
    pub fn update_score(&mut self, points: i32) {
        if self.is_won() {
            self.final_score += points;
        }
    }
}

enum Screen {
    Home,
    Game,
    GameOver,
}

struct MyApp {
    current_screen: Screen,
    game: Option<HangmanGame>,
    guess_input: String,
    final_score: i32,
}

impl MyApp {
    fn new() -> Self {
        MyApp {
            current_screen: Screen::Home,
            game: None,
            guess_input: String::new(),
            final_score: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let apply_style = style::styles();  // Applying styles
        ctx.set_style(apply_style);

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::Home => {
                    ui.add_space(ctx.available_rect().height() * 0.30);

                    let available_width = ctx.available_rect().width();
                    let panel_width = available_width * 0.30;

                    ui.horizontal(|ui| {
                        ui.add_space(available_width * 0.30);
                        style::homeScreenPanel().show(ui, |ui| {
                            ui.set_max_width(panel_width); // Set max width for the panel

                            ui.vertical_centered(|ui| {
                                ui.heading("Welcome to Hangman!");
                                ui.add_space(40.0);


                                if ui
                                .add(egui::Button::new("Play").min_size(egui::vec2(100.0, 30.0)))
                                .clicked() {
                                    self.game = Some(HangmanGame::new("california")); // You can randomize this
                                    self.current_screen = Screen::Game;
                                    
                                }
                                
                            });
                        });
                    });
                }
                
                Screen::Game => {
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([120.0, 40.0], egui::Button::new("Back to Home"))
                            .clicked()
                        {
                            self.current_screen = Screen::Home;
                            self.game = None;
                        }
                    });
                
                    if let Some(game) = &mut self.game {
                        ui.add_space(20.0);
                
                        // Centered game content
                        ui.vertical_centered(|ui| {
                            // Hangman ASCII art display
                            ui.monospace(game.display_hangman());
                            ui.add_space(15.0);
                
                            // Word progress
                            ui.label(egui::RichText::new(game.display_progress())
                                .size(24.0)
                                .strong());
                            ui.add_space(10.0);
                
                            // Remaining attempts
                            ui.label(egui::RichText::new(format!("Attempts left: {}", game.remaining_attempts))
                                .size(18.0)
                                .color(egui::Color32::RED));
                        });
                
                        ui.add_space(20.0);
                
                        if game.is_won() || game.is_lost() {
                            let score = game.calculate_score();
                            self.final_score += score;
                            self.current_screen = Screen::GameOver;
                        } else {
                            ui.vertical_centered(|ui| {
                                if !game.final_guess_mode {
                                    ui.label("Guess a letter:");
                                    let response = ui.add_sized([50.0, 30.0], egui::TextEdit::singleline(&mut self.guess_input));
                
                                    if response.changed() {
                                        self.guess_input = self.guess_input.chars().next().unwrap_or('\0').to_string();
                                    }
                                    
                                    ui.add_space(10.0);
                                    if ui.add_sized([100.0, 30.0], egui::Button::new("Guess")).clicked()
                                        && !self.guess_input.is_empty()
                                    {
                                        if let Some(c) = self.guess_input.chars().next() {
                                            if c.is_alphabetic() {
                                                game.guess(c);
                                                game.update_score(1);
                                            }
                                        }
                                        self.guess_input.clear();
                                    }
                                    
                                    ui.add_space(10.0);
                                    if ui.add_sized([120.0, 30.0], egui::Button::new("Final Guess")).clicked() {
                                        game.final_guess_mode = true;
                                    }
                                } else {
                                    ui.label("Enter the full word:");
                                    ui.add_sized([200.0, 30.0], egui::TextEdit::singleline(&mut game.final_guess_input));
                                    
                                    ui.add_space(10.0);
                                    if ui.add_sized([100.0, 30.0], egui::Button::new("Submit")).clicked() {
                                        let final_guess = game.final_guess_input.clone();
                                        game.process_final_guess(&final_guess);
                                        game.final_guess_mode = false;
                                        game.final_guess_input.clear();
                                    }
                                    
                                    ui.add_space(10.0);
                
                                    if ui.add_sized([100.0, 30.0], egui::Button::new("Cancel")).clicked() {
                                        game.final_guess_mode = false;
                                        game.final_guess_input.clear();
                                    }
                                }
                            });
                        }
                    }
                }
                

                Screen::GameOver => {
                    
                        ui.add_space(ctx.available_rect().height() * 0.30);
                    
                        let available_width = ctx.available_rect().width();
                        let panel_width = available_width * 0.30;
                    
                        ui.horizontal(|ui| {
                            ui.add_space(available_width * 0.30);
                            style::homeScreenPanel().show(ui, |ui| {
                                ui.set_max_width(panel_width); // Set max width for the panel
                            
                                ui.vertical_centered(|ui| {
                                    // Heading styling
                                    if let Some(game) = &self.game {
                                        if game.is_won() {
                                            ui.heading("🎉 Congratulations! You won!");
                                        } else {
                                            ui.heading("💀 Game Over!");
                                        }
                    
                                        ui.add_space(10.0);
                                        // Final score
                                        ui.label(format!("Final Score: {}", self.final_score));
                    
                                        // Word reveal and styling
                                        ui.label(format!("The word was: {}", game.secret_word));
                                    }
                    
                                    ui.add_space(20.0);
                    
                                    // Buttons for navigation
                                    if ui
                                        .add(egui::Button::new("Play Again").min_size(egui::vec2(100.0, 30.0)))
                                        .clicked()
                                    {
                                        self.game = Some(HangmanGame::new("california")); // Can randomize the word
                                        self.current_screen = Screen::Game;
                                    }
                    
                                    ui.add_space(10.0);
                    
                                    if ui
                                        .add(egui::Button::new("Back to Home").min_size(egui::vec2(100.0, 30.0)))
                                        .clicked()
                                    {
                                        self.current_screen = Screen::Home;
                                        self.game = None;
                                    }
                                });
                            });
                        });
                    
                }
            }
        });
    }
}

pub async fn run_hangman_gui() -> i32 {
    let options = eframe::NativeOptions::default();
    let app = MyApp::new();
    eframe::run_native(
        "Hangman",
        options,
        Box::new(|_cc| Box::new(MyApp::new()))
    );
    
    app.final_score // Returns the final score after the game ends???
    
}
