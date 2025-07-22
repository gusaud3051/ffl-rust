mod microtex;

use microtex::wrapper::MicroTeX;
use image::{RgbaImage, Rgba};

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
        let mut img = RgbaImage::from_pixel(
            render_width as u32,
            render_height as u32,
            Rgba([255, 255, 255, 255]) // White background
        );
        
        // Render to the buffer
        let mut buffer = img.as_mut().to_vec();
        renderer.draw_to_buffer(&mut buffer, render_width, render_height, 0, 0)?;
        
        // Convert buffer back to image (ARGB -> RGBA conversion)
        for y in 0..render_height as u32 {
            for x in 0..render_width as u32 {
                let idx = ((y * render_width as u32 + x) * 4) as usize;
                // Cairo uses ARGB format, we need RGBA
                let a = buffer[idx + 3];
                let r = buffer[idx + 2];
                let g = buffer[idx + 1];
                let b = buffer[idx];
                img.put_pixel(x, y, Rgba([r, g, b, a]));
            }
        }
        
        // Save as PNG for verification
        let png_path = format!("formula_{}.png", i + 1);
        img.save(&png_path)?;
        println!("Saved PNG to: {}", png_path);
    }
    
    println!("\nMicroTeX binding test completed successfully!");
    println!("Generated PNG files can be viewed to verify LaTeX rendering is working.");
    Ok(())
}
