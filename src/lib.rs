//! Travelling Salesman Problem Solvers.
//!
//! The aim of this crate is to host various Travelling Salesman Problem solvers. Patches
//! implementing useful algorithms most welcome.
//!
//! For more information, please see the [Travelling Salesman
//! Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem) Wikipedia article, and [In
//! Pursuit of the Traveling Salesman: Mathematics at the Limits of
//! Computation](http://www.amazon.com/Pursuit-Traveling-Salesman-Mathematics-Computation/dp/0691152705)
//!
//! The documentation for this crate can be [found
//! here](https://www.alfie.wtf/rustdoc/travelling_salesman/travelling_salesman).
//!
//!# Examples
//!
//!```
//!extern crate time;
//!extern crate travelling_salesman;
//!
//!fn main() {
//!  let tour = travelling_salesman::simulated_annealing::solve(
//!    &[
//!       (27.0, 78.0),
//!       (18.0, 24.0),
//!       (48.0, 62.0),
//!       (83.0, 77.0),
//!       (55.0, 56.0),
//!    ],
//!    time::Duration::seconds(1),
//!  );
//!
//!  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
//!}
//!```
//!
//!# Support
//!
//! Please report any bugs or feature requests at:
//!
//! * [https://gitlab.com/alfiedotwtf/travelling_salesman/issues](https://gitlab.com/alfiedotwtf/travelling_salesman/issues)
//!
//! Feel free to fork the repository and submit pull requests :)
//!
//!# Author
//!
//! [Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;
//!
//!# Warranty
//!
//! IT COMES WITHOUT WARRANTY OF ANY KIND.
//!
//!# Copyright and License
//!
//! Copyright (C) 2015 by Alfie John
//!
//! This program is free software: you can redistribute it and/or modify it under the terms of the
//! GNU General Public License as published by the Free Software Foundation, either version 3 of
//! the License, or (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
//! without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See
//! the GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License along with this program. If
//! not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
pub mod brute_force;
pub mod hill_climbing;
pub mod random_search;
pub mod simulated_annealing;

extern crate metaheuristics;
extern crate rand;
extern crate time;

use metaheuristics::Metaheuristics;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;

struct TravellingSalesman<'a> {
    distance_matrix: &'a Vec<Vec<f64>>,
    rng: &'a mut ThreadRng,
}

struct Candidate {
    route: Vec<usize>,
}

impl<'a> Metaheuristics<Candidate> for TravellingSalesman<'a> {
    fn clone_candidate(&mut self, candidate: &Candidate) -> Candidate {
        Candidate {
            route: candidate.route.clone(),
        }
    }

    fn generate_candidate(&mut self) -> Candidate {
        let mut route: Vec<usize> = self
            .distance_matrix
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        route.shuffle(&mut self.rng);

        let home_city = route[0];
        route.push(home_city);

        Candidate { route }
    }

    fn rank_candidate(&mut self, candidate: &Candidate) -> f64 {
        0.0 - get_route_distance(self.distance_matrix, &candidate.route)
    }

    fn tweak_candidate(&mut self, candidate: &Candidate) -> Candidate {
        if candidate.route.len() <= 3 {
            return self.clone_candidate(candidate);
        }

        let mut old_route = candidate.route.clone();
        old_route.pop();

        // get two cities to work with

        let start = self.rng.gen::<usize>() % old_route.len();
        let end = self.rng.gen::<usize>() % old_route.len();
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        // straight swap of the cities

        let mut swapped_route = old_route.clone();
        swapped_route.swap(start, end);

        // swap cities, then reverse the cities between them

        let split_route = old_route.clone();
        let safe_offset = if old_route.len() <= (end + 1) {
            old_route.len()
        } else {
            end + 1
        };
        let (left, right) = split_route.split_at(safe_offset);
        let (left, middle) = left.split_at(start);

        let mut middle = middle.to_vec();
        middle.reverse();

        let mut reordered_route = Vec::new();
        reordered_route.extend(left.iter());
        reordered_route.extend(middle.iter());
        reordered_route.extend(right.iter());

        // return shortest route

        let swapped_distance = get_route_distance(self.distance_matrix, &swapped_route);
        let reordered_distance = get_route_distance(self.distance_matrix, &reordered_route);
        let mut shortest_route = if swapped_distance < reordered_distance {
            swapped_route
        } else {
            reordered_route
        };

        let home_city = shortest_route[0];
        shortest_route.push(home_city);

        Candidate {
            route: shortest_route,
        }
    }
}

/// Represents a tour of the travelling salesman
pub struct Tour {
    /// the total distance travelled following this tour
    pub distance: f64,
    /// the ordered route for this tour
    pub route: Vec<usize>,
}

/// Utility function to convert city coordinates to a distance matrix
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// Returns a `Vec<Vec<f64>>`, containing the distance matrix.
///
///# Examples
///
///```
///extern crate travelling_salesman;
///
///fn main() {
///    let cities = [
///      (27.0, 78.0),
///      (18.0, 24.0),
///      (48.0, 62.0),
///      (83.0, 77.0),
///      (55.0, 56.0),
///    ];
///
///    let distance_matrix = travelling_salesman::get_distance_matrix(&cities);
///
///    println!("The distance between 1 and 2 is: {}", distance_matrix[1][2]);
///}
///```
pub fn get_distance_matrix(cities: &[(f64, f64)]) -> Vec<Vec<f64>> {
    cities
        .iter()
        .map(|row| {
            cities
                .iter()
                .map(|column| ((column.0 - row.0).powi(2) + (column.1 - row.1).powi(2)).sqrt())
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

/// Utility function to calculate the distance travelled following the specified route
///
/// `distance_matrix` is a `&Vec<Vec<f64>>` containing the distance matrix.
///
/// `route` is a `&Vec<usize>`, containing the route of the travelling salesman.
///
/// Returns an `f64`, representing the distance of the route travelled.
///
///# Examples
///
///```
///extern crate travelling_salesman;
///
///fn main() {
///    let cities = [
///      (27.0, 78.0),
///      (18.0, 24.0),
///      (48.0, 62.0),
///      (83.0, 77.0),
///      (55.0, 56.0),
///    ];
///
///    let route_distance = travelling_salesman::get_route_distance(
///      &travelling_salesman::get_distance_matrix(&cities),
///      &vec![0, 2, 3, 4, 1, 0]
///    );
///
///    println!("The route distance for the tour [0, 2, 3, 4, 1, 0] is {}", route_distance);
///}
///```
pub fn get_route_distance(distance_matrix: &[Vec<f64>], route: &[usize]) -> f64 {
    let mut route_iter = route.iter();
    let mut current_city = match route_iter.next() {
        None => return 0.0,
        Some(v) => *v,
    };

    route_iter.fold(0.0, |mut total_distance, &next_city| {
        total_distance += distance_matrix[current_city as usize][next_city as usize];
        current_city = next_city;
        total_distance
    })
}
