/// This is an auto-generated definition file for prefix and unit symbols. Do not edit it directly,
/// rather generate it using the python script located in scripts/builddefs.py. To run this script 
/// you need Python 3 with the Jinja 2 package installed.
/// 
{# This is a template only, not valid rust code. #}
use ::phf::phf_map;

pub const NUM_DIMENSION_TYPES: usize = {{num_dimension_types}};

pub const DIMENSION_TYPES : [&str ; NUM_DIMENSION_TYPES] = [
{% for item in dimension_types -%}
    {{ " "*4 }}"{{ item }}",
{% endfor -%}
];

pub static PREFIXES : phf::Map<&str, f64> = phf_map! {
{% for item in prefixes -%}
    {{ " "*4 }}"{{ item.symbol }}" => {{ item.scale }},
{% endfor -%}
};

pub static UNITS : phf::Map<&str, (f64, [i32; NUM_DIMENSION_TYPES])> = phf_map! {
{% for item in units -%}
    {{ " "*4 }}"{{ item.symbol }}" => ({{ item.scale }}, [{{ item.dims }}]),
{% endfor -%}
};