# Wasm debug

A general purpose crate for dealing with [Wasm DWARF][wasm-dwarf] for
and transforming it into DWARF that can be included in a compiled
object file or given to a debugger via the [GDB JIT interface][gdb-jit-interface].

This project is a fork of [`wasmtime-debug`][] that is not dependent
on any runtime.

When applying fixes to this crate, please send a copy of the patch to
[`wasmtime-debug][] if applicable.

When merging in fixes found in [`wasmtime-debug`] please be mindful of
updating copyright notices and possible license changes.

## Licenses

This crate contains code that is licensed and/or may be licensed under
the following licenses:

Apache 2.0 with LLVM exception:

```
   Copyright 2020 The Wasmtime Project Developers

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

Apache 2.0 with LLVM exception:

```
   Copyright 2020-present Wasmer, inc.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
```

See the [license][] for more information.

[`wasmtime-debug`]: https://crates.io/crates/wasmtime-debug
[wasm-dwarf]: https://yurydelendik.github.io/webassembly-dwarf/
[gdb-jit-interface]: https://sourceware.org/gdb/current/onlinedocs/gdb/JIT-Interface.html
[license]: LICENSE
