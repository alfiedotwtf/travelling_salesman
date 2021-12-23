//! Find an approximate solution to the Travelling Salesman Problem using Hill Climbing with random restarts
//!
//! For more information, please see the
//! [metaheuristics::hill_climbing::random_restarts](https://www.alfie.wtf/rustdoc/metaheuristics/metaheuristics/hill_climbing/random_restarts)
//! documentation.
//!
//!# Examples
//!
//!```
//!extern crate time;
//!extern crate travelling_salesman;
//!
//!fn main() {
//!  let tour = travelling_salesman::hill_climbing::random_restarts::solve(
//!    &[
//!       (27.0, 78.0),
//!       (18.0, 24.0),
//!       (48.0, 62.0),
//!       (83.0, 77.0),
//!       (55.0, 56.0),
//!    ],
//!    time::Duration::seconds(1),
//!    0.7,
//!  );
//!
//!  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
//!}
//!```
//!
extern crate metaheuristics;

use rand::thread_rng;
use time::Duration;

use super::super::{get_distance_matrix, get_route_distance, Tour, TravellingSalesman};

/// Returns an approximate solution to the Travelling Salesman Problem using Hill Climbing with random restarts
///
/// For more information, please see the
/// [metaheuristics::hill_climbing::random_restarts](https://www.alfie.wtf/rustdoc/metaheuristics/metaheuristics/hill_climbing/random_restarts/)
///
///# Parameters and Return Type
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// `runtime` is a `time::Duration`, specifying how long to spend searching for a solution.
///
/// `restart_probability` is a value within the range `[0.0, 1.0)` specifying the restart probability.
///
/// Returns a `travelling_salesman::Tour` struct, representing the approximate solution found.
///
///# Examples
///
///```
///extern crate time;
///extern crate travelling_salesman;
///
///fn main() {
///  let tour = travelling_salesman::hill_climbing::random_restarts::solve(
///    &[
///       (27.0, 78.0),
///       (18.0, 24.0),
///       (48.0, 62.0),
///       (83.0, 77.0),
///       (55.0, 56.0),
///    ],
///    time::Duration::seconds(1),
///    0.7,
///  );
///
///  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
///}
///```
pub fn solve(cities: &[(f64, f64)], runtime: Duration, restart_probability: f64) -> Tour {
    let mut tsp = TravellingSalesman {
        distance_matrix: &get_distance_matrix(cities),
        rng: &mut thread_rng(),
    };

    let best_candidate = metaheuristics::hill_climbing::random_restarts::solve(
        &mut tsp,
        runtime,
        restart_probability,
    );

    Tour {
        distance: get_route_distance(tsp.distance_matrix, &best_candidate.route),
        route: best_candidate.route,
    }
}
