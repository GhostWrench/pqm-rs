/// This is an auto-generated data file with unit definitions
/// 
/// To generate this file, run the python script located in ./scripts/buildunitdefs.py
///

use ::phf::phf_map;

pub const NUM_DIMENSION_TYPES: usize = 7;

pub static PREFIXES : phf::Map<&str, f64> = phf_map! {
    "u" => 1.0e-6,
    "m" => 1.0e-3,
    "k" => 1.0e3,
    "M" => 1.0e6,
};

pub static UNITS : phf::Map<&str, (f64, [i32; NUM_DIMENSION_TYPES])> = phf_map! {
    "1" => (1.0, [0,0,0,0,0,0,0]),
    "%" => (1.0, [0,0,0,0,0,0,0]),
    "g" => (1.0e-3, [1,0,0,0,0,0,0]),
    "m" => (1.0, [0,1,0,0,0,0,0]),
    "in" => (2.54e-2, [0,1,0,0,0,0,0]),
};
