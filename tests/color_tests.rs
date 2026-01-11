use crusty_cards::Color;
use std::collections::HashSet;

#[test]
fn test_color_ordering() {
    assert!(Color::Red < Color::Black);
}

#[test]
fn test_color_hash_in_hashset() {
    let mut set: HashSet<Color> = HashSet::new();
    assert!(set.insert(Color::Red));
    assert!(set.insert(Color::Black));
    assert_eq!(set.len(), 2);

    // Duplicates
    assert!(!set.insert(Color::Red));
    assert!(!set.insert(Color::Black));
    assert_eq!(set.len(), 2);
}

#[test]
fn test_color_display() {
    assert_eq!(format!("{}", Color::Red), "R");
    assert_eq!(format!("{}", Color::Black), "B");
}

#[test]
fn test_color_equality() {
    assert_eq!(Color::Red, Color::Red);
    assert_eq!(Color::Black, Color::Black);
    assert_ne!(Color::Red, Color::Black);
}

#[test]
fn test_color_clone_copy() {
    let color = Color::Red;
    let cloned = color.clone();
    let copied = color;
    assert_eq!(color, cloned);
    assert_eq!(color, copied);
}
