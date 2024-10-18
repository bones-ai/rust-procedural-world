#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_gen_sprite() {
    let mut gd = get_sprite(1234, &Size { x: 45, y: 45 }, 12, true);

    gd._ready();
    gd.draw_all();

    let color = gd.get_primary_color();
    println!("{:?}", color);

    let faction = gd.get_faction();
    println!("{:?}", faction);

    gd.write_html_file("./sprite.html");
}
