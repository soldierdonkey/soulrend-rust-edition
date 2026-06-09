use macroquad::texture::Texture2D;
use serde::Deserialize;
use std::mem::replace;
use crate::states::{ArmorSlot, Item, slot_to_index};

pub struct Inventory {
    pub hotbar: [Option<Item>; 9], // 9 total slots
    pub hotbar_selected: usize, // row
    pub inventory: [Option<Item>; 18], // 18 total slots
    pub armor: [Option<Item>; 4],
    pub mouse_slot: Option<Item> //What is currently stored in the mouse
}
impl Inventory {
    pub fn new() -> Self {
        Self {
            hotbar: Default::default(),
            hotbar_selected: 0,
            inventory: Default::default(),
            armor: Default::default(),
            mouse_slot: Default::default()
        }
    }
    pub fn get_selected_hotbar_item(&self) -> &Option<Item> {
        &self.hotbar[self.hotbar_selected]
    }
    pub fn read_slot(&self, slot: &SlotBinding) -> &Option<Item> {
        match slot {
            SlotBinding::Armor(armor_slot) => {
                &self.armor[slot_to_index(armor_slot)]
            }
            SlotBinding::Hotbar(index) => {
                &self.hotbar[*index]
            }
            SlotBinding::Inventory(index) => {
                &self.inventory[*index]
            }
            SlotBinding::Mouse => {
                &self.mouse_slot
            }
        }
    }
    pub fn get_slot(&mut self, slot: &SlotBinding) -> &mut Option<Item> {
        match slot {
            SlotBinding::Armor(armor_slot) => {
                &mut self.armor[slot_to_index(armor_slot)]
            }
            SlotBinding::Hotbar(index) => {
                &mut self.hotbar[*index]
            }
            SlotBinding::Inventory(index) => {
                &mut self.inventory[*index]
            }
            SlotBinding::Mouse => {
                &mut self.mouse_slot
            }
        }
    }
    pub fn set_slot(&mut self, slot: &SlotBinding, item: Option<Item>) {
        match slot {
            SlotBinding::Armor(armor_slot) => {
                self.armor[slot_to_index(armor_slot)] = item
            }
            SlotBinding::Hotbar(index) => {
                self.hotbar[*index] = item
            }
            SlotBinding::Inventory(index) => {
                self.inventory[*index] = item
            }
            SlotBinding::Mouse => {
                self.mouse_slot = item
            }
        }
    }
    pub fn swap_slots_safe(&mut self, slots: (&SlotBinding, &SlotBinding)) {
        // This is safer but slightly more complex because you must handle 
        // moving values out of the enum/array fields.
        // Although this is slower, this function will likely only be run a couple times per hour
        println!("{:#?}, {:#?}", self.read_slot(slots.0), self.read_slot(slots.1));
        let val_a = self.get_slot(slots.0).take();
        let val_b = self.get_slot(slots.1).take();
        
        self.set_slot(slots.0, val_b);
        self.set_slot(slots.1, val_a);
        println!("{:#?}, {:#?}", self.read_slot(slots.0), self.read_slot(slots.1));
    }
}


/// Identifiers mapping UI slots straight to actual live player equipment states
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum SlotBinding {
    Armor(ArmorSlot),
    Hotbar(usize),
    Inventory(usize),
    Mouse
}