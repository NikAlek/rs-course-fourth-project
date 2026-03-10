use libloading::Library;
use std::ffi::CString;
use std::path::{Path, PathBuf};
use crate::error::{ProcessorError};
use std::rc::Rc;


pub type ProcessImageFn = unsafe extern "C" fn(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const std::os::raw::c_char,
);

// Загрузчик плагинов
pub struct Plugin {
    library: Library,
    process_fn: ProcessImageFn,
}

impl Plugin {
    pub fn load(plugin_path: &Path, plugin_name: &str) -> Result<Self, ProcessorError> {
    
        let lib_path = Self::find_library_path(plugin_path, plugin_name)?;

        let library = unsafe {
            Library::new(&lib_path).map_err(ProcessorError::PluginLoad)?
        };

    

        let process_fn: libloading::Symbol<ProcessImageFn> = unsafe {
            library
                .get(b"process_image\0")
                .map_err(ProcessorError::PluginSymbol)?
        };


        let process_fn = unsafe { std::mem::transmute(process_fn) };

        Ok(Self {
            library,
            process_fn,
        })
    }

    fn find_library_path(plugin_path: &Path, plugin_name: &str) -> Result<PathBuf,ProcessorError> {
        let lib_name = Self::platform_lib_name(plugin_name)?;
        let full_path = plugin_path.join(&lib_name);

        if !full_path.exists() {
            return Err(ProcessorError::PluginNotFound(format!(
                "Expected: {:?}",
                full_path
            )));
        }

        Ok(full_path)
    }

    fn platform_lib_name(plugin_name: &str) -> Result<String, ProcessorError> {
        #[cfg(target_os = "linux")]
        return Ok(format!("lib{}.so", plugin_name));

        #[cfg(target_os = "windows")]
        return Ok(format!("{}.dll", plugin_name));

        #[cfg(target_os = "macos")]
        return Ok(format!("lib{}.dylib", plugin_name));

        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        return Err(ProcessorError::UnknownTargetPlatform);
    }

    pub unsafe fn process(
        &self,
        width: u32,
        height: u32,
        rgba_data: *mut u8,
        params: &str,
    ) -> Result<(), ProcessorError> {
        let params_cstr = CString::new(params).map_err(|_| ProcessorError::PluginExecution)?;

        (self.process_fn)(width, height, rgba_data, params_cstr.as_ptr());

        Ok(())
    }
}