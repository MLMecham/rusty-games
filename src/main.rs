use mongodb::{bson::doc, options::ClientOptions, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt, BufReader};
use futures::stream::TryStreamExt;
mod clear;
mod hangman;
mod dictionary;
use dictionary::get_word;
use hangman::run_hangman;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    _id: String,  // Username as the ID
    password: String,
    points: i32,
}

async fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin);
    reader.read_line(&mut input).await.unwrap();
    input.trim().to_string()
}

fn sanitize_username(input: &str) -> String {
    input.to_lowercase().chars().filter(|c| c.is_ascii_lowercase()).collect()
}

async fn find_user(collection: &mongodb::Collection<User>, username: &str) -> Result<Option<User>, Box<dyn Error>> {
    let filter = doc! { "_id": username };
    let user = collection.find_one(filter).await?;
    Ok(user)
}

async fn get_user(collection: &mongodb::Collection<User>, username: &str) -> Result<Option<User>, Box<dyn Error>> {
    let filter = doc! { "_id": username };
    let user = collection.find_one(filter).await?;
    Ok(user) // Returns the user object (if found)
}

async fn create_user(collection: &mongodb::Collection<User>, user: &User) -> Result<(), Box<dyn Error>> {
    if find_user(collection, &user._id).await?.is_some() {
        return Err("Username already exists!".into());
    }

    collection.insert_one(user).await?;
    Ok(())
}

async fn update_user_points(collection: &mongodb::Collection<User>, username: &str, new_points: i32,) -> Result<(), Box<dyn Error>> {
    let filter = doc! { "_id": username }; // Find user by _id (username)
    let update = doc! { "$set": { "points": new_points } }; // Set new points value

    let result = collection.update_one(filter, update).await?;

    if result.matched_count == 0 {
        return Err("User not found!".into());
    }

    println!("Updated {} user(s)", result.modified_count);
    Ok(())
}

async fn update_user_password(
    collection: &mongodb::Collection<User>,
    username: &str,
    new_password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let filter = doc! { "_id": username };
    let update = doc! { "$set": { "password": new_password } };

    let result = collection.update_one(filter, update).await?;
    
    if result.matched_count == 0 {
        return Err("User not found!".into());
    }

    println!("Password updated for {}!", username);
    Ok(())
}


async fn authenticate_user(
    collection: &mongodb::Collection<User>,
    username: &str,
    password: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    if let Some(user) = collection.find_one(doc! { "_id": username }).await? {
        return Ok(user.password == password);
    }
    Ok(false)
}


