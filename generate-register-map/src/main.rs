//! First argument is path to `vl53l1_register_map.h`, second is output `reg.rs` path.
//!
//! ## Steps
//!
//! 1. Split string by "#define VL53L1_".

use std::fmt::Write;
use std::path::PathBuf;

/// Information parsed from the header file.
#[derive(Debug)]
struct EntryInfo {
    name: String,
    index: u16,
    ty: Option<Type>,
    default: Option<Default>,
    msb: usize,
    lsb: usize,
    i2c_size: usize,
    fields: Vec<Field>,
    groups: Vec<String>,
}

#[derive(Debug)]
struct Field {
    name: String,
    // The last bit (inclusive).
    end: usize,
    // The first bit.
    start: usize,
    comment: Option<String>,
}

#[derive(Clone, Copy, Debug)]
enum Type {
    I16,
    I32,
    U8,
    U16,
    U32,
}

#[derive(Debug)]
enum Default {
    Named(String),
    Value(Value),
}

#[derive(Debug)]
enum Value {
    I16(i16),
    I32(i32),
    U8(u8),
    U16(u16),
    U32(u32),
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let src_path = PathBuf::from(args.next().unwrap());
    let dst_path = PathBuf::from(args.next().unwrap());

    // Retrieve the string.
    let src_string = std::fs::read_to_string(src_path).unwrap();

    // Split entries by their prefix and skip the top chunk.
    let entries = src_string.split("#define VL53L1_").skip(1);

    // Parse the entry info.
    let entry_infos: Vec<_> = entries.map(parse_entry_string).collect();

    // Generate the file.
    let rs = generate_reg_rs(&entry_infos);

    std::fs::write(&dst_path, rs.trim()).unwrap();
}

