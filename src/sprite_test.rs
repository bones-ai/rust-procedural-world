#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_gen_rand_sprite() {
    let sprite = Sprite::new_from_unix_seed();
    sprite.write_html_file("./sprite.html");
}

#[test]
fn test_gen_same_sprite() {
    let sprite = Sprite::new(1234);
    sprite.write_html_file("./sprite.html");
}
