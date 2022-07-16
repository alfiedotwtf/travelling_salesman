# Travelling Salesman

Travelling Salesman Problem Solvers

The aim of this crate is to host various Travelling Salesman Problem solvers.
Patches implementing useful algorithms most welcome.

For more information, please see the [Travelling Salesman
Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem) Wikipedia
article, and [In Pursuit of the Traveling Salesman: Mathematics at the Limits
of Computation](https://www.amazon.com/Pursuit-Traveling-Salesman-Mathematics-Computation/dp/0691152705).

The documentation for this crate can be [found
here](https://docs.rs/travelling_salesman).

# Examples

    extern crate time;
    extern crate travelling_salesman;

    fn main() {
      let tour = travelling_salesman::simulated_annealing::solve(
        &[
           (27.0, 78.0),
           (18.0, 24.0),
           (48.0, 62.0),
           (83.0, 77.0),
           (55.0, 56.0),
        ],
        time::Duration::seconds(1),
      );

      println!("Tour distance: {}, route: {:?}", tour.distance, tour.route);
    }

# Support

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/travelling_salesman/issues](https://github.com/alfiedotwtf/travelling_salesman/issues)

Feel free to fork the repository and submit pull requests :)

# Author

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# Warranty

IT COMES WITHOUT WARRANTY OF ANY KIND.

# Copyright and License

Copyright (C) 2021 to Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [https://www.gnu.org/licenses/](https://www.gnu.org/licenses/).
