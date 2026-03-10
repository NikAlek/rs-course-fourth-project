use image::{DynamicImage, RgbaImage};   
use crate::error::{ProcessorError};

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl ImageData {
    pub fn from_path(path: &std::path::Path) -> Result<Self, ProcessorError> {
        let img = image::open(path)
            .map_err(ProcessorError::ImageOpen)?
            .to_rgba8();

        let (width, height) = img.dimensions();

        if width == 0 || height == 0 {
            return Err(ProcessorError::InvalidDimensions(width, height));
        }

        let expected_size = width
            .checked_mul(height)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or_else(|| ProcessorError::InvalidDimensions(width, height))?;

        let pixels = img.into_raw();

        if pixels.len() != expected_size as usize {
            return Err(ProcessorError::BufferSizeMismatch(
                expected_size as usize,
                pixels.len(),
            ));
        }

        Ok(Self {
            width,
            height,
            pixels,
        })
    }

    // Сохраняет изображение в файл
    pub fn save(&self, path: &std::path::Path) -> Result<(), ProcessorError> {
        let img = RgbaImage::from_raw(self.width, self.height, self.pixels.clone())
            .ok_or_else(|| ProcessorError::InvalidDimensions(self.width, self.height))?;

        img.save(path).map_err(ProcessorError::ImageSave)?;
        Ok(())
    }

    // Получает мутабельный срез пикселей
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    // Вычисляет ожидаемый размер буфера в байтах
    pub fn expected_buffer_size(&self) -> usize {
        (self.width as usize) * (self.height as usize) * 4 
    }
}