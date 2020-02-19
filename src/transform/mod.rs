use crate::gc::build_dependencies;
use crate::types::{ModuleAddressMap, ModuleVmctxInfo, ValueLabelsRanges};
use crate::DebugInfoData;
use anyhow::Error;
use gimli::{
    write, DebugAddr, DebugAddrBase, DebugLine, DebugStr, LocationLists, RangeLists,
    UnitSectionOffset,
};
use simulate::generate_simulated_dwarf;
use std::collections::HashSet;
use thiserror::Error;
use unit::clone_unit;

pub use address_transform::AddressTransform;

mod address_transform;
mod attr;
mod expression;
mod line_program;
mod range_info_builder;
mod simulate;
mod unit;
mod utils;

pub(crate) trait Reader: gimli::Reader<Offset = usize> {}

impl<'input, Endian> Reader for gimli::EndianSlice<'input, Endian> where Endian: gimli::Endianity {}

#[derive(Error, Debug)]
#[error("Debug info transform error: {0}")]
pub struct TransformError(&'static str);

pub(crate) struct DebugInputContext<'a, R>
where
    R: Reader,
{
    debug_str: &'a DebugStr<R>,
    debug_line: &'a DebugLine<R>,
    debug_addr: &'a DebugAddr<R>,
    debug_addr_base: DebugAddrBase<R::Offset>,
    rnglists: &'a RangeLists<R>,
    loclists: &'a LocationLists<R>,
    reachable: &'a HashSet<UnitSectionOffset>,
}

pub fn transform_dwarf(
    pointer_width_bytes: u8,
    di: &DebugInfoData,
    at: &ModuleAddressMap,
    vmctx_info: &ModuleVmctxInfo,
    ranges: &ValueLabelsRanges,
) -> Result<write::Dwarf, Error> {
    let addr_tr = AddressTransform::new(at, &di.wasm_file);
    let reachable = build_dependencies(&di.dwarf, &addr_tr)?.get_reachable();

    let context = DebugInputContext {
        debug_str: &di.dwarf.debug_str,
        debug_line: &di.dwarf.debug_line,
        debug_addr: &di.dwarf.debug_addr,
        debug_addr_base: DebugAddrBase(0),
        rnglists: &di.dwarf.ranges,
        loclists: &di.dwarf.locations,
        reachable: &reachable,
    };

    let out_encoding = gimli::Encoding {
        format: gimli::Format::Dwarf32,
        // TODO: this should be configurable
        // macOS doesn't seem to support DWARF > 3
        version: 3,
        address_size: pointer_width_bytes,
    };

    let mut out_strings = write::StringTable::default();
    let mut out_units = write::UnitTable::default();

    let out_line_strings = write::LineStringTable::default();

    let mut translated = HashSet::new();
    let mut iter = di.dwarf.debug_info.units();
    while let Some(unit) = iter.next().unwrap_or(None) {
        let unit = di.dwarf.unit(unit)?;
        clone_unit(
            unit,
            &context,
            &addr_tr,
            &ranges,
            out_encoding,
            &vmctx_info,
            &mut out_units,
            &mut out_strings,
            &mut translated,
        )?;
    }

    generate_simulated_dwarf(
        &addr_tr,
        di,
        &vmctx_info,
        &ranges,
        &translated,
        out_encoding,
        &mut out_units,
        &mut out_strings,
    )?;

    Ok(write::Dwarf {
        units: out_units,
        line_programs: vec![],
        line_strings: out_line_strings,
        strings: out_strings,
    })
}
