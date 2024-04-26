# Curdle

A command line wordle clone written in Rust, just for fun and to try out Rust. 

Try to guess a 5 letter word with 5 attempts to get the correct answer. Correctly placed letters are green, correct letters incorrectly placed are yellow and incorrect letters are red.

![image](https://github.com/josephmurrells/curdle/assets/111430789/7a769060-9126-4181-a447-9f23b688eeea)

Uses words.json in same dir as executable to generate word list, if this file is missing it will fetch a new word list from this public wordle api https://wordle-api.cyclic.app/.
