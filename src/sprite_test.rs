#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_sprites() {
    let mut gd = _get_group_drawer(false);

    gd._ready();

    println!("gd.len(): {}", gd.children.len());
    for i in 0..gd.children.len() {
        let c = &gd.children[i];
        println!("gd.children[{}].cells.len(): {}", i, c.cells.len());
    }

    gd.draw_all();

    println!("gd.len(): {}", gd.children.len());
    for i in 0..gd.children.len() {
        let c = &gd.children[i];
        println!("gd.children[{}].cells.len(): {}", i, c.cells.len());
    }
}
