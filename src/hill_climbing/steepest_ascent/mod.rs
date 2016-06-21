extern crate metaheuristics;

use rand::thread_rng;
use time::{Duration};

use super::super::{
    get_distance_matrix,
    get_route_distance,
    Tour,
    TravellingSalesman,
};

pub fn solve(cities: &[(f64, f64)], duration: Duration, tries: u64) -> Tour {
    let mut tsp = TravellingSalesman {
        distance_matrix: &get_distance_matrix(cities),
        rng:             &mut thread_rng(),
    };

    let best_candidate = metaheuristics::hill_climbing::steepest_ascent::solve(&mut tsp, duration, tries);

    Tour {
        distance: get_route_distance(tsp.distance_matrix, &best_candidate.route),
        route:    best_candidate.route,
    }
}
