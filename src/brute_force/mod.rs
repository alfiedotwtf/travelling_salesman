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
//!     let cities = vec![
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

use super::*;

use std::collections::HashSet;
use std::iter::FromIterator;

/// Function that exactly solves by brute force search the Travelling Salesman Problem.
///
/// # Example
/// ```
/// extern crate travelling_salesman;
///
/// use travelling_salesman::brute_force;
///
/// fn main() {
///     let cities = vec![
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
pub fn solve(cities: &Vec<(f64, f64)>) -> Tour {
    let mut smallest_tour = Tour { distance: 0.0, route: vec![] };

    _brute_force(
        &get_distance_matrix(cities),
        HashSet::<u32>::from_iter((0..cities.len() as u32).collect::<Vec<u32>>()),
        vec![],
        &mut smallest_tour,
    );

    smallest_tour
}

fn _brute_force(
    distance_matrix: &Vec<Vec<f64>>,
    unvisited_cities: HashSet<u32>,
    current_route: Vec<u32>,
    smallest_tour: &mut Tour
) {
    for unvisited_city in &unvisited_cities {
        let mut my_unvisited_cities = unvisited_cities.clone();
        my_unvisited_cities.remove(&unvisited_city);

        let mut my_route = current_route.clone();
        my_route.push(*unvisited_city);

        if my_unvisited_cities.is_empty() {
            let first_city = my_route[0];
            my_route.push(first_city);

            let my_route_distance = get_route_distance(&distance_matrix, &my_route);

            if (smallest_tour.distance == 0.0) || (my_route_distance < smallest_tour.distance) {
                smallest_tour.distance = my_route_distance;
                smallest_tour.route    = my_route;
            }

            return;
        }

        _brute_force(distance_matrix, my_unvisited_cities, my_route, smallest_tour);
    }
}
