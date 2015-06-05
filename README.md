# NAME

travelling_salesman - Travelling Salesman Problem Solvers

The aim of this crate is to host various Travelling Salesman Problem solvers.
Patches implementing useful algorithms most welcome.

# USAGE

    extern crate travelling_salesman;

    use travelling_salesman::brute_force;

    fn main() {
        let cities = vec![
            (27.0, 78.0),
            (18.0, 24.0),
            (48.0, 62.0),
            (83.0, 17.0),
        ];

        let tour = brute_force::solve(&cities);
        println!("tour distance: {}, tour route: {:?}", tour.distance, tour.route);
    }

# SUPPORT

Please report any bugs or feature requests at:

* [https://github.com/alfiedotwtf/travelling_salesman/issues](https://github.com/alfiedotwtf/travelling_salesman/issues)

Watch the repository and keep up with the latest changes:

* [https://github.com/alfiedotwtf/travelling_salesman/subscription](https://github.com/alfiedotwtf/travelling_salesman/subscription)

Feel free to fork the repository and submit pull requests :)

# SEE ALSO

* [In Pursuit of the Traveling Salesman: Mathematics at the Limits of Computation](http://www.amazon.com/Pursuit-Traveling-Salesman-Mathematics-Computation/dp/0691152705)

# AUTHOR

[Alfie John](https://www.alfie.wtf) &lt;[alfie@alfie.wtf](mailto:alfie@alfie.wtf)&gt;

# WARRANTY

IT COMES WITHOUT WARRANTY OF ANY KIND.

# COPYRIGHT AND LICENSE

Copyright (C) 2015 by Alfie John

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program. If not, see [http://www.gnu.org/licenses/](http://www.gnu.org/licenses/).
