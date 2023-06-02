use macroquad::prelude::*;

pub fn has_collide(rect_a: &Rect, rect_b: &Rect) -> Option<(Rect, Vec2)> {
    if let Some(intersection) = rect_a.intersect(*rect_b) {
        let to = rect_b.center() - rect_a.center();
        let to_signum = to.signum();

        return Some((intersection, to_signum));
    }

    return None;
}
