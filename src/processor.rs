use mematrica::{CMatrix, CMatrixTrait, Matrix};

use crate::vec_to_map;

pub fn build_reactions_matrix(amount: usize, rects: &[(usize, usize)], e: &[usize], k: &[usize]) -> CMatrix<f64> {
    let n = amount + 1;
    let mut matrix = CMatrix::<f64>::zero(n, n);
    let mut reactions = vec![];

    for i in 0..amount {
        reactions.push(k[i] as f64 * e[i] as f64 * rects[i].0 as f64 / rects[i].1 as f64);
    }

    for i in 0..amount {
        if matrix[(i, i)] == 0.0 {
            matrix[(i, i)] = reactions[i];
            if i < amount - 1 {
                matrix[(i + 1, i + 1)] = reactions[i + 1] + reactions[i];
            }
        }
        matrix[(i, i + 1)] = -reactions[i];
        matrix[(i + 1, i)] = -reactions[i];
    }

    matrix[(amount, amount)] = reactions[amount - 1];
    matrix[(amount - 1, amount)] = -reactions[amount - 1];
    matrix[(amount, amount - 1)] = -reactions[amount - 1];
    matrix[(amount - 1, amount - 1)] = reactions[amount - 2] + reactions[amount - 1];

    //println!("{:?}", matrix);

    matrix
}

pub fn build_movements_vector(amount: usize, rects: &[(usize, usize)], distributed_loads: &[(i32, i32)], point_loads: &[(i32, i32)]) -> CMatrix<f64> {
    let n = amount + 1;
    let mut vector = CMatrix::<f64>::zero(n, 1);        
    let mut movements = vec![];
    let m_dist = vec_to_map(&distributed_loads[..]);
    let m_point = vec_to_map(&point_loads[..]);

    for i in 1..amount {
        let p = m_point.get(&(i as i32)).unwrap_or(&0);
        let p = *p as f64;

        let d1 = m_dist.get(&(i as i32)).unwrap_or(&0);
        let d1 = *d1 as f64;

        let d2 = m_dist.get(&((i + 1) as i32)).unwrap_or(&0);
        let d2 = *d2 as f64;

        let sum = p + (d1 * (rects[i - 1].1) as f64 / 20.0) + (d2 * (rects[i].1) as f64 / 20.0);
        movements.push(sum);
    }

    for i in 1..amount {
        vector[(i, 0)] = movements[i - 1];
    }

    vector[(0, 0)] = 0.0;
    vector[(amount, 0)] = 0.0;

    vector
}

pub fn count_deltas(amount: usize, reactions: &CMatrix<f64>, movements: &CMatrix<f64>) -> Option<Vec<f64>> {
    let n = amount + 1;

    let mut coefs = CMatrix::<f64>::zero(n, n);
    for i in 1..amount {
        for j in 1..amount {
            coefs[(i, j)] = reactions[(i, j)];
        }
    }
    
    coefs[(0, 0)] = 1.0;
    coefs[(amount, amount)] = 1.0;


    let mut v = vec![];
    for i in 0..n {
        v.push(movements[(i, 0)]);
    }

    println!("Deltas = {:?}", solve_system_equations(&coefs.get_elements(), &v));

    solve_system_equations(&coefs.get_elements(), &v)
}

fn solve_system_equations(coefficients: &Vec<Vec<f64>>, constants: &Vec<f64>) -> Option<Vec<f64>> {
    let n = coefficients.len();
    let m = coefficients[0].len();

    if n != m {
        return None;
    }

    let mut matrix = coefficients.clone();
    let mut vector = constants.clone();

    for i in 0..n {
        let pivot = matrix[i][i];

        if pivot == 0.0 {
            let mut found = false;

            for j in (i + 1)..n {
                if matrix[j][i] != 0.0 {
                    matrix.swap(i, j);
                    vector.swap(i, j);
                    found = true;
                    break;
                }
            }

            if !found {
                return None;
            }
        }

        for j in i..n {
            matrix[i][j] /= pivot;
        }

        vector[i] /= pivot;

        for j in (i + 1)..n {
            let factor = matrix[j][i];

            for k in i..n {
                matrix[j][k] -= factor * matrix[i][k];
            }

            vector[j] -= factor * vector[i];
        }
    }

    for i in (0..n).rev() {
        for j in (0..i).rev() {
            let factor = matrix[j][i];

            for k in 0..n {
                matrix[j][k] -= factor * matrix[i][k];
            }

            vector[j] -= factor * vector[i];
        }
    }

    Some(vector)
}

fn nx(e: f64, a: f64, l: f64, diff: f64, q: f64, coeff: f64) -> f64 {
    (e * a / l * diff) + (q * l / 20.0 * coeff)
}

pub fn count_forces(amount: usize, rects: &[(usize, usize)], e: &[usize], k: &[usize], reactions: &CMatrix<f64>, movements: &CMatrix<f64>, distributed_loads: &[(i32, i32)]) -> Vec<(f64, f64)> {

    let deltas = count_deltas(amount, reactions, movements).unwrap();
    let m_dist = vec_to_map(&distributed_loads[..]);

    let mut diff = vec![];
    diff.push(0.0);
    for i in 1..amount+1 {
        diff.push(deltas[i] - deltas[i - 1]);
    }
    diff.push(0.0);

    let mut n = vec![];
    for i in 0..amount {
        let q1 = *m_dist.get(&((i + 1) as i32)).unwrap_or(&0) as f64;
        n.push((nx(e[i] as f64, rects[i].0 as f64, rects[i].1 as f64, diff[i+1], q1 as f64, 1.0), nx(e[i] as f64, rects[i].0 as f64, rects[i].1 as f64, diff[i+1], q1 as f64, -1.0)));
    }

    println!("Nx = {:?}", n);
    n
}