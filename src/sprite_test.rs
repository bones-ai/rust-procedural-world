#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_gen_sprite() {
    let sprite = Sprite::new(1234);
    sprite.write_html_file("./sprite.html");
}
