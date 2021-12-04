use regex::Regex;

struct BingoRow {
    values: Vec<u8>,
}

impl BingoRow {
    fn new(s: &str) -> BingoRow {
        let mut proto = BingoRow { values: vec![] };
        let regex = Regex::new(r"\s+").unwrap();
        let values = regex.split(s);
        for v in values {
            let new_value = v.parse::<u8>();
            match new_value {
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

    pub fn pick_number(&mut self, n: u8) {
        self.numbers.push(n)
    }

    pub fn has_bingo(&self) -> bool {
        for row in &self.rows {
            if self.scan_row(row) {
                return true;
            }
        }

        for col_index in 0..self.rows[0].values.len() {
            if self.scan_col(col_index) {
                return true;
            }
        }

        false
    }

    fn scan_col(&self, col_index: usize) -> bool {
        for row_index in 0..self.rows.len() {
            if !self.numbers.contains(&self.get_value(col_index, row_index)) {
                return false;
            }
        }
        true
    }

    fn scan_row(&self, row: &BingoRow) -> bool {
        for column_value in &row.values {
            if !self.numbers.contains(&column_value) {
                return false;
            }
        }
        true
    }

    pub fn get_score(&self) -> u32 {
        let mut sum: u32 = 0;
        for row_index in 0..self.rows.len() {
            for col_index in 0..self.rows[row_index].values.len() {
                let v = self.get_value(col_index, row_index);
                if !self.numbers.contains(&v) {
                    sum += v as u32
                }
            }
        }
        sum * (*self.numbers.last().unwrap() as u32)
    }
}
