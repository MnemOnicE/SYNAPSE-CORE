use crate::horizon::{Foldable, HolographicHeader};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LidarPoint {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LidarPointCloud {
    pub points: Vec<LidarPoint>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
    pub point_count: usize,
}

impl Foldable for LidarPointCloud {
    type FoldedPayload = BoundingBox;

    fn fold(&self) -> (Self::FoldedPayload, f32) {
        if self.points.is_empty() {
            return (
                BoundingBox {
                    min_x: 0.0,
                    max_x: 0.0,
                    min_y: 0.0,
                    max_y: 0.0,
                    min_z: 0.0,
                    max_z: 0.0,
                    point_count: 0,
                },
                1.0, // 100% entropy loss for empty set
            );
        }

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;
        let mut min_z = f32::MAX;
        let mut max_z = f32::MIN;

        let mut valid_points = 0;
        for p in &self.points {
            if !p.x.is_finite() || !p.y.is_finite() || !p.z.is_finite() {
                continue;
            }
            valid_points += 1;
            if p.x < min_x {
                min_x = p.x;
            }
            if p.x > max_x {
                max_x = p.x;
            }
            if p.y < min_y {
                min_y = p.y;
            }
            if p.y > max_y {
                max_y = p.y;
            }
            if p.z < min_z {
                min_z = p.z;
            }
            if p.z > max_z {
                max_z = p.z;
            }
        }

        let bbox = BoundingBox {
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
            point_count: valid_points,
        };

        // Entropy loss is a rough heuristic: we lose individual point precision.
        let entropy_loss = 0.95; // 95% data loss, keeping only bounds.

        (bbox, entropy_loss)
    }

    fn unfold(_header: &HolographicHeader<Self::FoldedPayload>) -> Self {
        // In a true asymmetric recovery, if we only have the header, we can't
        // magically recreate the points. We either fetch them from the WAL
        // (handled elsewhere), or if we are forced to mock it from the header:
        // We'll return an empty cloud here as the 'lossy' unfold, acknowledging
        // the original points must be retrieved from bulk storage if needed.
        LidarPointCloud {
            points: Vec::new(), // Signifying data loss if read directly from header
        }
    }
}
