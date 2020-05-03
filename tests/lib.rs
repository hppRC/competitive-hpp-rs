use competitive_hpp::prelude::*;

#[test]
fn constants_test() {
    let dire = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    let dire8 = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];

    let alphabets = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    assert_eq!(dire, DIRE);
    assert_eq!(dire8, DIRE8);
    assert_eq!(alphabets, ALPHABETS);
}
