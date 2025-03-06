# Entropy

"I'm glad to have learned a few things about you," says Eloah. "I'm currently working on a project, and I'm wondering if you wouldn't mind helping with something. I think you have a certain quality, which is exactly what I'm looking for."

Surprised that a majestic talking lioness would be looking for something that you have, you ask, "What could that be?"

She replies, "Well, I'm in need of [entropy](https://en.wikipedia.org/wiki/Entropy). It's something I can't create for myself. But I have a feeling you would be a great soure of the entropy I need."

You're not quite sure what she's talking about. "If I have any of this entropy of which you speak, I certainly don't know where it is, much less how I could give it to you," you say uncertainly.

"Not to worry!" she replies, "All you need to do is give me a bunch of random numbers. Quick, just say whatever numbers pop into your mind. But *do* make them random. Well, without trying so hard to make them seem *really* random that they end up not being random after all..."

---

Write a Rust function called `create_entropy` that returns a random integer between two given integers. The given endpoints should be considered inclusive, and there should be equal probability to select any of the possible values. Use the 32-bit signed integer type for all values.

**Useful References:**
 - [Generating a Secret Number](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#generating-a-secret-number) (The Rust Book)
 - [Generate Random Values](https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html) (Rust Cookbook)
    - Note: some of the function names in these examples are outdated. If you follow this code, you will get some warnings telling you the names have changed. You can simply change the names to the new ones given in the message and the warnings will go away.
 - [rand](https://docs.rs/rand/latest/rand/) (Crate)
 - [RangeInclusive](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html) (Rust standard library)
 - [Integer Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types) (The Rust Book)
 - [Scalar Types](https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types) (Rust By Example)

## Extension

The entropy from those integers was good, but Eloah is looking for even *more entropy* for her mysterious project. "I think we can magnify the entropy density tremendously if you could give me non-whole numbers too," she ponders. "Yes, that would be absolutely magnificent."

---

Write a Rust function called `create_more_entropy` that returns a random floating-point value between two given floating-point values, selected with uniform probability. To practice doing something different this time, make the lower bound inclusive but the upper bound *exclusive*. Use the 64-bit floating point type for all values.

**Useful References:**
 - [Range](https://doc.rust-lang.org/std/ops/struct.Range.html) (Rust standard library)
 - [Floating-Point Types](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types) (The Rust Book)
 - The references from the original task above are also very useful here.
