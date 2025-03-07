# Creek Bridge

Eloah thanks you for the entropy, saying, "Now it's time for me to put it to use. I'm sure we'll see each other again soon." And she gracefully bounds off into the forest.

You continue walking through the forest. Soon you come across a creek you would like to cross. It's too large to wade through, so you decide you will build a bridge from nearby tree branches. You'll build the basic structure of your bridge by laying these branches across the creek side-by-side.

After collecting all the nearby branches, you measure the length and width (diameter) of each. You also measure the width of the creek at its narrowest point. What is the widest bridge you can build that crosses the creek? In order to make a solid foundation across the creek, a branch must be at least one unit of length longer than the width of the creek.

---

Fill in the implementation of the Rust function `calculate_bridge_width` to calculate the solution.

**Useful References:**
 - [Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html) (Rust By Example), [Vec](https://doc.rust-lang.org/std/vec/) (Rust Standard Library)
 - [Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html) (Rust By Example), in particular:
    - [if/else](https://doc.rust-lang.org/rust-by-example/flow_control/if_else.html)
    - [for and range](https://doc.rust-lang.org/rust-by-example/flow_control/for.html)
 - [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) (The Rust Book)
 - [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) (The Rust Book)

## Bonus 1

As practice for yourself, you may enjoy trying to implement this using all three types of loops: [loop](https://doc.rust-lang.org/rust-by-example/flow_control/loop.html), [while](https://doc.rust-lang.org/rust-by-example/flow_control/while.html), and [for](https://doc.rust-lang.org/rust-by-example/flow_control/for.html).

## Bonus 2

If you have prior experience with a [functional](https://en.wikipedia.org/wiki/Functional_programming) programming language, or are curious to learn how the functional programming style works in Rust, I would encourage you to try to solve this task using the ideas in [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html). The functional programming style can be quite fun, leading to very elegant and concise code constructs!

The core elements of functional programming in Rust are the [Iterator](https://doc.rust-lang.org/rust-by-example/fn/methods.html), and in particular the following methods:
 - [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) - Only keep items matching a certain condition.
   - Example: keep only even integers in a list of numbers.
 - [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) - Accumulate the values via some operation.
   - Example: compute the total sum or product of a list of numbers.
 - [sum](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) - Add up all the values.
   - This is just a convenience function that uses `fold` combined with the addition operation. You may want to see if you can compute the sum from `fold` directly for practice.
 - [map](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map) - Apply some mapping function (which can be thought of as conversion or translation) to all items.
   - Example: map each character to its upper-case version to produce an all-caps string.

## Extension

After building the bridge as calculated above, you chop up the rest of the wood and carry it across the creek until you find a nice clearing in the forest. You decide you will build a campfire in this clearing, and are curious how long this wood will burn for.

Given all the wood burns at a consistent given burn rate, and assuming each tree branch is a perfect cylinder, how long will your fire burn for if you use all the remaining wood?

---

Fill in the implementation of the Rust function `calculate_fire_lifespan` to calculate the solution.

**Useful References:**
 - [Associated Functions & Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html) (Rust By Example)
   - You may find it helpful (or at least good practice) to define a `volume(&self) -> f64` method for the `TreeBranch` type.
