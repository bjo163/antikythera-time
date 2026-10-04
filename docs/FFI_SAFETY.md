# FFI Safety Boundary

`mtime-ffi` is the only crate that does not inherit the workspace-level `unsafe_code = forbid` lint.

It instead uses `unsafe_code = deny` and a local `#[allow(unsafe_code)]` only on the exported `#[no_mangle]` function because current Rust treats symbol-export attributes as unsafe-code lint events.

The exported API:

- takes only value parameters;
- returns a `#[repr(C)]` value struct;
- accepts no raw pointers;
- dereferences no pointers;
- contains no `unsafe { ... }` block.

This exception permits the symbol attribute, not general unsafe implementation.
