extern crate metaheuristics;

pub mod random_restarts;
pub mod steepest_ascent;

use time::{Duration};
use rand::thread_rng;

use super::{
    get_route_distance,
    get_distance_matrix,
    Tour,
    TravellingSalesman,
};

pub fn solve(cities: &[(f64, f64)], duration: Duration) -> Tour {
    let mut tsp = TravellingSalesman {
        distance_matrix: &get_distance_matrix(cities),
        rng:             &mut thread_rng(),
    };

    let best_candidate = metaheuristics::hill_climbing::solve(&mut tsp, duration);

    Tour {
        distance: get_route_distance(tsp.distance_matrix, &best_candidate.route),
        route:    best_candidate.route,
    }
}
