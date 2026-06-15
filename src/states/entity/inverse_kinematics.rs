// use std::collections::BTreeMap;

// use macroquad::prelude::*;
// use serde::Deserialize;
// pub use crate::helper::direction::*;

// // =========================================================================
// //                REGISTRY DATA
// // =========================================================================
// /// Master modular container storing static configuration patterns
// #[derive(Debug, Clone, Deserialize)]
// pub struct KinematicsRegistryData {
//     /// Bounding box horizontal constraint (AABB Width)
//     pub aabb_width: f32,
//     /// Bounding box vertical constraint (AABB Height)
//     pub aabb_height: f32,
//     pub body: BodyRegistryData,
//     pub head: Option<HeadRegistryData>,
//     pub limbs: Vec<LimbRegistryData>,
//     pub poses: PoseListRegistryData
// }

// #[derive(Debug, Clone, Deserialize)]
// /// Modular Container for Body Data, hexagon shaped
// pub struct BodyRegistryData {
//     /// How hight up the center of the body is as a fraction of the AABB height
//     pub height_offset: f32,
//     /// How hight the shoulders are as a graction of height
//     pub shoulder_height: f32,
//     /// Bounding box horizontal constraint (AABB Width)
//     pub shoulder_width: f32,
//     pub neck_width: f32,
//     pub hip_width: f32,
//     pub height: f32,
//     pub weight: f32
//     // TODO: Automatically calculate connection point of limb
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct HeadRegistryData {
//     pub width: f32,
//     pub weight: f32
// }

// #[derive(Debug, Clone, Deserialize)]
// pub struct LimbRegistryData {
//     pub limb_type: LimbType,
//     pub name: String,
//     pub length: f32,
//     pub attach_height: f32,
//     pub attach_side: Direction,
//     pub texture_upper: String,
//     pub texture_lower: String,
//     pub texture_joint: String,
//     pub weight: f32
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
// pub enum LimbType {
//     Arm,
//     Leg,
//     Wing,
//     Tail,
//     Custom,
// }

// /// Only Stores IDs
// #[derive(Debug, Clone, Deserialize)]
// pub struct PoseListRegistryData {
//     pub standing: String,
//     pub walking: String,
//     pub prone: String,
//     pub additional: BTreeMap<String, String>
// }

// /// All positions are a as a fraction of AABB for compatibility reason, defaults to left
// #[derive(Debug, Clone, Deserialize)]
// pub struct PoseRegistryData {
//     pub body_offset: (f32, f32), // 0, 0 is normal, goes to 0.5 max
//     pub head: f32, // Rotation in radians
//     pub limbs: Vec<((f32, f32), Direction)>, // ((x, y), bias)
// } 
// impl PoseRegistryData {
//     /// Used for creating the right version of stances, only run on registry initialization
//     /// NO FORWARD FACING STANCES!
//     pub fn right(&self) -> PoseRegistryData {
//         PoseRegistryData {
//             body_offset: (-self.body_offset.0.clone(), self.body_offset.1.clone()),
//             head: -self.head.clone(),
//             limbs: self.limbs.clone().iter().map(
//                 |limb_data|
//                 ((-limb_data.0.0, limb_data.0.1), match limb_data.1 {
//                     Direction::Left => Direction::Right,
//                     Direction::Right => Direction::Left,
//                     Direction::Up => Direction::Up,
//                     _ => crate::global_panic!(direction limb_data.1)
//                 })
//             ).collect()
//         }
//     }
// }

// // =========================================================================
// //                REGISTRY DATA
// // =========================================================================
// /// Registry