/*
 * src/inputs.rs
 * tasinput2 - Plugin for creating TAS inputs
 *
 * This file is part of tasinput2.
 *
 * tasinput2 is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * tasinput2 is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with tasinput2.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::convert::{Into, TryInto};

/// Directional buttons
pub struct Directional {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl Directional {
    /// Instantiate a new directional
    pub fn new(up: bool, down: bool, left: bool, right: bool) -> Directional {
        Directional {
            up,
            down,
            left,
            right,
        }
    }
}

/// Inputs that can be retrieved from a controller object.
pub struct Inputs {
    pub a: bool,
    pub b: bool,
    pub z: bool,
    pub l: bool,
    pub r: bool,
    pub c: Directional,
    pub d: Directional,
    pub start: bool,
    pub x: i8,
    pub y: i8,
}

// helper function to get bits and bytes of u32
fn get_bit(value: &u32, bit_index: u8) -> bool {
    ((value >> (bit_index as u32)) & 0x01) == 0x01
}

fn get_byte(value: &u32, byte_index: u8) -> u8 {
    // TODO: the right way of doing this, because this is not it
    let modified_value = value >> ((byte_index * 8) as u32);
    (modified_value & 0xFF).try_into().unwrap_or(0)
}

fn set_bit(value: &mut u32, bit_index: u8, bit_value: &bool) {
    let mask = 0x01 << bit_index;
    match bit_value {
        true => *value |= mask,
        false => *value &= mask, // TODO: bitwise not
    };
}

fn set_byte(value: &mut u32, byte_index: u8, byte_value: &u8) {
    let shift = (byte_index * 8) as u32;
    let mask: u32 = 0xFF << shift; // TODO: bitwise not
    let byte_value_converted = *byte_value as u32;
    *value = (byte_value_converted << shift) | (*value & mask);
}

// test these functions
#[test]
fn get_bit_test() {
    let val = 0b10101100;
    assert_eq!(get_bit(&val, 0), false);
    assert_eq!(get_bit(&val, 1), false);
    assert_eq!(get_bit(&val, 2), true);
    assert_eq!(get_bit(&val, 3), true);
    assert_eq!(get_bit(&val, 4), false);
    assert_eq!(get_bit(&val, 5), true);
    assert_eq!(get_bit(&val, 6), false);
    assert_eq!(get_bit(&val, 7), true);
}

#[test]
fn get_byte_test() {
    let val = 0b10101100010101011111000010101010;
    assert_eq!(get_byte(&val, 0), 0b10101010);
    assert_eq!(get_byte(&val, 1), 0b11110000);
    assert_eq!(get_byte(&val, 2), 0b01010101);
    assert_eq!(get_byte(&val, 3), 0b10101100);
}

#[test]
fn set_bit_test() {
    let mut val = 0b10101100;
    set_bit(&mut val, 1, &true);
    assert_eq!(val, 0b10101110);
    set_bit(&mut val, 7, &false);
    assert_eq!(val, 0b00101110);
}

#[test]
fn set_byte_test() {
    let mut val = 0b10101100010101011111000010101010;
    set_byte(&mut val, 0, &0b11111111);
    assert_eq!(val, 0b10101100010101011111000011111111);
}

impl Inputs {
    /// Convert this input structure to its equivalent value.
    pub fn to_value(&self) -> u32 {
        let mut value: u32 = 0;
        set_bit(&mut value, 0, &self.d.right);
        set_bit(&mut value, 1, &self.d.left);
        set_bit(&mut value, 2, &self.d.down);
        set_bit(&mut value, 3, &self.d.up);
        set_bit(&mut value, 4, &self.start);
        set_bit(&mut value, 5, &self.z);
        set_bit(&mut value, 6, &self.b);
        set_bit(&mut value, 7, &self.a);
        set_bit(&mut value, 8, &self.c.right);
        set_bit(&mut value, 9, &self.c.left);
        set_bit(&mut value, 10, &self.c.down);
        set_bit(&mut value, 11, &self.c.up);
        set_bit(&mut value, 12, &self.r);
        set_bit(&mut value, 13, &self.l);
        set_byte(&mut value, 2, &self.x);
        set_byte(&mut value, 3, &self.y);
        value
    }

    /// Initialize a new set of inputs
    pub fn with_directionals(
        x: i8,
        y: i8,
        a: bool,
        b: bool,
        z: bool,
        l: bool,
        r: bool,
        start: bool,
        c: Directional,
        d: Directional,
    ) -> Self {
        Self {
            a,
            b,
            z,
            l,
            r,
            c,
            d,
            start,
            x,
            y,
        }
    }

    /// Initialize with a new set of inputs, without directionals
    pub fn new(
        x: i8,
        y: i8,
        a: bool,
        b: bool,
        z: bool,
        l: bool,
        r: bool,
        start: bool,
        c_up: bool,
        c_down: bool,
        c_left: bool,
        c_right: bool,
        d_up: bool,
        d_down: bool,
        d_left: bool,
        d_right: bool,
    ) -> Self {
        Self::with_directionals(
            x,
            y,
            a,
            b,
            z,
            l,
            r,
            start,
            Directional::new(c_up, c_down, c_left, c_right),
            Directional::new(d_up, d_down, d_left, d_right),
        )
    }

    /// Initialize inputs from a value
    pub fn from_value(value: &u32) -> Self {
        let d = Directional::new(
            get_bit(value, 3),
            get_bit(value, 2),
            get_bit(value, 1),
            get_bit(value, 0),
        );
        let start = get_bit(value, 4);
        let z = get_bit(value, 5);
        let b = get_bit(value, 6);
        let a = get_bit(value, 7);
        let c = Directional::new(
            get_bit(value, 11),
            get_bit(value, 10),
            get_bit(value, 9),
            get_bit(value, 8),
        );
        let r = get_bit(value, 12);
        let l = get_bit(value, 13);
        let x = get_byte(value, 2);
        let y = get_byte(value, 3);
        Self::with_directionals(x, y, a, b, z, l, r, start, c, d)
    }
}