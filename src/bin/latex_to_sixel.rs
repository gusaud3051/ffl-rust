use clap::Parser;
use std::io::{self, Read};
use ffl_rust::MicroTeX;
use image::{RgbImage, Rgb};
use sixel_rs::{
    optflags::DiffusionMethod,
    pixelformat::PixelFormat,
    sixel_string,
};

#[derive(Parser, Debug)]
#[command(name = "latex2sixel", author, version, about = "Convert LaTeX formulas to SIXEL graphics", long_about = None)]
struct Args {
    /// Print all information (overrides quiet mode)
    #[arg(short, long)]
    verbose: bool,

    /// Print only result SIXEL graphics (default)
    #[arg(short, long)]
    quiet: bool,

    /// LaTeX string to render (use '-' or omit to read from stdin)
    latex_string: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine verbosity level (verbose overrides quiet)
    let verbose = args.verbose;
    let _quiet = !args.verbose; // quiet is default unless verbose is set

    // Get LaTeX formula from args or stdin
    let latex_formula = match args.latex_string {
        Some(ref formula) if formula != "-" => formula.clone(),
        _ => {
            // Read from stdin
            if verbose {
                eprintln!("Reading LaTeX formula from stdin...");
            }
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer.trim().to_string()
        }
    };

    if latex_formula.is_empty() {
        eprintln!("Error: No LaTeX formula provided");
        eprintln!("Usage: latex2sixel 'formula' or echo 'formula' | latex2sixel");
        std::process::exit(1);
    }

    if verbose {
        eprintln!("LaTeX formula: {}", latex_formula);
    }

    // Initialize MicroTeX with the resource path
    let res_path = "ext/MicroTeX/res";
    if verbose {
        eprintln!("Initializing MicroTeX with resource path: {}", res_path);
    }
    let microtex = MicroTeX::init(res_path)?;
    
    // Disable debug mode for clean output
    microtex.set_debug(false);
    
    // Parse the LaTeX formula
    let width = 800;
    let text_size = 20.0;
    let line_space = 5.0;
    let color = 0xFF000000; // Black
    
    if verbose {
        eprintln!("Parsing LaTeX with settings:");
        eprintln!("  Width: {}", width);
        eprintln!("  Text size: {}", text_size);
        eprintln!("  Line space: {}", line_space);
        eprintln!("  Color: 0x{:08X}", color);
    }
    
    let renderer = match microtex.parse(&latex_formula, width, text_size, line_space, color) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to parse LaTeX formula: {}", e);
            std::process::exit(1);
        }
    };
    
    // Get dimensions
    let render_width = renderer.get_width();
    let render_height = renderer.get_height();
    
    if verbose {
        eprintln!("Render dimensions: {}x{}", render_width, render_height);
    }
    
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
    
    if verbose {
        eprintln!("Rendering LaTeX to buffer...");
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
    
    if verbose {
        eprintln!("Converting to SIXEL format...");
    }
    
    // Generate sixel output
    match sixel_string(
        &bytes,
        width as i32,
        height as i32,
        PixelFormat::RGB888,
        DiffusionMethod::Atkinson,
    ) {
        Ok(sixel_output) => {
            if verbose {
                eprintln!("SIXEL generation successful");
            }
            print!("{}", sixel_output);
        }
        Err(e) => {
            eprintln!("Failed to generate sixel: {:?}", e);
            std::process::exit(1);
        }
    }
    
    Ok(())
}