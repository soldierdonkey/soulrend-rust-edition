use macroquad::math::Vec2;
use serde::Deserialize;

use crate::states::{HeldWeapon, WeaponMoveset, WeaponType};

//===========================================================================================================
//                                                                                                           
//  #####    #####   ####    ##   ####  ######  #####    ##    ##        ####      ###    ######    ###    
//  ##  ##   ##     ##       ##  ##       ##    ##  ##    ##  ##         ##  ##   ## ##     ##     ## ##   
//  #####    #####  ##  ###  ##   ###     ##    #####      ####          ##  ##  ##   ##    ##    ##   ##  
//  ##  ##   ##     ##   ##  ##     ##    ##    ##  ##      ##           ##  ##  #######    ##    #######  
//  ##   ##  #####   ####    ##  ####     ##    ##   ##     ##           ####    ##   ##    ##    ##   ##  
//                                                                                                           
//===========================================================================================================


#[derive(Debug, Clone, Deserialize)]
pub struct KinematicsRegistryData {
    pub width: f32,
    pub height: f32,
    pub torso: String, //TorsoRegistryData
    pub leg_set: String, //LegSetRegistryData,
    pub arm_pairs: Vec<String>, //ArmPairRegistryData,
    pub other_limbs: Vec<String>, //LimbRegistryData
}

#[derive(Debug, Clone, Deserialize)]
pub struct TorsoRegistryData {
    width: f32, // Fraction of total bounding box
    height: f32, // Fraction of total bounding box
    vertical_offset: f32, // Fraction of total bounding box: 0.5 means centered around very top, -0.5 means centered around very bottom
    sprite: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct LegSetRegistryData {
    legs: Vec<String>, //LimbRegistryData
    walk_speed: f32, // how many seconds per one leg switch.
    friction: Vec<f32> // normalized friction of leg, used for leg-drag walk animations.
}

#[derive(Debug, Clone, Deserialize)]
pub struct ArmPairRegistryData {
    arms: (String, String), //(LimbRegistryData, LimbRegistryData),
    weapon: HeldWeapon,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LimbRegistryData {
    connection_point: (f32, f32), // connection as a fraction of total hitbox (0.5 0.5 is dead center)
    upper: String, //SegmentRegistryData, // closest to body,
    lower: String, //SegmentRegistryData, // forearm/shin
    connection: String, //JointRegistryData, //hand/foot
    bend: String, //JointRegistryData //elbow/knee
    // Joints drawn OVER segments!
}

#[derive(Debug, Clone, Deserialize)]
pub struct SegmentRegistryData {
    length: f32,
    width: f32,
    weight: f32,
    sprite: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JointRegistryData {
    length: f32,
    width: f32,
    weight: f32,
    sprite: String,
    balance: JointBalance
}

#[derive(Debug, Clone, Deserialize)]
pub enum JointBalance {
    Flat, // Flat on the ground, just like a foot
    Midpoint, // Midpoint angle between the segment above and the segment below.
    Controlled // Ignore angle balancing, used for hands because those lock to held weapons.
}

//====================================================================================================================
//                                                                                                                    
//  ##  ##     ##   ####  ######    ###    ##     ##   ####  #####  ####          ####      ###    ######    ###    
//  ##  ####   ##  ##       ##     ## ##   ####   ##  ##     ##     ##  ##        ##  ##   ## ##     ##     ## ##   
//  ##  ##  ## ##   ###     ##    ##   ##  ##  ## ##  ##     #####  ##  ##        ##  ##  ##   ##    ##    ##   ##  
//  ##  ##    ###     ##    ##    #######  ##    ###  ##     ##     ##  ##        ##  ##  #######    ##    #######  
//  ##  ##     ##  ####     ##    ##   ##  ##     ##   ####  #####  ####          ####    ##   ##    ##    ##   ##  
//                                                                                                                    
//====================================================================================================================


#[derive(Debug, Clone, Deserialize, Default)]
pub struct KinematicsData {
    pub leg_set: LegSet,
    pub arm_pairs: Vec<ArmPair>,
    pub other_limbs: Vec<Limb>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LegSet {
    legs: Vec<Limb>,
    current_leg: usize,
    time_until_next_leg: f32
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ArmPair {
    arms: (Limb, Limb),
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Limb {
    upper: Segment, // closest to body,
    lower: Segment, // forearm/shin
    connection: Joint, //hand/foot
    bend: Joint //elbow/knee
    // Joints drawn OVER segments!
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Segment {
    registry_data: String,
    angle: f32
    // no position, only angle. Position can be calculated from just angle, connection position, and registry data.
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Joint {
    registry_data: String,
    angle: f32
    // no position, only angle. Position can be calculated from just angle, connection position, and registry data.
}