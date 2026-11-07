Dungeon Supply Chest in Rust

A simple Rust program that loads items from a text file and displays the contents of a dungeon supply chest.

The program reads items from chest.txt. If the file does not exist, it uses default items such as Rope, Torch, and Potion.

The program also checks item positions safely using Vec::get() and handles errors without crashing.

AI usage

I used ChatGPT to help me understand Rust error handling, review my code, and compare my implementation with the teacher's examples.

I learned how to use Result, Option, match, and the ? operator to handle errors. I also learned how panic! works, how to prevent crashes using Vec::get(), and how backtraces help locate errors in the code.
