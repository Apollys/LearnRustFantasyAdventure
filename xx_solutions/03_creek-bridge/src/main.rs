#![allow(unused)]

use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq)]
struct TreeBranch {
    length: i32,
    diameter: i32,
}

impl TreeBranch {
    fn volume(&self) -> f64 {
        let radius = 0.5 * (self.diameter as f64);
        PI * radius * radius * (self.length as f64)
    }
}

/// Returns the maximum width of a bridge you can build across the creek with the given branches.
fn calculate_bridge_width(branches: &Vec<TreeBranch>, creek_width: i32) -> i32 {
    (branches.iter())
        // Keep only branches long enough to cross the creek.
        .filter(|branch| branch.length >= creek_width + 1)
        // Add up the total diameters of these kept branches.
        .fold(0, |accum, branch| accum + branch.diameter)
}

/// Returns the total duration (in seconds) your fire will burn for, using the wood that remains
/// after building the widest possible bridge across the creek.
/// The given `branches` corresponds to the original set of branches before building the bridge.
/// The given `burn_rate` is measured in volume per second.
fn calculate_fire_lifespan(
    branches: &Vec<TreeBranch>,
    creek_width: i32,
    burn_rate: f64,
) -> f64 {
    let unused_wood_volume = (branches.iter())
        .filter(|branch| branch.length < creek_width + 1)
        // Add up the volume of each branch, assuming each is a cylinder.
        .fold(0.0, |accum, branch| accum + branch.volume());

    unused_wood_volume / burn_rate
}

fn main() {
    let branches = vec![
        TreeBranch { length: 6, diameter: 10 },
        TreeBranch { length: 8, diameter: 8 },
        TreeBranch { length: 10, diameter: 6 },
        TreeBranch { length: 12, diameter: 4 },
        TreeBranch { length: 14, diameter: 2 },
    ];
    let creek_width = 9;

    // This should include all but the first two branches, therefore the total width should be 12.
    println!("Maximum bridge width: {}", calculate_bridge_width(&branches, creek_width));

    // The total wood volume to be burned is PI times the sum of the length times radius squared for
    // the first two branches: PI * (6 * 25 + 8 * 16) = PI * (150 + 128) = PI * 278.
    // With a burn rate of PI, the final fire lifespan should then be 278.
    println!("Fire lifespan: {}", calculate_fire_lifespan(&branches, creek_width, PI));
}
