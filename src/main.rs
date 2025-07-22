use ffl_rust::MicroTeX;
use image::{RgbImage, Rgb};
use sixel_rs::{
    optflags::DiffusionMethod,
    pixelformat::PixelFormat,
    sixel_string,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize MicroTeX with the resource path
    let res_path = "ext/MicroTeX/res";
    println!("Initializing MicroTeX with resources from: {}", res_path);
    let microtex = MicroTeX::init(res_path)?;
    
    // Disable debug mode for clean output
    microtex.set_debug(false);
    
    // Test LaTeX formulas
    let formulas = vec![
        r"E = mc^2",
        r"\frac{d}{dx} \sin(x) = \cos(x)",
        r"\sqrt[3]{(x-y)^3} = x-y",
        r"\int_{0}^{x}\frac1{t} \mathrm{d}t = \mathrm{ln}\, x",
    ];
    
    for (i, formula) in formulas.iter().enumerate() {
        println!("\n--- Rendering formula {} ---", i + 1);
        println!("LaTeX: {}", formula);
        
        // Parse the LaTeX formula
        let width = 800;
        let text_size = 20.0;
        let line_space = 5.0;
        let color = 0xFF000000; // Black
        
        let renderer = match microtex.parse(formula, width, text_size, line_space, color) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to parse formula: {}", e);
                continue;
            }
        };
        
        // Get dimensions
        let render_width = renderer.get_width();
        let render_height = renderer.get_height();
        let baseline = renderer.get_baseline();
        
        println!("Dimensions: {}x{}, baseline: {}", render_width, render_height, baseline);
        
        // Create an image buffer
        let mut img = RgbImage::from_pixel(
            render_width as u32,
            render_height as u32,
            Rgb([255, 255, 255]) // White background
        );
        
        // Create a temporary ARGB buffer for Cairo rendering with white background
        let mut argb_buffer = vec![0u8; (render_width * render_height * 4) as usize];
        // Initialize with white background (ARGB format)
        for i in (0..argb_buffer.len()).step_by(4) {
            argb_buffer[i] = 255;     // B
            argb_buffer[i + 1] = 255; // G  
            argb_buffer[i + 2] = 255; // R
            argb_buffer[i + 3] = 255; // A
        }
        renderer.draw_to_buffer(&mut argb_buffer, render_width, render_height, 0, 0)?;
        
        // Convert buffer back to image (ARGB -> RGB conversion, dropping alpha)
        for y in 0..render_height as u32 {
            for x in 0..render_width as u32 {
                let idx = ((y * render_width as u32 + x) * 4) as usize;
                // Cairo uses ARGB format, we need RGB (drop alpha)
                let r = argb_buffer[idx + 2];
                let g = argb_buffer[idx + 1];
                let b = argb_buffer[idx];
                img.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
        
        // Get dimensions and convert to raw bytes for sixel
        let (width, height) = img.dimensions();
        let bytes: Vec<u8> = img
            .pixels()
            .flat_map(|pixel| pixel.0.to_vec())
            .collect();
        
        // Generate sixel output
        match sixel_string(
            &bytes,
            width as i32,
            height as i32,
            PixelFormat::RGB888,
            DiffusionMethod::Atkinson,
        ) {
            Ok(sixel_output) => {
                println!("Sixel output:");
                println!("{}", sixel_output);
            }
            Err(e) => {
                eprintln!("Failed to generate sixel: {:?}", e);
            }
        }
    }
    
    println!("\nMicroTeX binding test completed successfully!");
    println!("LaTeX formulas rendered and displayed as SIXEL graphics.");
    Ok(())
}
