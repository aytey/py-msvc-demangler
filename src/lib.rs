use pyo3::exceptions;
use pyo3::prelude::*;

// This defines a python module. pyo3 will copy the rust doc comment
// below into a python docstring

/// A package for demangling MSVC C++ linker symbols
///
/// This package provides python bindings for the rust crate
/// [msvc_demangler](https://github.com/mstange/msvc-demangler-rust) by building
/// a native Python extension using [PyO3](https://github.com/pyO3/pyO3)
///
/// Basic usage:
///
/// >>> demangle('??_0klass@@QEAAHH@Z')
/// 'public: int __cdecl klass::operator/=(int)'
///
/// Passing an invalid identifier will throw a ValueError:
///
/// >>> demangle('invalid C++ symbol')
/// Traceback (most recent call last):
/// ...
/// ValueError: ('Could not format demangled name as string', 'does not start with b\'?\' (offset: 0, remaining: "invalid C++ symbol")')
#[pymodule]
fn msvc_demangler(_py: Python, m: &PyModule) -> PyResult<()> {
    // This adds a function to the python module:
    /// Demangles a mangled c++ linker symbol name and returns it as a string
    #[pyfn(m)]
    fn demangle(mangled: String) -> PyResult<String> {
        let flags = ::msvc_demangler::DemangleFlags::llvm();
        let demangled = ::msvc_demangler::demangle(&mangled[..], flags).map_err(|error| {
            exceptions::PyValueError::new_err((
                "Could not format demangled name as string",
                error.to_string(),
            ))
        })?;

        Ok(demangled)
    }

    Ok(())
}
