# Camp Location

As you continue to explore the enchanted forest, you decide it would be ideal for you to set up a camp location within the forest itself. This will allow you to explore the forest more thoroughly, as well as helping you connect with the ecosystem and energy of the forest more deeply.

You spend a few days mapping out locations of various sites of interest, including important resources (such as water or berry bushes) and locations where the terrain is good for setting up camp. Once your map is complete, the time has come to choose your ideal camp location.

You would like to choose a camp location on decent terrain that has reasonable access to all the resources you need. You decided that the best camp location would be the one with minimum total distance to all resources, where total distance is the sum of the distance from the camp site to each resource. What is your ideal camp location?

---

Fill in the implementation of the Rust function `choose_camp_location` to calculate the solution.

**Useful References:**
 - [`f64::hypot`](https://doc.rust-lang.org/std/primitive.f64.html#method.hypot)
 - [Associated Functions & Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html) (Rust By Example)
   - You may find it helpful to define a `distance_to(&self, other: &Self) -> f64` method for the `Vector2f` type. This will likely make your code cleaner and easier to read overall.
 - The `HashMap` references from the previous task will again be useful here:
   - [Updating a Hash Map](https://doc.rust-lang.org/book/ch08-03-hash-maps.html?highlight=hashmap%20entry#updating-a-hash-map) (The Rust Book)
   - [Examples](https://doc.rust-lang.org/std/collections/struct.HashMap.html#examples) (Standard Library Docs)
   - [Entry](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html) — useful for succinctly looking up and modifying values in a HashMap.

## Extension

Suppose resources are not equally important. For each resource, you give it an importance score. For example, if water is twice as important as food, you may give water an importance score of **2** and berry bushes an importance score of **1**.

You can then use these importance scores as weights for the distances to the resources, giving you a "campsite score" equal to the sum of each resource's importance score times its distance to the campsite. (Here, lower score is better.)

What is the camp location that yields the optimal campsite score?

Fill in the implementation of the Rust function `choose_camp_location_weighted` to calculate the solution.

**Useful References:**
 - [Same as those above.]

## Bonus Extension

Reflecting upon the original method you used to rank campsite locations (the minimal total distance to all resources), you notice a significant flaw. Imagine there was a water spring in one location, and a patch of berry bushes a distance *d* away. By this method, any location along the line between the spring and the berry bushes would give the same total distance score: *d*.

However, it seems reasonable to prefer a location somewhere near the midpoint between the spring and the berry bushes, so any time you need either one, you are reasonably close to it. Or, phrased another way, one thing being very far away is more problematic than a couple things being a medium distance away.

How would you improve the way you calculate the "campsite score" to reflect this?

Fill in the implementation of the Rust function `choose_camp_location_bonus` to calculate the solution.

Note: there are many possible ways to approach this problem. As long as your solution gives the maximal score to the midpoint between the spring and the berry bushes (in the above example), it will be considered correct.

**Useful References:**
 - [Same as those above.]
