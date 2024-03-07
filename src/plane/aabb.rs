use glam::Vec2;

use super::{segment::Segment, Point};

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub mins: Point,
    pub maxs: Point,
}

impl Aabb {
    #[inline]
    pub fn new(mins: Point, maxs: Point) -> Self {
        Self { mins, maxs }
    }

    #[inline]
    pub fn new_invalid() -> Self {
        Self::new(
            Point::new(f32::MIN, f32::MIN),
            Point::new(f32::MAX, f32::MAX),
        )
    }

    #[inline]
    pub fn intersects(&self, other: &Aabb) -> bool {
        self.mins.x <= other.maxs.x
            && self.maxs.x > other.mins.x
            && self.mins.y <= other.maxs.y
            && self.maxs.y > other.mins.y
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mins = Point::new(self.mins.x.max(other.mins.x), self.mins.y.max(other.mins.y));
        let maxs = Point::new(self.maxs.x.min(other.maxs.x), self.maxs.y.min(other.maxs.y));
        let result = Self { mins, maxs };

        for i in 0..2 {
            if result.mins[i] > result.maxs[i] {
                return None;
            }
        }
        Some(result)
    }

    pub fn clip_segment(&self, pa: Point, pb: Point) -> Option<Segment> {
        let ab = pb - pa;
        clip_aabb_line(self, &pa, &ab)
            .map(|clip| Segment::new(pa + ab * (clip.0).0.max(0.0), pa + ab * (clip.1).0.min(1.0)))
    }

    pub fn center(&self) -> Point {
        (self.mins + self.maxs) * 0.5
    }

    pub fn half_extents(&self) -> Vec2 {
        (self.maxs - self.mins) * 0.5
    }

    #[inline]
    pub fn extents(&self) -> Vec2 {
        self.maxs - self.mins
    }

    #[inline]
    pub fn contains(&self, other: &Aabb) -> bool {
        self.mins.x <= other.mins.x
            && self.mins.y <= other.mins.y
            && self.maxs.x >= other.maxs.x
            && self.maxs.y >= other.maxs.y
    }
}

pub fn clip_aabb_line(
    aabb: &Aabb,
    origin: &Point,
    dir: &Vec2,
) -> Option<((f32, Vec2, isize), (f32, Vec2, isize))> {
    let mut tmax = f32::MAX;
    let mut tmin = -tmax;
    let mut near_side = 0;
    let mut far_side = 0;
    let mut near_diag = false;
    let mut far_diag = false;

    for i in 0usize..2 {
        if (dir[i] - 0.0).abs() < 0.01 {
            if origin[i] < aabb.mins[i] || origin[i] > aabb.maxs[i] {
                return None;
            }
        } else {
            let denom = 1.0 / dir[i];
            let flip_sides;
            let mut inter_with_near_halfspace = (aabb.mins[i] - origin[i]) * denom;
            let mut inter_with_far_halfspace = (aabb.maxs[i] - origin[i]) * denom;

            if inter_with_near_halfspace > inter_with_far_halfspace {
                flip_sides = true;
                std::mem::swap(
                    &mut inter_with_near_halfspace,
                    &mut inter_with_far_halfspace,
                )
            } else {
                flip_sides = false;
            }

            if inter_with_near_halfspace > tmin {
                tmin = inter_with_near_halfspace;
                near_side = if flip_sides {
                    -(i as isize + 1)
                } else {
                    i as isize + 1
                };
                near_diag = false;
            } else if inter_with_near_halfspace == tmin {
                near_diag = true;
            }

            if inter_with_far_halfspace < tmax {
                tmax = inter_with_far_halfspace;
                far_side = if !flip_sides {
                    -(i as isize + 1)
                } else {
                    i as isize + 1
                };
                far_diag = false;
            } else if inter_with_far_halfspace == tmax {
                far_diag = true;
            }

            if tmax < 0.0 || tmin > tmax {
                return None;
            }
        }
    }

    let near = if near_diag {
        (tmin, -dir.normalize(), near_side)
    } else {
        let mut normal = Vec2::new(0.0, 0.0);

        if near_side < 0 {
            normal[(-near_side - 1) as usize] = 1.0;
        } else {
            normal[(near_side - 1) as usize] = -1.0;
        }

        (tmin, normal, near_side)
    };

    let far = if far_diag {
        (tmax, -dir.normalize(), far_side)
    } else {
        let mut normal = Vec2::new(0.0, 0.0);

        if far_side < 0 {
            normal[(-far_side - 1) as usize] = -1.0;
        } else {
            normal[(far_side - 1) as usize] = 1.0;
        }

        (tmax, normal, far_side)
    };

    Some((near, far))
}
