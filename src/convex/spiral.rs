use bevy::prelude::*;

pub fn algo(points: Vec<Vec2>) -> Vec<Vec2> {
    // Find the left most point
    let start = points
        .iter()
        .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
        .map(|e| *e)
        .unwrap_or(Vec2::ZERO);
    let mut spiral: Vec<_> = vec![start];
    let mut rem: Vec<_> = points.clone();
    loop {
        let last = *spiral.last().unwrap();
        rem = rem.into_iter().filter(|v| *v != last).collect();
        if rem.len() == 0 {
            break;
        }
        let next = rem
            .clone()
            .into_iter()
            .reduce(|all_on_right, v| {
                let cross = ((all_on_right - last).extend(0.0)).cross((v - last).extend(0.0));
                if cross.z > 0. {
                    v
                } else {
                    all_on_right
                }
            })
            .map(|e| e)
            .unwrap_or(Vec2::ZERO);
        spiral.push(next);
    }
    return spiral;
}
