#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_gen_sprite() {
    let mut gd = _get_group_drawer(false);
    gd._ready();
    gd.draw_all();
    gd.write_html_file("./sprite.html");
}
