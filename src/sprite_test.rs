#[cfg(test)]
use crate::sprite::*;

#[test]
fn test_sprites() {
    let sprite = get_sprite(1234, Size { x: 12, y: 160 }, 12381283, false);
    println!("groups.len(): {}", sprite.groups.len());
    println!("negative_groups.len(): {}", sprite.negative_groups.len());
    // println!("{:?}", sprite.negative_groups);

    // for group in sprite.negative_groups {
    //     println!("group.arr.len(): {}", group.arr.len());
    //     println!("{:?}", group.arr);
    // }
}
