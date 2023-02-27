use super::in_bounds::in_bounds;

pub fn get_king_moves(square: (i8, i8), occupied_self: Vec<(i8, i8)>, occupied_enemy: Vec<(i8, i8)>) -> Vec<(i8, i8)>{
    
    let king_movement: Vec<(i8, i8)> = vec![
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
        (-1, -1)
    ];

    let mut possible_squares: Vec<(i8, i8)> = Vec::new();
    for movement in king_movement {
        let mut rank = square.0 + movement.0;
        let mut file = square.1 + movement.1;

        let coordinates = (rank, file);
        // checks if values are out of bounds, or if it's occupied by own pieces
        if in_bounds(rank, file) || occupied_self.contains(&coordinates) {
            stop = true;
        } else if occupied_enemy.contains(&coordinates) {
            possible_squares.push(coordinates);
            stop = true;
        } else {
            possible_squares.push((rank,file));
        }
    }
    possible_squares
}