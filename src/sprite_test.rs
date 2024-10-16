#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_sprites() {
    let mut map = _get_random_map(&Size { x: 45, y: 45 });

    map = cellular_automata_do_steps(&mut map);

    let scheme = colorscheme_generator_generate_new_colorscheme(12);
    let eye_scheme = colorscheme_generator_generate_new_colorscheme(12);
}
