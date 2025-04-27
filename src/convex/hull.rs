use bevy::prelude::*;

pub fn algo(points: Vec<Vec2>) -> Vec<Vec2> {
    // Find the left most point
    let start = points
        .iter()
        .reduce(|left_most, v| if v.x < left_most.x { v } else { left_most })
        .map(|e| *e)
        .unwrap_or_default();
    let finish = points
        .iter()
        .filter(|v| **v != start)
        .reduce(|all_on_left, v| {
            let cross = (*all_on_left - start)
                .extend(0.0)
                .cross((*v - start).extend(0.0));
            if cross.z < 0. {
                v
            } else {
                all_on_left
            }
        })
        .map(|e| *e)
        .unwrap_or(Vec2::ZERO);
    let mut hull: Vec<_> = vec![start];
    let mut rem: Vec<_> = points.clone();
    loop {
        let last = *hull.last().unwrap();
        rem = rem.into_iter().filter(|v| *v != last).collect();
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
        hull.push(next);
        if next == finish {
            hull.push(start);
            break;
        }
    }
    return hull;
}
