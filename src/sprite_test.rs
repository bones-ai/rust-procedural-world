#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_sprites() {
    let mut map = _get_random_map(&Size { x: 45, y: 45 });

    map = cellular_automata_do_steps(&mut map);

    println!("{}", map.len());
    println!("{}", map[0].len());
    println!("{:?}", map);
}
