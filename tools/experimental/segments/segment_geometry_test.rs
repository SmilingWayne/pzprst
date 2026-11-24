use pzprst::{Board, Transform};

#[test]
fn segment_geometry_and_clear_snapshot() {
    let mut b = Board::new("slither", 3, 2).unwrap();
    b.segments.push(pzprst::Segment {
        start: (0, 0),
        end: (6, 4),
        origin: Some((2, 2)),
    });
    assert!(b.has_snapshot());
    let rotated = b.transform(Transform::Rot90);
    assert_eq!(rotated.segments[0].start, (4, 0));
    assert_eq!(rotated.segments[0].end, (0, 6));
    assert_eq!(rotated.segments[0].origin, Some((2, 2)));
    assert_eq!(rotated.transform(Transform::Rot270), b);
    b.clear_snapshot();
    assert!(!b.has_snapshot());
}

