use regex::Regex;

struct BingoRow {
    values: Vec<u8>,
}

impl BingoRow {
    fn new(input_strings: &str) -> BingoRow {
        let mut proto = BingoRow { values: vec![] };
        let regex = Regex::new(r"\s+").unwrap();

        for token in regex.split(input_strings) {
            match token.parse::<u8>() {
                Ok(n) => proto.values.push(n),
                _ => {}
            }
        }
        proto
    }
}

pub struct BingoBoard {
    rows: Vec<BingoRow>,
    pub has_won: bool,
    numbers: Vec<u8>,
}

impl BingoBoard {
    pub fn new(rows: Vec<&str>) -> BingoBoard {
        let mut proto = BingoBoard { rows: vec![], numbers: vec![], has_won: false };

        for row in rows {
            proto.rows.push(BingoRow::new(row))
        }
        proto
    }

    fn get_value(&self, col: usize, row: usize) -> u8 {
        self.rows[row].values[col]
    }

    fn get_width(&self) -> usize {
        self.rows[0].values.len()
    }

    fn get_height(&self) -> usize {
        self.rows.len()
    }

    pub fn pick_number(&mut self, n: u8) {
        self.numbers.push(n)
    }

    pub fn has_bingo(&self) -> bool {
        for row_index in 0..self.get_height() {
            if self.scan_row(row_index) {
                return true;
            }
        }

        for col_index in 0..self.get_width() {
            if self.scan_col(col_index) {
                return true;
            }
        }

        false
    }

    fn scan_col(&self, col_index: usize) -> bool {
        for row_index in 0..self.get_height() {
            if !self.numbers.contains(&self.get_value(col_index, row_index)) {
                return false;
            }
        }
        true
    }

    fn scan_row(&self, row_index: usize) -> bool {
        for col_index in 0..self.get_width() {
            if !self.numbers.contains(&self.get_value(col_index, row_index)) {
                return false;
            }
        }
        true
    }

    pub fn get_score(&self) -> u32 {
        let mut sum: u32 = 0;
        for row_index in 0..self.get_height() {
            for col_index in 0..self.get_width() {
                let v = self.get_value(col_index, row_index);
                if !self.numbers.contains(&v) {
                    sum += v as u32
                }
            }
        }
        sum * (*self.numbers.last().unwrap() as u32)
    }
}
