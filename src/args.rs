use clap::Parser;
use std::path::PathBuf;
use crate::error::{ProcessorError};


#[derive(Parser, Debug)]
#[command(
    about = "Image processor with dynamic plugin support",
)]
pub struct Args {
    /// Путь к исходному PNG-изображению
    #[arg(short, long, value_name = "PATH")]
    pub input: PathBuf,

    /// Путь для сохранения обработанного изображения
    #[arg(short, long, value_name = "PATH")]
    pub output: PathBuf,

    /// Имя плагина без расширения (например, mirror)
    #[arg(short = 'P', long, value_name = "NAME")]
    pub plugin: String,

    /// Путь к файлу с параметрами обработки
    #[arg(short, long, value_name = "PATH")]
    pub params: PathBuf,

    /// Путь к директории с плагинами
    #[arg(long, value_name = "PATH", default_value = "target/debug")]
    pub plugin_path: PathBuf,

}

impl Args {
    pub fn validate(&self) -> Result<(), ProcessorError> {

        if !self.input.exists() {
            return Err(ProcessorError::ParamsRead(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Input file not found: {:?}", self.input),
            )));
        }

        if !self.params.exists() {
            return Err(ProcessorError::ParamsRead(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Params file not found: {:?}", self.params),
            )));
        }

        if !self.plugin_path.exists() {
            return Err(ProcessorError::PluginNotFound(format!(
                "Plugin directory not found: {:?}",
                self.plugin_path
            )));
        }

        if self.input == self.output {
            return Err(ProcessorError::ParamsRead(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Input and output paths cannot be the same",
            )));
        }

        Ok(())
    }
}