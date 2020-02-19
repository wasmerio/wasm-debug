//! Data structures and functions used to get metadata into a format this
//! crate can understand.

use cranelift_entity::{EntityRef, PrimaryMap};
use std::collections::HashMap;

/// Index of a function.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DefinedFuncIndex(usize);

impl EntityRef for DefinedFuncIndex {
    fn new(v: usize) -> Self {
        Self(v)
    }

    fn index(self) -> usize {
        self.0
    }
}

/// Offset into the Wasm starting at the code section.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SourceLoc(u32);

impl SourceLoc {
    /// Create a `SourceLoc`.
    pub fn new(v: u32) -> Self {
        Self(v)
    }

    /// Get the inner value.
    pub fn get(&self) -> u32 {
        self.0
    }

    /// Check if this is the default `SourceLoc`.
    pub fn is_default(&self) -> bool {
        self.0 == !0
    }
}

/// Information about a compiled function.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompiledFunctionData {
    /// Information about the instructions in this function in order.
    pub instructions: Vec<CompiledInstructionData>,
    /// The start location in the Wasm of this function.
    pub start: SourceLoc,
    /// The end location in the Wasm of this function.
    pub end: SourceLoc,
    /// The offset into the compiled code where this function is.
    pub compiled_offset: usize,
    /// The size of the compiled function.
    pub compiled_size: usize,
}

/// Information about a compiled WebAssembly instruction.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CompiledInstructionData {
    /// The location in the Wasm of the instruction.
    pub loc: SourceLoc,
    /// The length of the instruction in bytes.
    pub length: usize,
    /// The offset from the start of the function? (TODO: figure out what this is).
    pub offset: usize,
}

/// Build a [`ModuleAddressMap`].
pub fn create_module_address_map<'a, I>(info: I) -> ModuleAddressMap
where
    I: Iterator<Item = &'a CompiledFunctionData>,
{
    let mut map = PrimaryMap::new();
    for cfd in info {
        map.push(cfd.clone());
    }
    map
}

/// Mapping to compiled functions.
pub type ModuleAddressMap = PrimaryMap<DefinedFuncIndex, CompiledFunctionData>;

/// Type to track in which register a value is located.
pub type RegUnit = u16;

/// Type to track where on the stack a value is located.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct StackSlot(u32);

impl StackSlot {
    /// Create a stack slot.
    pub fn from_u32(x: u32) -> Self {
        Self(x)
    }

    /// Get the inner value.
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

/// Type used to keep track of values during compilation.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct ValueLabel(u32);

impl ValueLabel {
    /// Create a value label.
    pub fn from_u32(x: u32) -> Self {
        Self(x)
    }

    /// Get the inner value.
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl EntityRef for ValueLabel {
    fn new(v: usize) -> Self {
        Self(v as u32)
    }

    fn index(self) -> usize {
        self.0 as usize
    }
}

/// The location where a value is.
#[derive(Debug, Clone, Copy)]
pub enum ValueLoc {
    Unassigned,
    /// Value is in a register.
    Reg(RegUnit),
    /// Value is at this location on the stack.
    Stack(StackSlot),
}

/// A range in which a value is valid.
#[derive(Debug, Clone)]
pub struct ValueLocRange {
    /// Where the value is.
    pub loc: ValueLoc,
    /// Where it starts being there.
    pub start: u32,
    /// Where it stops being there.
    pub end: u32,
}

/// Create a [`ValueLabelsRanges`] from data.
pub fn build_values_ranges<'a, I>(vlri_iter: I) -> ValueLabelsRanges
where
    I: Iterator<Item = &'a ValueLabelsRangesInner>,
{
    let mut map = PrimaryMap::new();

    for i in vlri_iter {
        map.push(i.clone());
    }

    map
}

/// Map of functions to information about when and where its values are valid.
pub type ValueLabelsRanges = PrimaryMap<DefinedFuncIndex, ValueLabelsRangesInner>;
/// Map of [`ValueLabel`] to all the locations that it's valid at.
pub type ValueLabelsRangesInner = HashMap<ValueLabel, Vec<ValueLocRange>>;

// Temporary code.  Code using this should be restructured or removed probably.
// seems too backend specific
pub fn get_vmctx_value_label() -> ValueLabel {
    // copied from cranelift_wasm
    ValueLabel(0xffff_fffe)
}

/// Information about the module and VM context.
pub struct ModuleVmctxInfo {
    /// Offset from the VMCtx where a pointer to memory can be found.
    ///
    /// Assume memory 0 for now.
    pub memory_offset: i64,
    /// The size of the VMCtx struct
    pub vmctx_size: i64,
    /// The offsets of the stack slots for each function.
    pub stack_slot_offsets: PrimaryMap<DefinedFuncIndex, Vec<Option<i32>>>,
}

impl ModuleVmctxInfo {
    pub fn new<'a, I>(memory_offset: i64, vmctx_size: i64, stack_slot_offsets: I) -> Self
    where
        I: Iterator<Item = &'a Vec<Option<i32>>>,
    {
        let mut map = PrimaryMap::new();
        for o in stack_slot_offsets {
            map.push(o.clone());
        }
        Self {
            memory_offset,
            vmctx_size,
            stack_slot_offsets: map,
        }
    }
}
