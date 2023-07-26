use movement1::Game;


fn main() {
    let mut game = Game::new();

    let mut positions = [0u32; 40];

    for _ in 0..1_000_000_000 {
        for position in game.take_turn().into_iter() {
            let index = position as usize;
            positions[index] += 1;
        }
    }

    for (i, &count) in positions.iter().enumerate() {
        println!("Landed at position {0} {1} times", i, count);
    }
}
