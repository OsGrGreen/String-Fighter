use glam::Vec3;
use segment::Segment;

use crate::spline::Spline;

pub mod segment;

pub fn create_segment_list(n:usize, start:Vec3,len:f32) -> Vec<Segment>{
    let mut segments = Vec::new();
    segments.push(Segment::new(start,len,0.0,0.0));
    for _ in 0..n{
        segments.push(Segment::new_child(&segments[0], len, 0.0, 0.0));
    }
    return segments
}

pub fn segment_to_straight_spline(segments: Vec<Segment>) -> Spline{
    let mut points:Vec<Vec3> = segments.iter().map(|segment| segment.end).rev().collect();
    points.push(segments.first().expect("Array was empty").start);
    points = points.windows(2).flat_map(|w| 
        if let [a, b] = w {
                vec![*a,0.75*(*a)+(0.25)*(*b), 0.25*(*a)+(0.75)*(*b),*b]
            } else {
                vec![]
            })
        .rev().collect();
    let spline = Spline::new(points);
    spline
}