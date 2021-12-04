use std::str::Lines;
use crate::board::BingoBoard;

pub struct BingoDriver {
    boards: Vec<BingoBoard>,
    numbers: Vec<u8>,
}

impl BingoDriver {
    pub fn new(lines: &mut Lines) -> BingoDriver {
        let mut proto = BingoDriver {
            numbers: vec![],
            boards: vec![],
        };

        let numbers_line = lines.next().unwrap();
        proto.numbers = numbers_line
            .split(",")
            .map(|v| v
                .parse()
                .unwrap())
            .collect();

        while lines.next().is_some() {
            let board_strings: Vec<&str> = lines.take(5).collect();
            proto.boards.push(BingoBoard::new(board_strings))
        }

        proto
    }

    pub fn process(&mut self) -> u32 {
        for n in &self.numbers {
            for board in &mut self.boards {
                board.pick_number(*n);

                if board.has_bingo() {
                    return board.get_score();
                }
            }
        }
        0
    }

    pub fn find_last(&mut self) -> u32 {
        let mut last_score = 0;

        for n in &self.numbers {
            for board in &mut self.boards {
                board.pick_number(*n);

                if board.has_bingo() && !board.has_won {
                    last_score = board.get_score();
                    board.has_won = true
                }
            }
        }
        last_score
    }
}