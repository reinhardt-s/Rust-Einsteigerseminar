# Final Project: The Crust of Rust Game

<center>
    <img src="crust_of_rust.png" alt="drawing" width="150"
    style="border-radius: 25px; box-shadow: 0 4px 8px 0 rgba(0, 0, 0, 0.2), 0 6px 20px 0 rgba(0, 0, 0, 0.19);" />
</center>

## Overview

Create a cli game that includes question management, and maintain a Hall of Fame for top scores.

### The Game

The user is presented a statement like "a lion can jump 36 feet".
The user then enters true or false.
If the user is right they get one point. If they are wrong, the game ends and the user is prompted to enter a player name for the hall of fame, which shows the best 10 attempts if they want to start an new one.

## Objectives

 - Utilize Rust's structs, traits, modules, and other features to build a well-structured and efficient application.
- Develop a trivia game with an engaging user experience and robust game logic.


### Project Requirements


1. **Trivia Statement Struct**  
Create a Statement struct containing a trivia statement and a boolean indicating if it's true or false.

2. **Traits Usage**  
Implement traits, such as a trait for checking if an answer is correct.

4. **Player Struct**  
Define a Player struct to hold information like current score, and name (to be added later).

5. **Randomized and Non-Repeating Statements**  
Randomly present trivia statements to the player.
Ensure the same statement is not presented twice.
If all statements are answered, display a "YOU WIN" message and prompt for the username. Then, record the score in the Hall of Fame and show it.

6. **Hall of Fame Module**  
Implement a separate module for the Hall of Fame.
Store the top 10 scores with player names.

7. **Custom Error Types**  
Create and utilize custom error types to handle various game scenarios.

8. **Game Logic Module**  
Develop a separate module for handling game logic.

9. **Testing**  
Write comprehensive tests for different components of the application.

10. **Use of Result**  
Utilize `Result` for error handling and data returning throughout the application.

11. **Logging Macro**  
Create a custom macro (e.g., `log!(DEBUG, "my message")`) for logging purposes.
Place the macro in a separate crate (lib).