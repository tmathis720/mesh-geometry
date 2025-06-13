use crate::{Float, Point3};

/// Axis-aligned bounding box in 3D.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb<T: Float> {
    /// Minimum corner of the bounding box
    pub min: Point3<T>,
    /// Maximum corner of the bounding box
    pub max: Point3<T>,
}

impl<T: Float> Aabb<T> {
    /// Empty box: min=+∞, max=-∞
    pub fn empty() -> Self {
        let inf = T::infinity();
        let neg = T::neg_infinity();
        Aabb {
            min: Point3::new(inf, inf, inf),
            max: Point3::new(neg, neg, neg),
        }
    }

    /// Build from a list of points
    pub fn from_points(pts: &[Point3<T>]) -> Self {
        let mut bb: Aabb<T> = Aabb::empty();
        for &p in pts {
            bb.min.x = bb.min.x.min(p.x);
            bb.min.y = bb.min.y.min(p.y);
            bb.min.z = bb.min.z.min(p.z);
            bb.max.x = bb.max.x.max(p.x);
            bb.max.y = bb.max.y.max(p.y);
            bb.max.z = bb.max.z.max(p.z);
        }
        bb
    }

    /// Extend this box to include `p`
    pub fn grow(&mut self, p: Point3<T>) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    /// Test intersection with another AABB
    pub fn intersects(&self, other: &Aabb<T>) -> bool {
        !(self.max.x < other.min.x ||
          self.min.x > other.max.x ||
          self.max.y < other.min.y ||
          self.min.y > other.max.y ||
          self.max.z < other.min.z ||
          self.min.z > other.max.z)
    }

    /// Test if `p` is inside (inclusive)
    pub fn contains(&self, p: Point3<T>) -> bool {
        p.x >= self.min.x && p.x <= self.max.x &&
        p.y >= self.min.y && p.y <= self.max.y &&
        p.z >= self.min.z && p.z <= self.max.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point3;

    #[test]
    fn aabb_from_and_contains() {
        let pts = [
            Point3::new(-1.0,2.0,0.0),
            Point3::new(3.0,-2.0,5.0),
        ];
        let bb = Aabb::from_points(&pts);
        assert!(bb.contains(Point3::new(0.0,0.0,2.5)));
        assert!(!bb.contains(Point3::new(-2.0,0.0,0.0)));
    }

    #[test]
    fn aabb_intersect() {
        let b1 = Aabb {
            min: Point3::new(0.0,0.0,0.0),
            max: Point3::new(1.0,1.0,1.0),
        };
        let b2 = Aabb {
            min: Point3::new(0.5,0.5,0.5),
            max: Point3::new(2.0,2.0,2.0),
        };
        assert!(b1.intersects(&b2));
        let b3 = Aabb {
            min: Point3::new(2.0,2.0,2.0),
            max: Point3::new(3.0,3.0,3.0),
        };
        assert!(!b1.intersects(&b3));
    }
}
