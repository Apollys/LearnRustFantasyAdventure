#![allow(unused)]

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vector2f {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Resource {
    FreshWater,
    BerryBushes,
    Firewood,
    Stones,
}

const ALL_RESOURCES: [Resource; 4] =
    [Resource::FreshWater, Resource::BerryBushes, Resource::Firewood, Resource::Stones];

#[derive(Clone, Debug, PartialEq)]
struct CampMap {
    /// Locations that have good enough terrain to set up camp.
    possible_camp_locations: Vec<Vector2f>,

    /// Pairs of (resource, location).
    resource_locations: Vec<(Resource, Vector2f)>,
}

// --- Base Task Solution ---

/// Returns a camp location that has the minimal sum total distance to all resources.
///
/// Assumes that at least one instance of each resource type is present in the map.
fn choose_camp_location(map: &CampMap) -> Vector2f {
    let mut optimal_camp_location = None;
    let mut optimal_total_distance = f64::MAX;
    for &camp_location in map.possible_camp_locations.iter() {
        let total_distance = total_distance_to_resources(map, camp_location);
        if total_distance < optimal_total_distance {
            optimal_camp_location = Some(camp_location);
            optimal_total_distance = total_distance;
        }
    }

    optimal_camp_location.unwrap()
}

impl Vector2f {
    fn distance_to(&self, other: &Self) -> f64 {
        f64::hypot(other.x - self.x, other.y - self.y)
    }
}

/// Returns the sum total distance to the nearest of each type of resource from the given camp
/// location.
fn total_distance_to_resources(map: &CampMap, camp_location: Vector2f) -> f64 {
    // Compute the distance to the nearest resource of each type.
    let mut nearest_resource_distances = HashMap::<Resource, f64>::new();
    for (resource, resource_location) in map.resource_locations.iter() {
        let distance = camp_location.distance_to(resource_location);
        match nearest_resource_distances.entry(*resource) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(distance);
            }
            Entry::Occupied(mut occupied_entry) => {
                if distance < *occupied_entry.get() {
                    occupied_entry.insert(distance);
                }
            } // Note: OccupiedEntry::insert takes a `&mut self` parameter, while VacantEntry::insert
              // takes a `self`. Therefore, we need the `mut` keyword only in the Occupied case above.
              //   - https://doc.rust-lang.org/std/collections/hash_map/struct.OccupiedEntry.html#method.insert
              //   - https://doc.rust-lang.org/std/collections/hash_map/struct.VacantEntry.html#method.insert
        }
    }

    nearest_resource_distances.values().into_iter().sum()
}

// --- Extension Solution ---

fn choose_camp_location_weighted(
    map: &CampMap,
    resource_importance_factors: &HashMap<Resource, f64>,
) -> Vector2f {
    let mut optimal_camp_location = None;
    let mut optimal_total_score = f64::MAX;
    for &camp_location in map.possible_camp_locations.iter() {
        let total_score =
            compute_camp_location_score(map, resource_importance_factors, camp_location);
        if total_score < optimal_total_score {
            optimal_camp_location = Some(camp_location);
            optimal_total_score = total_score;
        }
    }

    optimal_camp_location.unwrap()
}

fn compute_camp_location_score(
    map: &CampMap,
    resource_importance_factors: &HashMap<Resource, f64>,
    camp_location: Vector2f,
) -> f64 {
    let mut nearest_resource_scores = HashMap::<Resource, f64>::new();
    for (resource, resource_location) in map.resource_locations.iter() {
        // Compute the resource score.
        let resource_factor = resource_importance_factors.get(resource).unwrap();
        let distance = camp_location.distance_to(resource_location);
        let resource_score = resource_factor * distance;

        // Update the map if the resource score is a new optimal for this resource type.
        match nearest_resource_scores.entry(*resource) {
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(resource_score);
            }
            Entry::Occupied(mut occupied_entry) => {
                if resource_score < *occupied_entry.get() {
                    occupied_entry.insert(resource_score);
                }
            }
        }
    }

    nearest_resource_scores.values().into_iter().sum()
}

// --- Bonus Extension Solution ---

fn choose_camp_location_bonus(
    map: &CampMap,
    resource_importance_factors: &HashMap<Resource, f64>,
) -> Vector2f {
    // Potential solution: implement this in the exact same way as the extension solution,
    // except compute the resource score using the distance squared. This penalizes one
    // thing being very far away more than two things each being half as far away:
    //   resource_score = resource_factor * distance * distance;
    //
    // For example, consider the choice between one resource being a distance 2 away versus
    // two resources each being a distance 1 away. Letting resource_factor = 1 for the sake
    // of comparison, this gives us: 2² = 4 versus 2 × (1²) = 2.
    //
    // The optimal choice is to be exactly halfway between the two resources.
    unimplemented!();
}

fn main() {}
