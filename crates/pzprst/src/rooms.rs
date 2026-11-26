//! Room components derived from pzpr question borders.

use crate::board::{Board, Group};

/// Components follow the reference graph's cell scan order; room tops are
/// selected by column, then row, as in `CellList.getTopCell`.
pub(crate) fn components(b: &Board) -> Vec<Vec<usize>> {
    let cols = b.cols();
    let rows = b.rows();
    let mut seen = vec![false; cols * rows];
    let mut rooms = Vec::new();
    for start in 0..seen.len() {
        if seen[start] || b.genre.room_cell_excluded(&b.cells[start]) {
            continue;
        }
        let mut room = Vec::new();
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(i) = stack.pop() {
            room.push(i);
            let x = i % cols;
            let y = i / cols;
            for (nx, ny) in [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ] {
                if nx >= cols || ny >= rows {
                    continue;
                }
                let next = ny * cols + nx;
                if seen[next] || b.genre.room_cell_excluded(&b.cells[next]) {
                    continue;
                }
                let bx = (x + nx + 1) as i32;
                let by = (y + ny + 1) as i32;
                if b.obj(Group::Border, bx, by)
                    .is_some_and(|edge| b.genre.room_border_is_wall(edge))
                {
                    continue;
                }
                seen[next] = true;
                stack.push(next);
            }
        }
        rooms.push(room);
    }
    rooms
}

pub(crate) fn top(room: &[usize], cols: usize) -> usize {
    *room.iter().min_by_key(|&&i| (i % cols, i / cols)).unwrap()
}

/// Move the room clue to the top cell after a board operation.
pub(crate) fn normalize_clues(b: &mut Board) {
    for room in components(b) {
        let top = top(&room, b.cols());
        let value = (b.cells[top].qnum != -1)
            .then_some(b.cells[top].qnum)
            .or_else(|| {
                room.iter()
                    .find_map(|&i| (b.cells[i].qnum != -1).then_some(b.cells[i].qnum))
            });
        for &i in &room {
            b.cells[i].qnum = -1;
        }
        if let Some(value) = value {
            b.cells[top].qnum = value;
        }
    }
}
