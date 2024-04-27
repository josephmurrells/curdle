# Curdle

A command line wordle clone written in Rust, just for fun and to try out Rust. 

Try to guess a five letter word with five attempts to get the correct answer. Correctly placed letters are green, correct letters incorrectly placed are yellow and incorrect letters are red.

![image](https://github.com/josephmurrells/curdle/assets/111430789/7b580b7f-7b32-4558-b76b-39dc3b30f7b1)

Uses words.json in same dir as executable to generate word list, if this file is missing it will fetch a new word list from this public wordle api https://wordle-api.cyclic.app/.
