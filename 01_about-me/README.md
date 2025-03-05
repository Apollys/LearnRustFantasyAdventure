# About Me

We begin our adventure in a small village nestled in the lush valley of a beautiful wilderness
landscape. Here, the people are few and the natural creatures of the land are many. The sounds of
nature far outstrip the sounds of machines, and when night falls, the light from the stars far
outshines any light from the village.

The people here are not afraid of the creatures of the land, the darkness of night, or the crisp
chill of the wintry wind. No, quite the opposite – they've learned there's peaceful serenity in
these things; and if you learn to befriend them as these people have, you wouldn't give them up for
all the wordly comforts that other "more advanced" civilizations have to offer.

This gentle village is your home. It has been ever since you were born. And though you've traveled
to many other lands and visited many great cities with fancy machines and advanced scientific
knowledge, they all seem to be so far behind this little village in some ineffable way. Perhaps this
is the difference between knowledge and wisdom.

There is a vast forest to the north of your village. You've been to its edge a few times, each time
feeling a primal magnetism as you approach it. There is something fascinating and mysterious about
this forest. And as you have learned from the people of your village, the darkness is nothing to
fear and the creatures of the land are you not your enemies, as long as you do not make yourself
their enemies or conjure fear upon yourself. And so, today you venture into the forest, filled not
with fear or anxiety, but with a sense of curiosity and excitement.

The indescribable aura of magnetic energy grows stronger as you trek deeper into the forest. After
some time, you come across a majestic lioness, sitting calmly by a stream, seemingly engrossed in
watching the water flow over the rocks in its infinitely varying patterns. You stop, not wanting to
disturb the lion, and turn to quietly walk a different way. Just at that moment...

"I am Eloah," the lioness speaks. "What's your name?"

## Task 1

Write a Rust program that prints your name when it is run.

To get started, initialize a Rust package in this directory with the following command, run from
within this directory:

```
cargo init . --name=about-me
```

> Explanation:
>
>  - `cargo init` initializes a new Rust package.
>  - The dot `.` indicates to initialize the package in the current directory.
>  - We use `--name=about-me` to specify the package name. (If you try omitting this option, you
>    will see that it chooses the name `01_about-me` by default, which gives an error because it
>    starts with a number.)

After running this command, you will notice that (among other things) this creates a `src` directory
containing a `main.rs` file. Open this `main.rs` file, write your code in that file, and then run it
with the command `cargo run`.

**Useful References:**

 - [println!](https://doc.rust-lang.org/rust-by-example/hello/print.html) (Rust By Example)

### Extension

If a Lion can speak in full sentences, so can you!

Update the above program to print out your name within a complete sentence. Save your name to a
variable first, and then use that variable in the print statement.

Example output:

```
"My name is Cody. Where did you learn to speak Human anyway?"
```

**Useful References:**
 - [Variables](https://doc.rust-lang.org/rust-by-example/variable_bindings.html) (Rust By Example)
 - [Strings](https://doc.rust-lang.org/rust-by-example/std/str.html) (Rust By Example)

## Task 2

The teachings of the village people seem to have their impression on you, for you manage to carry
forth the conversation without being entirely startled by the prospect of a calm, talking lion.
After some time, you mention that you often spend your free time reading. At this, Eloah's eyes
immediately show a sparkle of interest.

"I love reading," Eloah says. "Limitless adventure lies in the pages of books. What's one of your
favorite books?"

---

Write a Rust function that takes no parameters and returns the name of this book.

Call your function from within the existing `main` function, and print the result to ensure that it
works.

**Useful References:**
 - [Functions](https://doc.rust-lang.org/rust-by-example/fn.html) (Rust By Example)
 - [Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) (The Rust Book)

Hint: When writing a function, start by writing the "skeleton" of the function – an empty function
definition with no code inside. To do this, you will need three things:

1. The function's name: conventionally this is written in `snake_case` in Rust. For example:
   `calculate_the_answer`.
2. The function's parameters: a name and a type for each.
3. The function's return type.

### Extension

Eloah would also like to know the name of the author of this book. Update your function to return a
*pair* of `String`s: the book name and the author name.

(Note: *pair* is a commonly-used term to refer to a tuple with two elements.)

**Useful References:**
 - [Tuples](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)

### Extension B

You might have felt this was a little confusing returning a pair strings. How would the user of this
function (if they were someone else) know that the first is the book name, and the second the author
name?

We can give more structure and meaning to collections of values with structs. Update your function
to return a struct instead of a pair, still containing the book name and author name.

**Useful References:**
 - [Structs](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
