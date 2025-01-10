# word-quest
A simple word roguelike game developed with Rust.

Play Now! at https://kidskoding.itch.io/word-quest. Linux support is coming soon!!!

Create words using the given letter tiles to reach the target score for each round. You win the game if you can survive 5 rounds! If you can't reach the target score in 4 attempts, the game is over!

## Scoring System

- **Common Letters (10 pts)**: E, A, T
- **Regular Letters (15 pts)**: H, I, N, O, S, R
- **Medium Letters (20 pts)**: D, L
- **Less Common (25 pts)**: U, F, M, C, G, Y
- **Rare Letters (30 pts)**: P, B, W
- **Very Rare (35 pts)**: V, K, J, X, Q, Z

The score of each word is the total score of all characters along with a score multiplier based on the word's length!

## Controls

- Click tiles or type letters to build words
- Press ENTER or click 'Play Word' to submit
- Press BACKSPACE or 'X' to clear the current word
- Use 'Shuffle' to rearrange tiles
- Use 'Discard' to get new tiles (3 per round)
A simple word roguelike game developed with Rust and Macroquad