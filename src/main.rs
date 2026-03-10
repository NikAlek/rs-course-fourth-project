use clap::Parser;
use image_processor::{Args, ImageData, Plugin, ProcessorError};

fn main() -> Result<(), ProcessorError> {
    let args = Args::parse();   

    args.validate()?;

    println!("Input:  {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Plugin: {:?}", args.plugin);
    println!("Params: {:?}", args.params);

    // Загрузка изображения
    println!("\nLoading image...");
    let mut image_data = ImageData::from_path(&args.input)?;

    println!(
        "Image loaded: {}x{} ({} bytes)",
        image_data.width,
        image_data.height,
        image_data.pixels.len()
    );

    println!("\nReading parameters.");
    let params_content = std::fs::read_to_string(&args.params)
        .map_err(ProcessorError::ParamsRead)?;
    println!("Parameters loaded ({} bytes)", params_content.len());


    println!("Loading plugin.");
    let plugin = Plugin::load(&args.plugin_path, &args.plugin)?;
    println!("Plugin loaded from {:?}", args.plugin_path);


    unsafe {
        plugin.process(
            image_data.width,
            image_data.height,
            image_data.pixels_mut().as_mut_ptr(),
            &params_content,
        )?;
    }

    println!("\nSaving result.");
    image_data.save(&args.output)?;
    println!("Result saved to {:?}", args.output);

    Ok(())
}
