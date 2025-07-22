use std::ffi::CString;
use std::path::Path;

use super::*;

/// Safe wrapper for MicroTeX LaTeX rendering
pub struct MicroTeX {
    initialized: bool,
}

impl MicroTeX {
    /// Initialize MicroTeX with the resource directory
    pub fn init<P: AsRef<Path>>(res_path: P) -> Result<Self, String> {
        let path_str = res_path.as_ref()
            .to_str()
            .ok_or_else(|| "Invalid path encoding".to_string())?;
        
        let c_path = CString::new(path_str)
            .map_err(|e| format!("Failed to create C string: {}", e))?;
        
        unsafe {
            microtex_init_with_path(c_path.as_ptr());
        }
        
        Ok(MicroTeX { initialized: true })
    }
    
    /// Initialize with default resource path
    pub fn init_default() -> Result<Self, String> {
        unsafe {
            microtex_init();
        }
        Ok(MicroTeX { initialized: true })
    }
    
    /// Set debug mode
    pub fn set_debug(&self, debug: bool) {
        unsafe {
            microtex_set_debug(if debug { 1 } else { 0 });
        }
    }
    
    /// Parse LaTeX code and create a renderer
    pub fn parse(&self, latex: &str, width: i32, text_size: f32, line_space: f32, color: u32) -> Result<TeXRenderer, String> {
        if !self.initialized {
            return Err("MicroTeX not initialized".to_string());
        }
        
        let c_latex = CString::new(latex)
            .map_err(|e| format!("Failed to create C string: {}", e))?;
        
        unsafe {
            let handle = microtex_parse(
                c_latex.as_ptr(),
                width,
                text_size,
                line_space,
                color
            );
            
            if handle.is_null() {
                Err("Failed to parse LaTeX".to_string())
            } else {
                Ok(TeXRenderer { handle })
            }
        }
    }
}

impl Drop for MicroTeX {
    fn drop(&mut self) {
        if self.initialized {
            unsafe {
                microtex_release();
            }
        }
    }
}

/// Safe wrapper for TeXRender
pub struct TeXRenderer {
    handle: TeXRender_Handle,
}

impl TeXRenderer {
    /// Get the width of the rendered formula
    pub fn get_width(&self) -> i32 {
        unsafe {
            microtex_render_get_width(self.handle)
        }
    }
    
    /// Get the height of the rendered formula
    pub fn get_height(&self) -> i32 {
        unsafe {
            microtex_render_get_height(self.handle)
        }
    }
    
    /// Get the depth of the rendered formula
    pub fn get_depth(&self) -> i32 {
        unsafe {
            microtex_render_get_depth(self.handle)
        }
    }
    
    /// Get the baseline of the rendered formula
    pub fn get_baseline(&self) -> f32 {
        unsafe {
            microtex_render_get_baseline(self.handle)
        }
    }
    
    /// Draw to an RGBA buffer
    pub fn draw_to_buffer(&self, buffer: &mut [u8], width: i32, height: i32, x: i32, y: i32) -> Result<(), String> {
        if buffer.len() < (width * height * 4) as usize {
            return Err("Buffer too small".to_string());
        }
        
        unsafe {
            microtex_render_draw_to_buffer(
                self.handle,
                buffer.as_mut_ptr(),
                width,
                height,
                x,
                y
            );
        }
        
        Ok(())
    }
}

impl Drop for TeXRenderer {
    fn drop(&mut self) {
        unsafe {
            microtex_render_free(self.handle);
        }
    }
}