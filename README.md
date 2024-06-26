# 100 Days of Rust

My journey of learning the [Rust](https://www.rust-lang.org/) programming language by working at it for 100 consecutive days.

![Rust](https://img.shields.io/badge/100_Days-Rust-000000?&logo=rust&logoColor=whi000000te&url=https://www.rust-lang.org/)
![pre-commit](https://img.shields.io/badge/pre--commit-enabled-FAB040?&logo=precommit&logoColor=FAB040&url=https://www.rust-lang.org/)

To begin, I will be working through [*The Rust Programming Language* (2nd ed.)](https://www.amazon.com/Rust-Programming-Language-2nd-dp-1718503105/dp/1718503105/ref=dp_ob_title_bk) by Klabnik and Nichols.

Other possible resources:

- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
- [Rust Cookbook](https://rust-lang-nursery.github.io/rust-cookbook/)

To play with:

- [Rapier](https://rapier.rs/) physics engine
- [Bevy](https://bevyengine.org/) game engine
- [ndarray](https://docs.rs/ndarray/latest/ndarray/#)
- web scraping

## Progress

| Day | Description |
|----:|:------------|
|   1 | Rust "Hello, world!" and learned about the key `cargo` commands |
|   2 | Learned about variables, type conversion, and the match statement |
|   3 | Learned about 'loop' and 'match' keywords; read about basics of variables and functions |
|   4 | Finished up control flow syntax and began learning about ownership |
|   5 | Finshed chapter 4 on ownership |
|   6 | Started learning about structs |
|   7 | Finished up the basics of structs and now moving on to enums |
|   8 | Wrapped up enums and `Option<t>`, began learning more details about creates, etc. |
|   9 | More details about paths, scope, and public items |
|  10 | Finished the chapter on crate and module structures |
|  11 | Experimented with the rust-bio crate |
|  12 | Read the chapter on collections |
|  13 | Started reading about exception handling |
|  14 | Finished reading about exceptions and now onto generics and traits |
|  15 | Read up on generics and now learning about traits |
|  16 | Began reading about generic lifetimes and lifetime annotation 🤯 |
|  17 | Played around with example code from the internet, just general practice |
|  18 | Finished reading the chapter on generics and lifetime annotations; worked through the chapter of rust by example on conversions |
|  19 | Working through examples of flow control code just practising writing rust code |
|  20 | More practice with match and now working on examples with functions |
|  21 | Finished the set of examples for functions and now playing with random number generation in the cookbook |
|  22 | Finished the set of examples for functions and now playing with random number generation in the cookbook |
|  23 | Played with the `rand` and `rand_distr` crates using examples from the cookbook |
|  24 | Experimented with the concurrency examples in the cookbook |
|  25 | More examples with concurrency that gave me a good opporunity to learn about the "error-chain" crate |
|  26 | Learning about the "ndarray" crate |
|  27 | Learning about the "ndarray" crate |
|  28 | Finished the examples with "ndarray" and practiced some web requests including with async |
|  29 | Began working through the rust cli cookbook |
|  30 | Trying to refactor and test the simple cl tool |
|  31 | Succesfully refactored some of the demo cli tool to enable testing and added a unit test and two integration tests |
|  32 | Started a new simple project to query the omnivore api |
|  33 | Started a new simple project to query the omnivore api |
|  34 | Started chapter 13 of the book on functional programming in rust and revisited the chapter on generics, traits, and lifetimes |
|  35 | Converted a web-scraping project from python to rust, builst some some aws s3 interactions in rust for a work project |
|  36 | Worked on getting the trout stocking web-scraping code onto deta and writing to the provided database |
|  37 | Refactored my trout stocking code into binary and library and used axum to make a server interface |
|  38 | Working on getting my trout stocking scraping app deployed to deta space |
|  39 | Continued working on deplying my trout scraping deta app, now trying to use github actions to compile |
|  40 | Read about logging and tracing in rust and added logging to my app |
|  41 | Refactored the data scraping micro into a workspace to separate the scraping logic from the web-server front-end for deta |
|  42 | Add an index data base to my trout stocking deta space project |
|  43 | End point to initiate re-indexing of data bases periodically |
|  44 | Reduced code duplication and complexity by using traits and enums in my trout scraping project |
|  45 | Refactored more code for the trout data scraping project |
|  46 | Started working through the 'rustlings' problems |
|  47 | Continued with rustlings practice |
|  48 | Continued with rustlings practice (strings, hashmaps, modules, quiz 2) |
|  49 | Continued with rustlings practice (options and error handling) |
|  50 | Continued with rustlings practice (generics, trains, quiz 3) |
|  51 | Rustlings on lifetimes and tests, also setup advent of code 2022 in rust project |
|  52 | Rustlings iterators (took me a little while because the editor doesnt provide syntax help) |
|  53 | Advent of code 2022 day 3 puzzles - a lot of practice with iterators |
|  54 | Advent of code 2022 day 4 puzzles |
|  55 | Advent of code 2022 day 5 puzzle 1 gave me a lot of practice with parsing the input data |
|  56 | Advent of code 2022 day 5 puzzle 1 gave me a lot of practice with parsing the input data |
|  57 | Advent of code 2022 day 6 puzzle was actually easy, giving me some confidence |
|  58 | Started puzzle 7 for advent of code 2022 |
|  59 | Working example for puzzle 7, but run time error |
|  60 | Refactoring puzzle 7, but still got the same error; i think i know what the problem is though |
|  61 | Solved puzzle 7 |
|  62 | Solved puzzle 8 and it runs way faster with `--release` |
|  63 | Parsed the input for puzzle 9 of AoC 2022 |
|  64 | Finished both puzzles for day 9 of AoC 2022 |
|  65 | Parsed input and prepared key models for puzzles of day 10 of AoC 2022 |
|  66 | Finished puzzles for day 10 of AoC 2022 |
|  67 | Finished puzzles for day 11 of AoC 2022 but had to look up the hint for part 2 |
|  68 | Finished puzzles for day 12 of AoC 2022 |
|  69 | Start Advent of Code 2023 in Rust |
|  70 | Day 2 of AoC 2023 |
|  71 | Day 3 of AoC 2023 |
|  72 | Day 4 of AoC 2023 |
|  73 | Day 5 of AoC 2023 |
|  74 | Day 6 of AoC 2023 (brute force for puzzle 2 actually worked fine) |
|  75 | AoC 2023 day 6 |
|  76 | AoC 2023 day 7 was great practice with generics |
|  77 | Day 9 of AoC 2023 (an easy day) |
|  78 | Day 10 of AoC 2023; still working on part 2, but getting good practice at using the 'petgraph' crate |
|  79 | Day 11 of AoC 2023 gave me good practice with the 'ndarray' crate |
|  80 | Was stumped by part 2 of day 12 of AoC, but got help and was able to implement the recommended algorithm in rust easily enough |
|  81 | I completed day 13 of AoC without too much trouble, i'm definitely feeling more competent with rust |
|  82 | Completed another day of AoC without too much struggling against the language |
|  83 | Successfully coded the process described in day 15 of AoC in one attempt! |
|  84 | AoC 2023 Day 16 |
|  85 | I think I'm close to a solution for Day 17 of AoC 2023, but something is slightly off. |
|  86 | Solved AoC 2023 Day 17. |
|  87 | Day 18 of AoC 2023. |
|  88 | Start day 19 of aoc 2023, but it's pushing the limits of my ability to architect rust code |
|  89 | Finished day 19 puzzle 1, but it's going to have to change a lot for part 2 |
|  90 | Aoc 2023 day 20 part 1 |
|  91 | Aoc 2023 day 20 part 2 |
|  92 | Aoc 2023 day 21 part 1 |
|  93 | Read about closures and iterators in the rust book and did the rustlings exercises on smart pointers |
|  94 | Read the chapter on advanced uses of cargo and workspaces and started the chapter on smart pointers |
|  95 | Finished reading about smart pointers |
|  96 | Began ch 16 on "fearless concurrency" and completed appropraite rustlings exercise |
|  97 | Finished ch 16 on fearling concurrency and the corresponding rustlings exercises |
|  98 | Read chater 17 about oop (no paired rustlings) |
|  99 | Finished rust book reading about advanced features |
|  100 | Finished rustlings |
<!-- [[NEXT DAY]] -- DO NOT REMOVE. -->
