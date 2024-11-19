use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    
    let (player_token_type, enemy_token_type) = if player_number == '1' {
        (vec!['@','a'], vec!['$','s'])
    } else {
        (vec!['$','s'], vec!['@','a'])
    };

    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
        let number_of_grid = grid_details[..grid_details.len()-1].parse::<usize>().unwrap();

        let mut grid = Vec::new();
        let mut all_player_coordonate = Vec::new();
        let mut all_enemy_coordonate = Vec::new();

        for i in 0..number_of_grid + 1 {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if i < 1 { continue; }
            
            let row: Vec<char> = input[4..input.len()-1].chars().collect();
            for j in 0..row.len() {
                if player_token_type.contains(&row[j]) { all_player_coordonate.push((j,i)); }
                if enemy_token_type.contains(&row[j]) { all_enemy_coordonate.push((j,i)); }
            }
            grid.push(row);
        }
        
        let mut piece = Vec::new();
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let piece_details = input.split_whitespace().collect::<Vec<&str>>();
        let piece_lines = piece_details[2][..piece_details[2].len()-1].parse::<i32>().unwrap();

        for _ in 0..piece_lines {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let row: Vec<char> = input[..input.len()-1].chars().collect();
            piece.push(row);
        }

        let (piece_x, piece_y) = place_piece(&grid, &piece, &all_player_coordonate, &all_enemy_coordonate, &player_token_type);
        println!("{} {}", piece_x, piece_y);
    }
}

fn place_piece( grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, occupied_positions: &Vec<(usize, usize)>, target_positions: &Vec<(usize, usize)>, piece_chars: &Vec<char> ) -> (usize, usize) {
    let grid_width = grid[0].len();   // Largeur de la grille
    let piece_width = piece[0].len(); // Largeur de la pièce
    let mut best_distance = f32::MAX; // Meilleure distance trouvée
    let mut best_position = (0, 0);  // Meilleure position trouvée

    // Déterminer les bornes des positions occupées dans la grille
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (grid.len(), 0, grid_width, 0);
    for (x, y) in occupied_positions {
        min_x = min_x.min(*x);
        max_x = max_x.max(*x);
        min_y = min_y.min(*y);
        max_y = max_y.max(*y);
    }

    // Définir les limites de la zone de recherche
    let (mut start_x, mut end_x, mut start_y, mut end_y) = (0, grid_width - piece_width + 1, 0, grid.len() - piece.len() + 1);
    
    // Ajuster les limites horizontales
    let x_offset = min_x as i32 - piece_width as i32 - 1;
    if x_offset > 0 {
        start_x = min_x - piece_width + 1;
    }
    if max_x + piece_width - 1 < grid_width {
        end_x = max_x + 1;
    }

    // Ajuster les limites verticales
    let y_offset = min_y as i32 - piece.len() as i32 + 1;
    if y_offset > 0 {
        start_y = min_y - piece.len();
    }
    if max_y + piece.len() - 1 < grid.len() {
        end_y = max_y;
    }

    // Parcourir les positions possibles dans la zone de recherche
    for y in start_y..end_y {
        for x in start_x..end_x {
            if is_piece_can_be_place(grid, piece, piece_chars, x, y) {
                let distance = calculate_manhattan_distance(piece, target_positions, (x, y));
                if distance < best_distance {
                    best_distance = distance;
                    best_position = (x, y);
                }
            }
        }
    }

    best_position
}

fn calculate_manhattan_distance(piece: &Vec<Vec<char>>, all_enemy_coordonate: &Vec<(usize,usize)>, (xg, yg): (usize, usize)) -> f32 {
    let mut min_dist = f32::MAX;

    for yp in 0..piece.len() {
        for xp in 0..piece[0].len() {
            if piece[yp][xp] != '.' {
                for (xe, ye) in all_enemy_coordonate {
                    //  let  dist=(((*ye as f32)-((yp + yg) as f32) ).powf(2.) + ((*xe as f32)-((xp + xg) as f32)).powf(2.)).sqrt();
                    
                    let dist = ((*xe as isize - (xg + xp) as isize).abs() + 
                                (*ye as isize - (yg + yp) as isize).abs()) as f32;
                    min_dist = min_dist.min(dist);
                }
            }
        }
    }
     
    min_dist
}

fn is_piece_can_be_place(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, player_token_type: &Vec<char>, xg: usize, yg: usize) -> bool {
    let mut cross = 0;
    let prows = piece[0].len();

    for yp in 0..piece.len() {
        for xp in 0..prows {
            if piece[yp][xp] != '.' {
                if player_token_type.contains(&grid[yg+yp][xg+xp]) {
                    cross += 1;
                    if cross > 1 { return false; }
                } else if grid[yg+yp][xg+xp] != '.' {
                    return false;
                }
            }
        }
    }
    
    cross == 1
}