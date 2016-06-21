//! Brute Force Travelling Salesman Problem Solver
//!
//! This crate implements an exact solution brute force search for the Travelling Salesman
//! Problem.
//!
//! # Examples
//! ```
//! extern crate travelling_salesman;
//!
//! use travelling_salesman::brute_force;
//!
//! fn main() {
//!     let cities = [
//!         (27.0, 78.0),
//!         (18.0, 24.0),
//!         (48.0, 62.0),
//!         (83.0, 17.0),
//!     ];
//!
//!     let tour = brute_force::solve(&cities);
//!     println!("tour distance: {}, tour route: {:?}", tour.distance, tour.route);
//! }
//! ```

use std::collections::HashSet;
use std::iter::FromIterator;

use super::{
    get_distance_matrix,
    get_route_distance,
    Tour,
};

/// Function that exactly solves by brute force search the Travelling Salesman Problem.
///
/// # Example
/// ```
/// extern crate travelling_salesman;
///
/// use travelling_salesman::brute_force;
///
/// fn main() {
///     let cities = [
///         (27.0, 78.0),
///         (18.0, 24.0),
///         (48.0, 62.0),
///         (83.0, 17.0),
///     ];
///
///     let tour = brute_force::solve(&cities);
///     println!("tour distance: {}, tour route: {:?}", tour.distance, tour.route);
/// }
/// ```
pub fn solve(cities: &[(f64, f64)]) -> Tour {
    let mut smallest_tour = Tour {
        distance: 0.0,
        route: vec![]
    };

    if cities.len() == 0 {
        return smallest_tour;
    }

    let mut unvisited_cities = HashSet::<usize>::from_iter((0..cities.len()).collect::<Vec<usize>>());
    let current_route        = vec!(0);
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
    distance_matrix:  &Vec<Vec<f64>>,
    unvisited_cities: HashSet<usize>,
    current_route:    Vec<usize>,
    smallest_tour:    &mut Tour
) {
    for unvisited_city in &unvisited_cities {
        let mut my_unvisited_cities = unvisited_cities.clone();
        my_unvisited_cities.remove(&unvisited_city);

        let mut my_route = current_route.clone();
        my_route.push(*unvisited_city);

        if my_unvisited_cities.is_empty() {
            let home_city = my_route[0];
            my_route.push(home_city);

            let my_route_distance = get_route_distance(&distance_matrix, &my_route);

            if (smallest_tour.distance == 0.0) || (my_route_distance < smallest_tour.distance) {
                smallest_tour.distance = my_route_distance;
                smallest_tour.route    = my_route;
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