async fn delete_user(collection: &mongodb::Collection<User>, username: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = collection.delete_one(doc! { "_id": username }).await?;
    
    if result.deleted_count == 0 {
        return Err("User not found!".into());
    }

    println!("User {} deleted!", username);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // MongoDB connection string
    let uri = "mongodb+srv://mechamit000:mecham123@cluster0.imffk.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";

    // Connect to MongoDB
    let client_options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(client_options)?;

    println!("Connected to MongoDB!");

    // Access the database and collection
    let db = client.database("rusty"); // Database name
    let collection = db.collection::<User>("user"); // Collection name

    //active user
    let mut active_user: Option<User> = None;

    //activate reader for tokio main
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin).lines();



    loop {
        println!("Welcome to Rusty Games!");
        println!("1. Log In");
        println!("2. Create Account");
        println!("3. Continue as Guest");
        println!("4. Quit");
        println!("5. Show Dictionary Word");
    
        let choice = reader.next_line().await?.unwrap_or_default().trim().to_string();
    
        match choice.as_str() {
            "1" => {
                println!("Enter your username:");
                let username = reader.next_line().await?.unwrap_or_default().trim().to_lowercase();
                
                println!("Enter a password:");
                let mut password = reader.next_line().await?.unwrap_or_default().trim().to_string();

                while password.is_empty() || password.contains(' ') {
                    println!("Password cannot be blank or contain spaces. Try again:");
                    password = reader.next_line().await?.unwrap_or_default().trim().to_string();
                }
                
                match authenticate_user(&collection, &username, &password).await {
                    Ok(true) => {
                        if let Ok(Some(user)) = get_user(&collection, &username).await {
                            active_user = Some(user);
                            println!("Login successful!");
                        }
                    }
                    Ok(false) => println!("Invalid credentials."),
                    Err(e) => println!("Error: {}", e),
                }
            }
            "2" => {
                println!("Creating account...");
                println!("Enter a username (lowercase only, cannot be changed):");
    
                let mut username = reader.next_line().await?.unwrap_or_default().trim().to_lowercase();
                while username.is_empty() || username.contains(' ') || username.chars().any(|c| !c.is_ascii_lowercase()) {
                    println!("Usernames must contain only lowercase letters, no spaces, and cannot be blank. Try again:");
                    username = reader.next_line().await?.unwrap_or_default().trim().to_lowercase();
                }
    
                println!("Enter a password:");
                let password = reader.next_line().await?.unwrap_or_default().trim().to_string();
    
                let new_user = User {
                    // leave the clone on username for now. it could be used to cross check the database
                    _id: username.clone(),
                    password,
                    points: 0,
                };
    
                match create_user(&collection, &new_user).await {
                    Ok(_) => {
                        active_user = Some(new_user);
                        println!("Account created successfully! Your username can never be changed.");
                    }
                    Err(e) => {
                        println!("Error: {}", e); // Now this will print "Username already exists!"
                    }
                }
            }
            "3" => {
                println!("Continuing as guest...");
                active_user = Some(User {
                    _id: "Guest".to_string(),
                    password: "".to_string(),
                    points: 0,
                });
            }
            "4" => {
                println!("Goodbye!");
                return Ok(());
            }
            "5" => {
                match get_word().await{
                    Ok(word) => { println!("Random word: {}", word);}
                    Err(e) =>{ println!("Error retrieving word: {}", e);}
                }
            },
            _ => {
                println!("Invalid choice. Try again.");
            }
        }
    
        while let Some(user) = &active_user {
            clear::clear_terminal().unwrap();
    
            println!("Welcome, {}! You have {} points.\nWhat would you like to do?", user._id, user.points);
            println!("1. Play game");
            println!("2. View leaderboard");
            println!("3. Settings");
            println!("4. Log out");
    
            let game_choice = reader.next_line().await?.unwrap_or_default().trim().to_string();
    
            match game_choice.as_str() {
                "1" => {
                    println!("Starting the game...");
                    let game_score: i32 = run_hangman();
                    if let Some(user) = active_user.as_mut() { // Get a mutable reference to active_user
                        user.points += game_score;
                        update_user_points(&collection, &user._id, user.points).await.expect("Failed to update points");
                    }

                    

                },
                "2" => {
                    println!("Fetching leaderboard...");

                    let pipeline = vec![
                    doc! { "$sort": { "points": -1 } }, // Sort by points in descending order
                    doc! { "$limit": 3 } // Take only the top 3 players
                ];

                let mut cursor = collection.aggregate(pipeline).await?;

                println!("🏆 Leaderboard 🏆");
                let mut rank = 1;
                while let Some(player) = cursor.try_next().await? {
                    println!("{}. {} - {} points", rank, player.get("_id").unwrap(), player.get("points").unwrap());
                    rank += 1;
                }

                println!("\nPress enter to continue...");
                let mut input = String::new();
                let mut stdin = io::BufReader::new(io::stdin());
                stdin.read_line(&mut input).await?;

                },
                "3" => {
                        println!("Opening Settings...");

                        while let Some(user) = active_user.as_ref() {
                            println!("Settings Menu:");
                            println!("1. View User Info");
                            println!("2. Update Password");
                            println!("3. Delete Account");
                            println!("4. Back to Main Menu");

                            let settings_choice = reader.next_line().await?.unwrap_or_default().trim().to_string();

                            match settings_choice.as_str() {
                                "1" => {
                                    println!("User Info:");
                                    println!("Username: {}", user._id);
                                    println!("Points: {}", user.points);
                                }
                                "2" => {
                                    println!("Enter new password (no spaces, cannot be blank):");
                                    let mut new_password = reader.next_line().await?.unwrap_or_default().trim().to_string();

                                    while new_password.is_empty() || new_password.contains(' ') {
                                        println!("Password cannot be blank or contain spaces. Try again:");
                                        new_password = reader.next_line().await?.unwrap_or_default().trim().to_string();
                                    }

                                    if let Err(e) = update_user_password(&collection, &user._id, &new_password).await {
                                        println!("Error updating password: {}", e);
                                    } else {
                                        println!("Password successfully updated.");
                                    }
                                }
                                "3" => {
                                    println!("Are you sure you want to delete your account? Type 'yes' to confirm:");
                                    let confirmation = reader.next_line().await?.unwrap_or_default().trim().to_lowercase();

                                    if confirmation == "yes" {
                                        if let Err(e) = delete_user(&collection, &user._id).await {
                                            println!("Error deleting account: {}", e);
                                        } else {
                                            println!("Account deleted successfully.");
                                            active_user = None; // Log out user after deletion
                                            break; // Exit settings
                                        }
                                    } else {
                                        println!("Account deletion canceled.");
                                    }
                                }
                                "4" => {
                                    println!("Returning to main menu...");
                                    break; // Exit settings loop
                                }
                                _ => println!("Invalid choice. Try again."),
                            }
                        }
                    },

                // Back to game menu
                "4" => {
                    println!("Logging out...");
                    active_user = None;
                }
                _ => println!("Invalid choice. Try again."),
            }
        }
    }
    




  

    
}













