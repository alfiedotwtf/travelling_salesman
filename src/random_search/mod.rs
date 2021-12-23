//! Find an approximate solution to the Travelling Salesman Problem using Random Search
//!
//! For more information, please see the
//! [metaheuristics::random_search](https://www.alfie.wtf/rustdoc/metaheuristics/metaheuristics/random_search)
//! documentation.
//!
//!# Examples
//!
//!```
//!extern crate time;
//!extern crate travelling_salesman;
//!
//!fn main() {
//!  let tour = travelling_salesman::random_search::solve(
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
extern crate metaheuristics;

use rand::thread_rng;
use time::Duration;

use super::{get_distance_matrix, get_route_distance, Tour, TravellingSalesman};

/// Returns an approximate solution to the Travelling Salesman Problem using Random Search
///
/// For more information, please see the
/// [metaheuristics::random_search](https://www.alfie.wtf/rustdoc/metaheuristics/metaheuristics/random_search/)
/// documentation.
///
///# Parameters and Return Type
///
/// `cities` is an array slice, containing `(x,y)` tuple coordinates for each city.
///
/// `runtime` is a `time::Duration`, specifying how long to spend searching for a solution.
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
///  let tour = travelling_salesman::random_search::solve(
///    &[
///       (27.0, 78.0),
///       (18.0, 24.0),
///       (48.0, 62.0),
///       (83.0, 77.0),
///       (55.0, 56.0),
///    ],
///    time::Duration::seconds(1),
///  );
///
///  println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
///}
///```
pub fn solve(cities: &[(f64, f64)], runtime: Duration) -> Tour {
    let mut tsp = TravellingSalesman {
        distance_matrix: &get_distance_matrix(cities),
        rng: &mut thread_rng(),
    };

    let best_candidate = metaheuristics::random_search::solve(&mut tsp, runtime);

    Tour {
        distance: get_route_distance(tsp.distance_matrix, &best_candidate.route),
        route: best_candidate.route,
    }
}