fn parse_entry_string(entry_s: &str) -> EntryInfo {
    // Read name and index from the first line.
    let l1 = entry_s.lines().next().unwrap();
    let mut tt = l1.split_whitespace();
    let name = tt.next().unwrap().to_string();
    let index_hex = tt.next().unwrap();
    let index = u16::from_str_radix(&index_hex[2..], 16).unwrap();

    // Find the entry type.
    let ty = entry_s.split("type:").nth(1).map(|s| {
        fn ty_from_str(s: &str) -> Type {
            match s {
                "int16_t" => Type::I16,
                "int32_t" => Type::I32,
                "uint8_t" => Type::U8,
                "uint16_t" => Type::U16,
                "uint32_t" => Type::U32,
                _ => panic!("unexpected type: {:?}", s),
            }
        }
        let ty_s = s.trim_start().split_whitespace().next().unwrap();
        ty_from_str(ty_s)
    });

    // The default value if there is one.
    let default = entry_s.split("default:").nth(1).map(|s| {
        let value_s = s.split("\\n").next().unwrap().trim();
        let ty = ty.as_ref().unwrap();
        fn parse_value(ty: &Type, s: &str) -> Option<Value> {
            let s = &s[2..];
            match ty {
                Type::I16 => i16::from_str_radix(s, 16).ok().map(Value::I16),
                Type::I32 => i32::from_str_radix(s, 16).ok().map(Value::I32),
                Type::U8 => u8::from_str_radix(s, 16).ok().map(Value::U8),
                Type::U16 => u16::from_str_radix(s, 16).ok().map(Value::U16),
                Type::U32 => u32::from_str_radix(s, 16).ok().map(Value::U32),
            }
        }
        parse_value(ty, value_s)
            .map(Default::Value)
            .unwrap_or_else(|| Default::Named(value_s.trim().to_string()))
    });

    // Parse the info.
    let info = entry_s.split("info: \\n").nth(1).unwrap();
    let msb = info
        .split("- msb =")
        .nth(1)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let lsb = info
        .split("- lsb =")
        .nth(1)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    let i2c_size = info
        .split("- i2c_size =")
        .nth(1)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    // Parse the groups.
    let groups: Vec<_> = entry_s
        .split("groups:")
        .nth(1)
        .map(|s| {
            let s = s.split("[").nth(1).unwrap();
            let s = s.split("]").next().unwrap();
            s.split(",")
                .map(|s| s.trim())
                .map(|s| s.split("'").nth(1).unwrap())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();

    // Parse the fields.
    let fields: Vec<_> = entry_s
        .split("fields:")
        .nth(1)
        .map(|s| {
            s.lines()
                .skip(1)
                .map(str::trim)
                .filter_map(|l| l.split("- ").nth(1))
                .map(|s| {
                    let (end, start) = {
                        let s = s.split("[").nth(1).unwrap().split("]").next().unwrap();
                        let mut split = s.split(":");
                        let end = split.next().unwrap().parse().unwrap();
                        let start = split.next().map(|s| s.parse().unwrap()).unwrap_or(msb);
                        (end, start)
                    };

                    let name_comment_s = s.split(" = ").nth(1).unwrap().trim_start();

                    let mut split = name_comment_s.split_whitespace();
                    let name = split.next().unwrap().trim().to_string();
                    let comment = split
                        .next()
                        .map(|_| name_comment_s[name.len()..].trim().to_string());
                    Field {
                        end,
                        start,
                        name,
                        comment,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    EntryInfo {
        name,
        index,
        ty,
        default,
        msb,
        lsb,
        i2c_size,
        groups,
        fields,
    }
}

/// Generate a rust file with:
///
/// - A `Entry` trait.
/// - A unique type for every entry along with `Entry` trait implementation.
///     - All entries without types are zero-sized.
///     - All entries with types are `bitfield!`s.
/// - An `Index` enum with a variant for every entry.
/// - A `State` enum with a variant for every entry.
fn generate_reg_rs(entry_infos: &[EntryInfo]) -> String {
    let header = r##"//! The register map.
//!
//! *This file is generated from the original `vl53l1_register_map.h` file via the
//! `generate-register-map` crate that exists within this same repository.*
//!
//! - See the `Entry` trait for information about entries within the register map.
//! - See the `Index` type for a dynamic representation of entry indices into the register map.
//! - See the `State` type for a dynamic representation of entry state.

#![allow(non_snake_case)]

pub mod settings;
pub mod structs;

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
"##
    .to_string();

    // Type definitions.
    let mut type_defs: Vec<String> = Vec::new();
    for entry in entry_infos {

        let ty = entry.ty.unwrap_or_else(|| ty_from_i2c_size(entry.i2c_size));
        let ty_size = ty_size(ty);

        // By ensuring we can infer size from type, we can reduce duplication of information.
        assert_eq!(ty_size, entry.i2c_size);

        let u_ty = ty_u_str(ty);
        let ty = ty_str(ty);

        let mut s = "bitfield! {".to_string();
        write!(
            &mut s,
            r#"
    #[derive(Clone, Copy, Eq, Hash, PartialEq)]
    pub struct {}({});
    impl Debug;
    {};"#,
            entry.name, u_ty, ty,
        )
        .unwrap();

        if entry.fields.is_empty()
            || entry.fields.len() == 1 // && entry.fields[0].name == &entry.name.to_lowercase()[..]
        {
            let bit_range = match entry.msb == entry.lsb {
                true => format!("{}", entry.msb),
                false => format!("{}, {}", entry.msb, entry.lsb),
            };
            write!(
                &mut s,
                r#"
    pub get, set: {};"#,
                bit_range,
            )
            .unwrap();
        } else {
            fn shortened_field_name(s: &str) -> &str {
                s.split("__").last().unwrap_or(s)
            }

            // The set of field indices that should use their original name due to
            // conflicts with the shortened name.
            let mut use_original_name = std::collections::HashSet::new();
            // Collect names for each field.
            let mut shortened_names = std::collections::HashMap::new();
            for (ix, f) in entry.fields.iter().enumerate() {
                let short_name = shortened_field_name(&f.name);
                if let Some(old_ix) = shortened_names.insert(short_name, ix) {
                    use_original_name.insert(old_ix);
                    use_original_name.insert(ix);
                }
            }

            // Getter setter for each field.
            for (ix, field) in entry.fields.iter().enumerate() {
                let bit_range = match field.start == field.end {
                    true => format!("{}", field.start),
                    false => format!("{}, {}", field.end, field.start),
                };

                // Avoid verbose field getters/setters.
                let name = match use_original_name.contains(&ix) {
                    true => &field.name,
                    false => shortened_field_name(&field.name),
                };

                // If the name would start with a number, ensure the getter name starts
                // with get.
                let getter = if name
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .parse::<u8>()
                    .is_ok()
                {
                    format!("get_{}", name)
                } else {
                    name.to_string()
                };

                write!(
                    &mut s,
                    r#"
    pub {}, set_{}: {};"#,
                    getter, name, bit_range,
                )
                .unwrap();
            }
        }

        s.push_str("\n}\n");
        type_defs.push(s.to_string());
    }

    // Index definition.
    let mut index_enum_s =
        r#"/// A dynamic representation of an entry's 16-bit index within the register map.
#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Index {
"#
        .to_string();
    let mut visited_indices = std::collections::HashSet::new();
    for entry in entry_infos {
        if visited_indices.insert(entry.index) {
            write!(
                &mut index_enum_s,
                "    {} = 0x{:04X},\n",
                entry.name, entry.index
            )
            .unwrap();
        }
    }
    index_enum_s.push_str("}\n");

    // State definition.
    let mut state_enum_s = r#"/// A dynamic representation of entry state.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum State {
"#
    .to_string();
    for entry in entry_infos {
        write!(&mut state_enum_s, "    {}({}),\n", entry.name, entry.name).unwrap();
    }
    state_enum_s.push_str("}\n");

    // Entry implementations.
    let mut entry_impls = String::new();
    let mut visited = std::collections::HashMap::new();
    for entry in entry_infos {
        let index_name = visited.entry(entry.index).or_insert_with(|| &entry.name);
        write!(&mut entry_impls, "impl Entry for {} ", entry.name).unwrap();
        entry_impls.push_str("{\n");

        // Associated consts, types.
        write!(
            &mut entry_impls,
            "    const INDEX: Index = Index::{};\n    type Array = [u8; {}];\n\n",
            index_name,
            entry.i2c_size,
        )
        .unwrap();

        // Methods.
        entry_impls.push_str("    fn into_array(self) -> Self::Array {
        self.0.to_be_bytes()
    }

    fn from_array(arr: Self::Array) -> Self {");

        write!(
            &mut entry_impls,
            "
        Self({}::from_be_bytes(arr))\n    ",
            ty_u_str(entry.ty.unwrap_or_else(|| ty_from_i2c_size(entry.i2c_size))),
        )
        .unwrap();
        entry_impls.push_str("}\n");

        entry_impls.push_str("}\n\n");
    }

    // Index implementation.
    let index_impl = r#"impl Into<[u8; 2]> for Index {
    /// Convert the index to two contiguous bytes as they should appear in I2C comms.
    fn into(self) -> [u8; 2] {
        (self as u16).to_be_bytes()
    }
}"#
    .to_string();

    Some(header)
        .into_iter()
        .chain(type_defs)
        .chain(Some(index_enum_s))
        .chain(Some(state_enum_s))
        .chain(Some(entry_impls))
        .chain(Some(index_impl))
        .map(|s| format!("{}\n", s))
        .collect()
}

fn ty_from_i2c_size(i2c_size: usize) -> Type {
    match i2c_size {
        1 => Type::U8,
        2 => Type::U16,
        4 => Type::U32,
        _ => unreachable!("unexpected `i2c_size`"),
    }
}

fn ty_size(ty: Type) -> usize {
    match ty {
        Type::I16 => 2,
        Type::I32 => 4,
        Type::U8 => 1,
        Type::U16 => 2,
        Type::U32 => 4,
    }
}

fn ty_str(ty: Type) -> &'static str {
    match ty {
        Type::I16 => "i16",
        Type::I32 => "i32",
        Type::U8 => "u8",
        Type::U16 => "u16",
        Type::U32 => "u32",
    }
}

fn ty_u_str(ty: Type) -> &'static str {
    match ty {
        Type::I16 => "u16",
        Type::I32 => "u32",
        Type::U8 => "u8",
        Type::U16 => "u16",
        Type::U32 => "u32",
    }
}
