use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Failed to open input image: {0}")]
    ImageOpen(#[source] image::ImageError),

    #[error("Failed to save output image: {0}")]
    ImageSave(#[source] image::ImageError),

    #[error("Failed to read parameters file: {0}")]
    ParamsRead(#[source] std::io::Error),

    #[error("Plugin library not found: {0}")]
    PluginNotFound(String),

    #[error("Failed to load plugin library: {0}")]
    PluginLoad(#[source] libloading::Error),

    #[error("Failed to find process_image function in plugin: {0}")]
    PluginSymbol(#[source] libloading::Error),

    #[error("Invalid image dimensions: width={0}, height={1}")]
    InvalidDimensions(u32, u32),

    #[error("Buffer size mismatch: expected={0}, actual={1}")]
    BufferSizeMismatch(usize, usize),

    #[error("Plugin execution failed")]
    PluginExecution,
}