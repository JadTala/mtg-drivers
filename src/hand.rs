use std::{f32::consts::PI};

use nalgebra::{Point3, distance};
use serde::{Serialize, Deserialize};
use strum_macros::EnumIter;

const FINGERTIP_HITBOX_RADIUS: f32 = 0.01;
const FINGER_BENDING_MIN_TRESHOLD: f32 = -60.0 / 180.0 * PI;
const FINGER_BENDING_MAX_TRESHOLD: f32 = 60.0 / 180.0 * PI;

#[derive(Default, Clone)]
pub struct Hand {
    model: HandModel,
    gestures: HandGestures,
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum HandPart
{
    Palm,
    Thumb,
    Index,
    Middle,
    Ring,
    Little
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
pub struct HandModel
{
    // Cartesian coordinates
    palm_coords: [[f32; 3]; 1],
    thumb_coords: [[f32; 3]; 3],
    index_coords: [[f32; 3]; 4],
    middle_coords: [[f32; 3]; 4],
    ring_coords: [[f32; 3]; 4],
    little_coords: [[f32; 3]; 4],

    // Euler angles
    palm_euler: [f32; 3],
    thumb_euler: [f32; 3],
    index_euler: [f32; 3],
    middle_euler: [f32; 3],
    ring_euler: [f32; 3],
    little_euler: [f32; 3],
}

impl HandModel {
    pub fn from_raw_data(raw_data: Vec<u8>) -> HandModel {
        let (head, body, _tail) = unsafe { 
            raw_data.align_to::<HandModel>()
        };
        assert!(head.is_empty(), "Data was not aligned");
        body[0]
    }
}

#[derive(Debug, Default, Clone)]
pub struct HandGestures
{
    finger_touching_thumb: Option<HandPart>,
    bent_fingers: Vec<HandPart>,
    acceleration : [f32; 3]
}

impl Hand
{
    pub fn get_palm_coords(&self) -> [[f32; 3]; 1]
    {
        self.model.palm_coords
    }

    pub fn get_thumb_coords(&self) -> [[f32; 3]; 3]
    {
        self.model.thumb_coords
    }

    pub fn get_index_coords(&self) -> [[f32; 3]; 4]
    {
        self.model.index_coords
    }

    pub fn get_middle_coords(&self) -> [[f32; 3]; 4]
    {
        self.model.middle_coords
    }

    pub fn get_ring_coords(&self) -> [[f32; 3]; 4]
    {
        self.model.ring_coords
    }

    pub fn get_little_coords(&self) -> [[f32; 3]; 4]
    {
        self.model.little_coords
    }

    pub fn get_euler(&self, part: HandPart) -> [f32; 3]
    {
        match part
        {
            HandPart::Palm => self.model.palm_euler,
            HandPart::Thumb => self.model.thumb_euler,
            HandPart::Index => self.model.index_euler,
            HandPart::Middle => self.model.middle_euler,
            HandPart::Ring => self.model.ring_euler,
            HandPart::Little => self.model.little_euler,
        }
    }

    pub fn get_bent_fingers(&self) -> Vec<HandPart>
    {
        self.gestures.bent_fingers.clone()
    }

    pub fn get_finger_touching_thumb(&self) -> Option<HandPart>
    {
        self.gestures.finger_touching_thumb.clone()
    }

    pub fn get_acceleration(&self) -> [f32; 3]
    {
        self.gestures.acceleration.clone()
    }

    pub fn update_model(&mut self, new_model: HandModel)
    {
        self.model = new_model;
        self.update_gestures();
    }

    pub fn update_gestures(&mut self)
    {
        // Update bent fingers gesture
        self.gestures.bent_fingers.clear();
        if !(FINGER_BENDING_MIN_TRESHOLD <= self.model.thumb_euler[1] && self.model.thumb_euler[1] <= FINGER_BENDING_MAX_TRESHOLD)
        {
            self.gestures.bent_fingers.push(HandPart::Thumb);
        }
        if !(FINGER_BENDING_MIN_TRESHOLD <= self.model.index_euler[1] && self.model.index_euler[1] <= FINGER_BENDING_MAX_TRESHOLD)
        {
            self.gestures.bent_fingers.push(HandPart::Index);
        }
        if !(FINGER_BENDING_MIN_TRESHOLD <= self.model.middle_euler[1] && self.model.middle_euler[1] <= FINGER_BENDING_MAX_TRESHOLD)
        {
            self.gestures.bent_fingers.push(HandPart::Middle);
        }
        if !(FINGER_BENDING_MIN_TRESHOLD <= self.model.ring_euler[1] && self.model.ring_euler[1] <= FINGER_BENDING_MAX_TRESHOLD)
        {
            self.gestures.bent_fingers.push(HandPart::Ring);
        }
        if !(FINGER_BENDING_MIN_TRESHOLD <= self.model.little_euler[1] && self.model.little_euler[1] <= FINGER_BENDING_MAX_TRESHOLD)
        {
            self.gestures.bent_fingers.push(HandPart::Little);
        }

        self.gestures.finger_touching_thumb = None;
        // Update thumb touches finger gesture
        if self.gestures.bent_fingers.contains(&HandPart::Thumb)
        {
            if self.gestures.bent_fingers.contains(&HandPart::Index)
            {
                self.gestures.finger_touching_thumb = Some(HandPart::Index);
            }
            else if self.gestures.bent_fingers.contains(&HandPart::Middle)
            {
                self.gestures.finger_touching_thumb = Some(HandPart::Middle);
            }
            else if self.gestures.bent_fingers.contains(&HandPart::Ring)
            {
                self.gestures.finger_touching_thumb = Some(HandPart::Ring);
            }
            else if self.gestures.bent_fingers.contains(&HandPart::Little)
            {
                self.gestures.finger_touching_thumb = Some(HandPart::Little);
            }
        }
    }
}