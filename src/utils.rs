use floem::kurbo::{Point, Rect};

pub struct Segment {
    pub p1: Point,
    pub p2: Point,
}

impl Segment {
    /// Returns the intersection point between the segment and the rectangle's boundary.
    /// If the segment starts inside the rectangle, returns the exit point; otherwise, the entry point.
    /// Uses the Liang-Barsky algorithm for parametric clipping.
    pub fn intersect_rect(&self, rect: &Rect) -> Option<Point> {
        let dx = self.p2.x - self.p1.x;
        let dy = self.p2.y - self.p1.y;
        let mut t_min = 0.0;
        let mut t_max = 1.0;

        // Updates t_min and t_max for a boundary defined by (p, q).
        // Returns false if the segment is parallel to the boundary and outside.
        let update = |p: f64, q: f64, t_min: &mut f64, t_max: &mut f64| -> bool {
            if p.abs() < std::f64::EPSILON {
                if q < 0.0 {
                    return false;
                }
            } else {
                let t = q / p;
                if p < 0.0 {
                    if t > *t_min {
                        *t_min = t;
                    }
                } else {
                    if t < *t_max {
                        *t_max = t;
                    }
                }
            }
            true
        };

        // Left boundary: x = rect.x0
        if !update(-dx, self.p1.x - rect.x0, &mut t_min, &mut t_max) {
            return None;
        }
        // Right boundary: x = rect.x1
        if !update(dx, rect.x1 - self.p1.x, &mut t_min, &mut t_max) {
            return None;
        }
        // Top boundary: y = rect.y0
        if !update(-dy, self.p1.y - rect.y0, &mut t_min, &mut t_max) {
            return None;
        }
        // Bottom boundary: y = rect.y1
        if !update(dy, rect.y1 - self.p1.y, &mut t_min, &mut t_max) {
            return None;
        }

        if t_min > t_max {
            return None;
        }

        // Determine which intersection to return:
        // if p1 is inside, return the exit point (t_max); otherwise, the entry point (t_min).
        let inside = self.p1.x >= rect.x0
            && self.p1.x <= rect.x1
            && self.p1.y >= rect.y0
            && self.p1.y <= rect.y1;
        let t = if inside { t_max } else { t_min };

        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(Point {
            x: self.p1.x + t * dx,
            y: self.p1.y + t * dy,
        })
    }
}

pub mod signal_serde {
    use crate::RwSignal;
    use floem::prelude::SignalGet;
    use serde::{Deserialize, Deserializer, Serialize, Serializer}; // Adjust the import to match your project

    // Serialize the inner value of the signal.
    pub fn serialize<S, T>(signal: &RwSignal<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize + Clone + 'static,
    {
        signal.get_untracked().serialize(serializer)
    }

    // Deserialize a value and wrap it in a new signal.
    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<RwSignal<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + 'static,
    {
        let value = T::deserialize(deserializer)?;
        Ok(RwSignal::new(value))
    }
}
