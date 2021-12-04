#[cfg(test)]

mod board_tests {
    use crate::board::BingoBoard;


    #[test]
    fn no_picks_give_no_wins() {
        let input = vec![
          "0 0 0 0 0",
          "0 0 0 0 0",
          "0 0 0 0 0",
          "0 0 0 0 0",
          "0 0 0 0 0"
        ];

        let board = BingoBoard::new(input);

        assert_eq!(false, board.has_bingo());
    }

    #[test]
    fn picks_for_first_row_gives_bingo() {
        let input = vec![
            "1 2 3 4 5",
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(true, board.has_bingo());
    }

    #[test]
    fn picks_for_second_row_gives_bingo() {
        let input = vec![
            "0 0 0 0 0",
            "1 2 3 4 5",
            "0 0 0 0 0",
            "0 0 0 0 0",
            "0 0 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(true, board.has_bingo());
    }

    #[test]
    fn picks_for_first_column_gives_bingo() {
        let input = vec![
            "1 0 0 0 0",
            "2 0 0 0 0",
            "3 0 0 0 0",
            "4 0 0 0 0",
            "5 0 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(true, board.has_bingo());
    }

    #[test]
    fn picks_for_second_column_gives_bingo() {
        let input = vec![
            "0 1 0 0 0",
            "0 2 0 0 0",
            "0 3 0 0 0",
            "0 4 0 0 0",
            "0 5 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(true, board.has_bingo());
    }

    #[test]
    fn score_for_unmarked_0_is_0() {
        let input = vec![
            "0 1 0 0 0",
            "0 2 0 0 0",
            "0 3 0 0 0",
            "0 4 0 0 0",
            "0 5 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(0, board.get_score());
    }

    #[test]
    fn score_for_unmarked_6_and_last_called_is_30() {
        let input = vec![
            "0 1 0 0 0",
            "0 2 0 0 0",
            "0 3 0 6 0",
            "0 4 0 0 0",
            "0 5 0 0 0"
        ];

        let mut board = BingoBoard::new(input);
        let picks = vec![1,2,3,4,5];

        for pick in picks {
            board.pick_number(pick);
        }

        assert_eq!(30, board.get_score());
    }
}