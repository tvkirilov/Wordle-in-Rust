# Wordle-in-Rust
Worlde is a simple and fun word-guessing game written in Rust. Try to crack the hidden word in a limited number of attempts, with hints after each guess to guide you along the way!

This is a recreation of the catchy popular game "Wordle" in the programming language "Rust".

🎮 How to Play

A secret win word is chosen at game start.

You have up to 5 attempts to guess it.

After each valid guess, every letter is marked:

[X] — Full match: correct letter in the correct position

(X) — Partial match: letter exists in the word but in a different position

>X< — No match: letter is not in the word

Use the hints to refine your next guess.

You win if you guess the word in time; otherwise, you lose after 5 attempts.

The guess history is printed line by line. At the start, an empty row of |_| placeholders is shown for each character in the word (e.g., a 5-letter word shows |_||_||_||_||_|).
