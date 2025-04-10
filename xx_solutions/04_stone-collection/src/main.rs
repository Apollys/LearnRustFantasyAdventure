#![allow(unused)]

use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum StoneType {
    Granite,
    Marble,
    CloudedSapphire,
    RadiantAmethyst,
    ArcaneCrystal,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
struct Stone {
    type_: StoneType,
    weight: f64,
}

/// Returns a vector containing pairs of (stone_type, count), with one item
/// per stone type. (Stone types for which there are no stones may be omitted.)
fn count_stone_types(stones: &Vec<Stone>) -> Vec<(StoneType, usize)> {
    let mut stone_type_counts = HashMap::<StoneType, usize>::new();
    for stone in stones.iter() {
        *stone_type_counts.entry(stone.type_).or_default() += 1;
    }

    stone_type_counts.into_iter().collect::<Vec<_>>()
}

/// Returns the total weight of all the stones of Unknown type.
fn compute_total_unknown_weight(stones: &Vec<Stone>) -> f64 {
    (stones.iter())
        .filter(|stone| stone.type_ == StoneType::Unknown)
        .fold(0.0, |partial_sum, stone| partial_sum + stone.weight)

    // Note: the fold line above is provided as an example for how to use `fold`.
    // In this case, it could be replaced by just: `.sum()`
}

/// Returns the total value that will be earned by selling all the stones
/// of a known type (any type other than `Unknown`).
///
/// The given stone_type_values is a map from stone type to value per unit weight,
/// containing each stone type (except possibly Unknown) as a key.
fn compute_total_known_stone_value(
    stones: &Vec<Stone>,
    stone_type_values: &HashMap<StoneType, f64>,
) -> f64 {
    (stones.iter())
        .filter(|stone| stone.type_ != StoneType::Unknown)
        .map(|stone| stone.weight * stone_type_values.get(&stone.type_).unwrap())
        .sum()
}

/// Returns the total value that will be earned by selling the stones, after keeping all
/// unknown stones for yourself, as well as the num_stones_to_keep most valuable stones.
///
/// The given stone_type_values is a map from stone type to value per unit weight,
/// containing each stone type (except possibly Unknown) as a key.
fn compute_total_selected_stone_value(
    stones: &Vec<Stone>,
    stone_type_values: &HashMap<StoneType, f64>,
    num_stones_to_keep: usize,
) -> f64 {
    let mut stone_values = (stones.iter())
        .filter(|stone| stone.type_ != StoneType::Unknown)
        .map(|stone| stone.weight * stone_type_values.get(&stone.type_).unwrap())
        .collect::<Vec<_>>();

    // We have to use partial_cmp to sort floats.
    stone_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if num_stones_to_keep >= stone_values.len() {
        return 0.0;
    }

    let end_index_exclusive = stone_values.len() - num_stones_to_keep;
    (0..end_index_exclusive).map(|i| stone_values[i]).sum()
}

fn main() {
    let stones = vec![
        Stone { type_: StoneType::Granite, weight: 1.0 },
        Stone { type_: StoneType::Marble, weight: 1.0 },
        Stone { type_: StoneType::Granite, weight: 1.0 },
        Stone { type_: StoneType::CloudedSapphire, weight: 2.0 },
        Stone { type_: StoneType::CloudedSapphire, weight: 2.0 },
        Stone { type_: StoneType::Unknown, weight: 1.0 },
        Stone { type_: StoneType::ArcaneCrystal, weight: 10.0 },
        Stone { type_: StoneType::Unknown, weight: 2.0 },
        Stone { type_: StoneType::Unknown, weight: 3.0 },
    ];

    let stone_type_values = HashMap::from([
        (StoneType::Granite, 2.0),
        (StoneType::Marble, 3.0),
        (StoneType::CloudedSapphire, 5.0),
        (StoneType::ArcaneCrystal, 10.0),
    ]);

    println!("count_stone_types: {:?}", count_stone_types(&stones));

    println!("compute_total_unknown_weight: {:?}", compute_total_unknown_weight(&stones));

    println!(
        "compute_total_known_stone_value: {:?}",
        compute_total_known_stone_value(&stones, &stone_type_values)
    );

    let num_stones_to_keep = 2;
    println!(
        "compute_total_selected_stone_value: {:?}",
        compute_total_selected_stone_value(
            &stones,
            &stone_type_values,
            num_stones_to_keep
        )
    );
}
