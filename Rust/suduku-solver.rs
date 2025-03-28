impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = [0; 9];
        let mut cols = [0; 9];
        let mut boxes = [0; 9];
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] != '.' {
                    let mask = 1 << (board[r][c] as u8 - b'1');
                    rows[r] |= mask;
                    cols[c] |= mask;
                    boxes[(r / 3) * 3 + c / 3] |= mask;
                }
            }
        }
        Self::backtrack(board, &mut rows, &mut cols, &mut boxes);
    }

    fn backtrack(board: &mut Vec<Vec<char>>, rows: &mut [u16; 9], cols: &mut [u16; 9], boxes: &mut [u16; 9]) -> bool {
        match Self::find_min_candidates(board, rows, cols, boxes) {
            Some((r, c)) => {
                let index = (r / 3) * 3 + c / 3;
                let candidates = !(rows[r] | cols[c] | boxes[index]);
                for num in 1..=9 {
                    let mask = 1 << (num - 1);
                    if (candidates & mask) != 0 {
                        board[r][c] = (b'0' + num) as char;
                        rows[r] |= mask;
                        cols[c] |= mask;
                        boxes[index] |= mask;
                        if Self::backtrack(board, rows, cols, boxes) {
                            return true;
                        }
                        board[r][c] = '.';
                        rows[r] ^= mask;
                        cols[c] ^= mask;
                        boxes[index] ^= mask;
                    }
                }
                false
            }
            None => true,
        }
    }

    fn find_min_candidates(board: &Vec<Vec<char>>, rows: &[u16; 9], cols: &[u16; 9], boxes: &[u16; 9]) -> Option<(usize, usize)> {
        let mut min_candidates = 10;
        let mut candidate = None;
        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' {
                    let candidates = !(rows[r] | cols[c] | boxes[(r / 3) * 3 + c / 3]);
                    let count = candidates.count_ones();
                    if count < min_candidates {
                        min_candidates = count;
                        candidate = Some((r, c));
                        if min_candidates == 1 {
                            return candidate;
                        }
                    }
                }
            }
        }
        candidate
    }
}