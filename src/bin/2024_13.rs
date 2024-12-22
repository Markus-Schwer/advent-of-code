const INPUT: &str = include_str!("../../resources/2024_13.txt");

fn is_upper_triangular(matrix: &[[f64; 3]; 2]) -> bool {
    for i in 0..2 {
        for j in 0..i {
            if matrix[i][j] != 0.0 {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    let re = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)").unwrap();

    let mut total_cost = 0.0;

    for (_, [xa, ya, xb, yb, x, y]) in re.captures_iter(INPUT).map(|c| c.extract()) {
        let xa: f64 = xa.parse().unwrap();
        let ya: f64 = ya.parse().unwrap();
        let xb: f64 = xb.parse().unwrap();
        let yb: f64 = yb.parse().unwrap();
        let x: f64 = x.parse().unwrap();
        let x = x + 10000000000000.0;
        let y: f64 = y.parse().unwrap();
        let y = y + 10000000000000.0;

        let mut coefficient_matrix = [[xa, xb, x], [ya, yb, y]];
        const N: usize = 2;

        let mut i = 0;
        while i < N {
            let pivot = coefficient_matrix[i][i];

            if pivot == 0.0 {
                for j in i+1..N {
                    if coefficient_matrix[j][i] != 0.0 {
                        coefficient_matrix.swap(j, i);
                        break;
                    }
                    i -= 1;
                }
            }

            for j in i+1..N {
                let ratio = coefficient_matrix[j][i] / pivot;
                for k in 0..N+1 {
                    coefficient_matrix[j][k] -= ratio * coefficient_matrix[i][k];
                }
            }

            i += 1;

            if is_upper_triangular(&coefficient_matrix) {
                break;
            }
        }

        let mut solutions = [0.; N];
        for i in (0..N).rev() {
            let rhs_only: f64 = (i..N).map(|j| coefficient_matrix[i][j] * solutions[j]).sum();
            solutions[i] = (coefficient_matrix[i][N] - rhs_only) / coefficient_matrix[i][i];
        }

        let (a, b) = (solutions[0].round_ties_even(), solutions[1].round_ties_even());

        if x != xa*a + xb*b || y != ya*a + yb*b {
            continue;
        }

        let cost = a * 3.0 + b;
        total_cost += cost;
    }

    println!("total cost {}", total_cost);
}
