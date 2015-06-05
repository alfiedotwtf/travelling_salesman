//! Travelling Salesman Problem Solvers.
//!
//! The aim of this crate is to host various Travelling Salesman Problem solvers. Patches
//! implementing useful algorithms most welcome.
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
//!
//! ```

pub mod brute_force;

/// Represents a tour of the travelling salesman
pub struct Tour {
    /// the total distance travelled following this tour
    pub distance: f64,
    /// the ordered route for this tour
    pub route:    Vec<u32>,
}

/// Utility function to convert city coordinates to a distance matrix
///
/// # Examples
/// ```
/// extern crate travelling_salesman;
///
/// use travelling_salesman::*;
///
/// fn main() {
///     let cities = vec![
///         (27.0, 78.0),
///         (18.0, 24.0),
///         (48.0, 62.0),
///         (83.0, 17.0),
///     ];
///
///     let distance_matrix = get_distance_matrix(&cities);
///
///     assert!(distance_matrix[0][0] == 0.0);
///     assert!(distance_matrix[1][2] == distance_matrix[2][1]);
///     assert!((distance_matrix[0][3] - 82.807005).abs() < 0.000001);
/// }
/// ```
pub fn get_distance_matrix(cities: &Vec<(f64, f64)>) -> Vec<Vec<f64>> {
    cities.iter().map(|row| {
        cities.iter().map(|column| {
            ((column.0 - row.0).powi(2) + (column.1 - row.1).powi(2)).sqrt()
        }).collect::<Vec<f64>>()
    }).collect::<Vec<Vec<f64>>>()
}

/// Utility function to calculate the distance travelled following the specified route
///
/// # Examples
/// ```
/// extern crate travelling_salesman;
///
/// use travelling_salesman::*;
///
/// fn main() {
///     let cities = vec![
///         (27.0, 78.0),
///         (18.0, 24.0),
///         (48.0, 62.0),
///         (83.0, 17.0),
///     ];
///
///     let distance_matrix = get_distance_matrix(&cities);
///     let route_distance  = get_route_distance(&distance_matrix, &vec![3, 0, 2, 1, 3]);
///
///     assert!((route_distance - 222.998472).abs() < 0.000001);
/// }
/// ```
pub fn get_route_distance(distance_matrix: &Vec<Vec<f64>>, route: &Vec<u32>) -> f64 {
    let mut route_iter   = route.iter();
    let mut current_city = match route_iter.next() {
        None    => return 0.0,
        Some(v) => *v,
    };

    route_iter.fold(0.0, |mut total_distance, &next_city| {
        total_distance += distance_matrix[current_city as usize][next_city as usize];
        current_city    = next_city;
        total_distance
    })
}
