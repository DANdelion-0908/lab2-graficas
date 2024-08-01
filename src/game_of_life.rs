use std::fmt;

#[derive(Clone)]
pub struct GameOfLife {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl GameOfLife {
    // Crear un nuevo juego de la vida con el tamaño especificado
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![false; width]; height];
        GameOfLife { width, height, grid }
    }

    // Inicializar el tablero con un patrón
    pub fn initialize(&mut self, pattern: Vec<(usize, usize)>) {
        for (x, y) in pattern {
            if x < self.width && y < self.height {
                self.grid[y][x] = true;
            }
        }
    }

    // Obtener el estado de una célula (considerando los bordes)
    fn get_cell(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            false
        } else {
            self.grid[y][x]
        }
    }

    // Contar los vecinos vivos alrededor de una célula
    fn count_alive_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in x.saturating_sub(1)..=x.saturating_add(1) {
            for j in y.saturating_sub(1)..=y.saturating_add(1) {
                if (i != x || j != y) && self.get_cell(i, j) {
                    count += 1;
                }
            }
        }
        count
    }

    // Actualizar el tablero al siguiente estado
    pub fn update(&mut self) {
        let mut new_grid = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let alive_neighbors = self.count_alive_neighbors(x, y);
                new_grid[y][x] = match (self.grid[y][x], alive_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.grid = new_grid;
    }
}

// Implementar fmt::Display para imprimir el tablero
impl fmt::Display for GameOfLife {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for &cell in row {
                let symbol = if cell { '■' } else { ' ' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
