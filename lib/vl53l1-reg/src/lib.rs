//! A crate for low-level access to the registers on the VL53L1X.
//!
//! Provides:
//!
//! - A type for every "entry" in the register map.
//! - An `Entry` trait for abstracting over register entries.
//! - An `Index` type for a dynamic representation of entry indices into the register map.
//! - An `State` type for a dynamic representation of entry state.
//! - `write_*` and `read_*` functions for writing and reading to and from the VL53L1X's registers.
//! - A `structs` module with commonly grouped registers and generated methods for writing and
//!   reading these groups at once.
//!
//! The generated code takes quite a while to compile (~7 secs) so it has been split into this
//! separate crate and re-exported under the `reg` module in the main crate.

#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate bitfield;

mod map;
pub mod settings;
pub mod structs;

use embedded_hal::blocking::i2c;
pub use map::*;
pub use structs::Entries;

/// Implemented for all entries within the register map.
///
/// An entry represents a single value represented by 1, 2 or 4 8-bit registers.
pub trait Entry: Sized {
    /// The unique index indicating the location of the entry within the register map.
    const INDEX: Index;
    /// The array type representing the entry encoded in bytes ordered for I2C (MSB).
    type Array: AsMut<[u8]> + AsRef<[u8]> + Default;

    /// Encode self in an array, ready for transmission over I2C (to MSB).
    fn into_array(self) -> Self::Array;
    /// Decode self from an of bytes in the order they were received over I2C (from MSB).
    fn from_array(arr: Self::Array) -> Self;
    /// Access the `Index` via reference.
    fn index(&self) -> Index {
        Self::INDEX
    }
}

const INDEX_LEN: usize = 2;
const DATA_LEN: usize = 200;

/// The maximum amount of data that may be written to `write_slice` at once.
pub const MAX_SLICE_LEN: usize = DATA_LEN - INDEX_LEN;
/// The slave address of the VL53L1X. Note that `embedded-hal` I2C API adds the w/r bit for us.
pub const SLAVE_ADDR: u8 = 0x52 >> 1;

/// Write given slice of data to the device at the given index.
///
/// **Panic**s if `slice.len()` is greater than `MAX_SLICE_LEN`.
pub fn write_slice<I>(i2c: &mut I, index: Index, slice: &[u8]) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    assert!(slice.len() < MAX_SLICE_LEN);
    let mut data = [0u8; DATA_LEN];

    // Get actual data length.
    let data_len = INDEX_LEN + slice.len();
    let data = &mut data[..data_len];

    // Write the index.
    let [ix_a, ix_b]: [u8; 2] = index.into();
    data[0] = ix_a;
    data[1] = ix_b;

    // Write the slice.
    for (d, s) in data[INDEX_LEN..].iter_mut().zip(slice) {
        *d = *s;
    }

    // Write the data to the slave.
    i2c.write(SLAVE_ADDR, &data)
}

/// Read the value at the given index into the given slice.
///
/// The length of the given slice must represent the length of the expected amount of data to be
/// read.
pub fn read_slice<I>(i2c: &mut I, index: Index, slice: &mut [u8]) -> Result<(), I::Error>
where
    I: i2c::WriteRead,
{
    let arr: [u8; 2] = index.into();
    i2c.write_read(SLAVE_ADDR, &arr, slice)
}

/// Shorthand for writing a slice with a single byte.
pub fn write_byte<I>(i2c: &mut I, index: Index, byte: u8) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    write_slice(i2c, index, &[byte])
}

/// Shorthand for reading a single byte from the register at the given index.
pub fn read_byte<I>(i2c: &mut I, index: Index) -> Result<u8, I::Error>
where
    I: i2c::WriteRead,
{
    let mut b = [0u8];
    read_slice(i2c, index, &mut b)?;
    let [b] = b;
    Ok(b)
}

/// Shorthand for writing a slice with a single word.
pub fn write_word<I>(i2c: &mut I, index: Index, word: u16) -> Result<(), I::Error>
where
    I: i2c::Write,
{
    let [a, b] = word.to_be_bytes();
    write_slice(i2c, index, &[a, b])
}

/// Shorthand for reading two consecutive bytes from the given index.
pub fn read_word<I>(i2c: &mut I, index: Index) -> Result<u16, I::Error>
where
    I: i2c::WriteRead,
{
    let mut bs = [0u8; 2];
    read_slice(i2c, index, &mut bs)?;
    Ok(u16::from_be_bytes(bs))
}

/// Read the the given entry.
pub fn write_entry<I, E>(i2c: &mut I, entry: E) -> Result<(), I::Error>
where
    I: i2c::Write,
    E: Entry,
{
    let arr = entry.into_array();
    write_slice(i2c, E::INDEX, arr.as_ref())
}

/// Read the value for a single entry.
pub fn read_entry<I, E>(i2c: &mut I) -> Result<E, I::Error>
where
    I: i2c::WriteRead,
    E: Entry,
{
    let mut arr: E::Array = Default::default();
    read_slice(i2c, E::INDEX, arr.as_mut())?;
    Ok(E::from_array(arr))
}
