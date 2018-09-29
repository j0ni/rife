use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash)]
pub struct LCell {
    pub x: i32,
    pub y: i32,
}

impl LCell {
    pub fn new(x: i32, y: i32) -> LCell {
        LCell { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    pub width: i32,
    pub height: i32,
    pub state: HashSet<LCell>,
}

impl Ord for LCell {
    fn cmp(&self, other: &LCell) -> Ordering {
        let x_cmp = self.x.cmp(&other.x);
        if x_cmp == Ordering::Equal {
            self.y.cmp(&other.y)
        } else {
            x_cmp
        }
    }
}

impl PartialOrd for LCell {
    fn partial_cmp(&self, other: &LCell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LCell {
    fn eq(&self, other: &LCell) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Board {
    pub fn new(width: i32, height: i32) -> Board {
        Board {
            width,
            height,
            state: HashSet::new(),
        }
    }

    pub fn init(&mut self, initial_state: &Vec<(i32, i32)>) {
        for tup in initial_state.iter() {
            self.state.insert(LCell::new(tup.0, tup.1));
        }
    }

    fn neighbours(&self, cell: &LCell) -> HashSet<LCell> {
        let mut nbs: HashSet<LCell> = HashSet::new();

        for n in -1..2 {
            for m in -1..2 {
                let mut a = cell.x + n;
                let mut b = cell.y + m;

                if (cell.x, cell.y) != (a, b) {
                    if a >= self.width {
                        a = 0;
                    } else if a < 0 {
                        a = self.width - 1;
                    }

                    if b >= self.height {
                        b = 0;
                    } else if b < 0 {
                        b = self.height - 1;
                    }

                    nbs.insert(LCell::new(a, b));
                }
            }
        }

        assert_eq!(nbs.len(), 8);
        nbs
    }

    pub fn next_turn(&mut self) -> Board {
        let mut s: HashSet<LCell> = HashSet::new();

        for c in &self.state {
            let nbs = self.neighbours(&c);
            let living_nb_cnt = nbs.intersection(&self.state).count();
            if living_nb_cnt == 2 || living_nb_cnt == 3 {
                s.insert(c.clone());
            }
        }

        for m in 0..self.width {
            for n in 0..self.height {
                let c = LCell::new(m, n);
                if !self.state.contains(&c) {
                    let nbs = self.neighbours(&c);
                    let living_nb_cnt = nbs.intersection(&self.state).count();
                    if living_nb_cnt == 3 {
                        s.insert(c);
                    }
                }
            }
        }

        let mut board = self.clone();
        board.state = s;

        board
    }
}

impl Iterator for Board {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        Some(self.next_turn())
    }
}
