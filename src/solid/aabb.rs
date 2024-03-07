use glam::Vec3;

use super::Point;

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
            Point::new(f32::MIN, f32::MIN, f32::MIN),
            Point::new(f32::MAX, f32::MAX, f32::MAX),
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
        let mins = Point::new(
            self.mins.x.max(other.mins.x),
            self.mins.y.max(other.mins.y),
            self.mins.z.max(other.mins.z),
        );
        let maxs = Point::new(
            self.maxs.x.min(other.maxs.x),
            self.maxs.y.min(other.maxs.y),
            self.maxs.z.min(other.maxs.z),
        );
        let result = Self { mins, maxs };

        for i in 0..3 {
            if result.mins[i] > result.maxs[i] {
                return None;
            }
        }
        Some(result)
    }

    pub fn center(&self) -> Point {
        (self.mins + self.maxs) * 0.5
    }

    pub fn half_extents(&self) -> Vec3 {
        (self.maxs - self.mins) * 0.5
    }

    #[inline]
    pub fn extents(&self) -> Vec3 {
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
