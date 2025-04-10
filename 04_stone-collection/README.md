# Stone Collection

Over the next week, you spend your days exploring the forest further. Throughout your explorations, you begin to collect a variety of fascinating stones that you come across. Some of these stones even seem to glow with a radiant energy. Yet others have a swirling appearance as though two differently-colored liquids are slowly being mixed, though they are hard as any other stone.

At the end of the week, you place all the most interesting stones carefully in a leather pouch and bring back to your village. You take them to the Arcane Jeweller of the village, asking what he might know about these mysterious stones.

The jeweler looks over your stones meticulously. For each stone, he first weighs it precisely, and then examines it closely, occasionally bringing out some special tool here or there. With his great experience handling mysterious stones, he has no trouble identifying the types and properties of most of the stones. However, some of the stones have an aura of magic that even he has never felt before. These stones remain as classification unknown.

Given the Arcane Jeweller's type classification of each stone from your collection, how many stones do you have of each type? (Consider all unknowns as belonging to a single type.)

---

Fill in the implementation of the Rust function `count_stone_types` to calculate the solution.

**Useful References:**
 - Enums:
   - [The Rust Book](https://doc.rust-lang.org/beta/book/ch06-01-defining-an-enum.html)
   - [Rust By Example](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
 - HashMap:
   - [Updating a Hash Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html?highlight=hashmap%20entry#updating-a-hash-map) (The Rust Book)
   - [Examples](https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples) (Standard Library Docs)
   - [Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) — useful for succinctly looking up and modifying values in a HashMap.

## Extension A

You separate out the *unknown*-type stones and place them in a special wooden box in your home. What is the total weight of these unknown-type stones?

Fill in the implementation of the Rust function `compute_total_unknown_weight` to calculate the solution.

**Useful References:**
 - [Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html) with Enums (The Rust Book)
 - Functional methods (also mentioned in the previous task):
   - [filter](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter) - Only keep items matching a certain condition.
   - [fold](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold) - Accumulate the values via some operation.
   - [sum](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.sum) - Add up all the values.

## Extension B

For the remaining stones, the jeweller tells you the weight and type of each. He also tells you the value of each type of stone per unit weight. According to this information, what is the total value of these stones?

Fill in the implementation of the Rust function `compute_total_known_stone_value` to calculate the solution.

**Useful References:**
 - Same as above.

## Extension C

Of the known-type stones, you decide you will keep the *k* most valuable stones for yourself, and sell the jeweller the remaining stones. How much should he pay you for these stones?

Fill in the implementation of the Rust function `compute_total_selected_stone_value` to calculate the solution.

**Useful References:**
 - [Sorting Vectors](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html) (The Rust Cookbook)
 - [Use `partial_cmp` to sort floats](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html#sort-a-vector-of-floats) (same page as above)
