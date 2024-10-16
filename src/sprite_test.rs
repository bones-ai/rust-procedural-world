#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_sprites() {
    let n_colors = 12;

    let mut map = _get_random_map(&Size { x: 45, y: 45 });

    map = cellular_automata_do_steps(&mut map);

    let scheme = colorscheme_generator_generate_new_colorscheme(n_colors);
    let eye_scheme = colorscheme_generator_generate_new_colorscheme(n_colors);

    let all_groups = color_filler_fill_colors(&mut map, scheme, eye_scheme, n_colors, true);
}
