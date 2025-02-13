use std::collections::{HashSet, HashMap, VecDeque};
fn _p1(_input: &str) -> usize {
    // Parsear el mapa en una matriz de enteros
    let grid: Vec<Vec<u32>> = _input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_score = 0;

    // Direcciones para moverse (arriba, abajo, izquierda, derecha)
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Buscar todos los trailheads (posiciones con valor 0)
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                // BFS para calcular las posiciones alcanzables con un incremento gradual
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_back((r, c));
                visited.insert((r, c));
                let mut reachable_nines = HashSet::new();

                while let Some((x, y)) = queue.pop_front() {
                    for &(dx, dy) in &directions {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0
                            && ny >= 0
                            && (nx as usize) < rows
                            && (ny as usize) < cols
                            && !visited.contains(&(nx as usize, ny as usize))
                        {
                            let nx = nx as usize;
                            let ny = ny as usize;
                            // Solo movernos si el valor es 1 más que el actual
                            if grid[nx][ny] == grid[x][y] + 1 {
                                queue.push_back((nx, ny));
                                visited.insert((nx, ny));

                                // Si alcanzamos un 9, añadirlo al conjunto de alcanzables
                                if grid[nx][ny] == 9 {
                                    reachable_nines.insert((nx, ny));
                                }
                            }
                        }
                    }
                }

                // El score del trailhead es el número de posiciones 9 alcanzables
                total_score += reachable_nines.len();
            }
        }
    }

    total_score
}

fn _p2(_input: &str) -> usize {
    // Parsear el mapa en una matriz de enteros
    let grid: Vec<Vec<u32>> = _input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_rating = 0;

    // Direcciones para moverse (arriba, abajo, izquierda, derecha)
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    // Función para calcular los caminos desde una posición usando DFS
    fn dfs(
        grid: &Vec<Vec<u32>>,
        x: usize,
        y: usize,
        visited: &mut HashSet<(usize, usize)>,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        // Si ya calculamos los caminos desde esta posición, usamos el memo
        if let Some(&count) = memo.get(&(x, y)) {
            return count;
        }

        let mut total_paths = 0;
        visited.insert((x, y));

        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0
                && ny >= 0
                && (nx as usize) < grid.len()
                && (ny as usize) < grid[0].len()
            {
                let nx = nx as usize;
                let ny = ny as usize;

                if !visited.contains(&(nx, ny)) && grid[nx][ny] == grid[x][y] + 1 {
                    total_paths += dfs(grid, nx, ny, visited, memo);
                }
            }
        }

        // Si estamos en un punto final (un 9), cuenta como 1 camino
        if grid[x][y] == 9 {
            total_paths += 1;
        }

        visited.remove(&(x, y));
        memo.insert((x, y), total_paths);
        total_paths
    }

    // Buscar todos los trailheads (posiciones con valor 0)
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == 0 {
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
                let rating = dfs(&grid, r, c, &mut visited, &mut memo);
                total_rating += rating;
            }
        }
    }

    total_rating
}



pub fn p1() -> usize {
    _p1(include_str!("d10.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d10.txt"))
}

#[cfg(test)]
mod tests {
    use crate::d10::*;

    #[test]
    fn test_p1() {
        assert_eq!(36, _p1(include_str!("d10_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(81, _p2(include_str!("d10_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(510, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(1058, p2());
    }
}