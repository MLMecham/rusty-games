# Overview

This project features an app designed to let the users play classic games from their childhood and compare their scores with their friends.
This project was developed to learn more about Rust, cloud database manangement, game logic, and rust GUI, and other rust API.
The app allows users to:
* Create accounts
* Log into existing accounts
* Play a number of classic games including hangman.
* Query the real time database to show the users with the highest scores.

# Development Environment

The app was built using the following tools and technologies:
* Rust programming language for its fast async functions.
* Rust cargo to manage the packages and project.
* Mongodb atlas to host our database on the cloud
* Ninja API to give a random word for our games
* Tokio async package, to allow our main function to work asychronisouly and cleanly.
* VS Code: As the primary code editor for development.
* Serde to work with JSON data from the internet
* Plus much more.

Programming language and libraries:
* Language: Rust.
Libraries:
* serde for JSON serialization and deserialization
* futures
* tokio asyncbufreadext for input and bufReader
* Standard library Error package
* Mongodb client
* Standard collection Hashset

# Useful Websites

* [React Native Official Site](https://reactnative.dev/)
* [v0 by Vercel, 0 to 1 app Generator](https://v0.dev/)
* 


# Future Work


* Enhanced Styling: Improve the UI/UX to make the app more visually appealing and user-friendly.
* Notification System: More games including; black jack, snake, tetris, battleship, and more.
* Live Server: The mongodb only works on specified IP addresses, if we make a server that does all the database connections, we can have people play on clients from anywhere. 




































## Welcome to Rusty-Games!


This project was designed to help us learn the wonderful language rust. 


### What you will find

1. A fully functional connection to a mongodb database that updates in real time. This allows users to create accounts, log in, and save the points they earn from games.

2. A working hangman game that uses ninja API to randomly select a word. This function also takes advantage of rusts robust error handling to provide a word from our word bank should the API be down.

3. A GUI made to make the game more interactable.


### Things that will be added in the future.

1. A remote server so that the game can be played from anywhere with an internet connection. The issue right now is that mongodb requires that the device running the code have its IP stored in settings, meaning that if anybody wishes to play, they must be a mongodb admin. That is kind of worrying.

2. Some games that we will add in the future include: A four player backjack, Minesweeper, Snake, Tetris, and battleship. 
