//! Find an exact solution to the Travelling Salesman Problem using Brute Force
//!
//! **Note: This isn't really a useful algorithm as Brute force is `O(n!)`, and is only included
//! for completeness.**
//!
//!# Examples
//!
//!```
//!extern crate travelling_salesman;
//!
//!fn main() {
//!  let tour = travelling_salesman::brute_force::solve(
//!    &[
//!       (27.0, 78.0),
//!       (18.0, 24.0),
//!       (48.0, 62.0),
//!       (83.0, 77.0),
//!       (55.0, 56.0),
//!    ],
//!  );
//!
//!  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
//!}
//!```
//!
use std::collections::HashSet;
use std::iter::FromIterator;

use super::{get_distance_matrix, get_route_distance, Tour};

/// Returns an exact solution to the Travelling Salesman Problem using Brute Force
///
/// **Note: This isn't really a useful algorithm as Brute force is `O(n!)`, and is only included
/// for completeness.**
///
///# Parameters and Return Type
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// Returns a `travelling_salesman::Tour` struct, representing the approximate solution found.
///
///# Examples
///
///```
///extern crate travelling_salesman;
///
///fn main() {
///  let tour = travelling_salesman::brute_force::solve(
///    &[
///       (27.0, 78.0),
///       (18.0, 24.0),
///       (48.0, 62.0),
///       (83.0, 77.0),
///       (55.0, 56.0),
///    ],
///  );
///
///  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
///}
///```
pub fn solve(cities: &[(f64, f64)]) -> Tour {
    let mut smallest_tour = Tour {
        distance: 0.0,
        route: vec![],
    };

    if cities.is_empty() {
        return smallest_tour;
    }

    let mut unvisited_cities =
        HashSet::<usize>::from_iter((0..cities.len()).collect::<Vec<usize>>());
    let current_route = vec![0];
    unvisited_cities.remove(&0);

    _brute_force(
        &get_distance_matrix(cities),
        unvisited_cities,
        current_route,
        &mut smallest_tour,
    );

    smallest_tour
}

fn _brute_force(
    distance_matrix: &[Vec<f64>],
    unvisited_cities: HashSet<usize>,
    current_route: Vec<usize>,
    smallest_tour: &mut Tour,
) {
    for unvisited_city in &unvisited_cities {
        let mut my_unvisited_cities = unvisited_cities.clone();
        my_unvisited_cities.remove(unvisited_city);

        let mut my_route = current_route.clone();
        my_route.push(*unvisited_city);

        if my_unvisited_cities.is_empty() {
            let home_city = my_route[0];
            my_route.push(home_city);

            let my_route_distance = get_route_distance(distance_matrix, &my_route);

            if (smallest_tour.distance == 0.0) || (my_route_distance < smallest_tour.distance) {
                smallest_tour.distance = my_route_distance;
                smallest_tour.route = my_route;
            }

            return;
        }

        _brute_force(
            distance_matrix,
            my_unvisited_cities,
            my_route,
            smallest_tour,
        );
    }
}
