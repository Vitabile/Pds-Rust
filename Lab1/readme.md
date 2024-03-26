# Exercise 1
## Goal 
The goal of the exercise is to make a "slugify" function that converts a generic string into a
slug.

## Description
The term "slug" refers to a string converted to readable format, consisting only of the
[a-z][0-9]- characters.
In the original string, ineligible characters are converted following these rules:
* all recognized accented characters are converted to the unaccented equivalent
* everything is converted to lower case
* any remaining characters that are not in [a-z][0-9] are converted to "-"
* two consecutive "-"s are not allowed, only the first one is kept
* a final "-" is not allowed unless it is the only character in the string

# Exercise 2
## Exercise 2.1
Preliminar exercises:
* Open, read and save a file: read a "test.txt" file with text in it and save the text repeated 10 times in the same file.
* Use of enum define an enum Error with two values in it: Simple(SystemTime) and Complex(SystemTime, String) and make a print_error(e: Error) function that will prints out the type of error and the information it contains (without using {:?} debug, but handling the enum values appropriately).
* Using self, &self and &mut self In the methods of a struct you can use self to have a reference to the object you are operating on: what is the difference in behavior between self, &self and &mut self?

## Exercise 2.2
### Goal
A program needs to handle the construction of a 20x20 naval battle pattern saved on file.
The file format is as follows (21 lines):
* LINE 1: N1 N2 N3 N4, 4 integers separated by space indicating the number of ships of lengths 1, 2 , 3 and 4, respectively, that can still be added to the board
* LINES 2..21, 20 lines of 20 characters with " " (space) for empty boxes and "B" for those with ships

### How to run it
The construction of the board is done in steps, invoking the program with parameters
* __cargo run -- new --file board.txt --boats 4,3,2,1__

This creates a new empty board in the file board.txt and can accommodate 4 ships of 1, 3 2 ships, 2 3 ships, and 1 4 ship.
* __cargo run -- add --file board.txt --boat 3V --start 10,10__

Reads the board in board.txt, adds a 3-square ship vertically, starting from box (10,10) and going down 3 boxes, to (12,10). Possible directions: H and V. Adding the ship, saves the result in board, also updating the available ships in the first line.
