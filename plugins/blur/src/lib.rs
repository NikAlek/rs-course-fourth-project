

use std::os::raw::c_char;
use std::ffi::CStr;

struct BlurParams {
    // Размер ядра размытия
    kernel_size: u32,
    // Количество итераций размытия
    iterations: u32,
}

impl Default for BlurParams {
    fn default() -> Self {
        Self {
            kernel_size: 3,
            iterations: 1,
        }
    }
}

impl BlurParams {

    // Формат: "kernel_size=5,iterations=2"
    fn parse(params: &str) -> Self {
        let mut result = Self::default();

        for param in params.split(',') {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();

                match key {
                    "kernel_size" => {
                        if let Ok(size) = value.parse::<u32>() {
                            // Kernel size должен быть нечётным и >= 3
                            result.kernel_size = if size < 3 {
                                3
                            } else if size % 2 == 0 {
                                size + 1
                            } else {
                                size
                            };
                        }
                    }
                    "iterations" => {
                        if let Ok(iter) = value.parse::<u32>() {
                            result.iterations = iter.max(1).min(10);
                        }
                    }
                    _ => {}
                }
            }
        }

        println!("[blur] Parsed params: kernel_size={}, iterations={}", 
                  result.kernel_size, result.iterations);

        result
    }

    // Возвращает радиус размытия (половина размера ядра)
    fn radius(&self) -> u32 {
        self.kernel_size / 2
    }
}


#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) {

    let params_str = unsafe {
        if params.is_null() {
            String::new()
        } else {
            CStr::from_ptr(params)
                .to_string_lossy()
                .into_owned()
        }
    };

    println!("[blur] Called with dimensions: {}x{}", width, height);
    println!("[blur] Raw params: {}", params_str);


    if width <= 0 || height <= 0 {
        eprintln!("[blur] Error: Invalid dimensions");
        return;
    }


    let blur_params = BlurParams::parse(&params_str);


    unsafe {
        let total_bytes = match (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(4))
        {
            Some(size) => size,
            None => {
                eprintln!("[blur] Error: Buffer size overflow");
                return;
            }
        };

        let data = std::slice::from_raw_parts_mut(rgba_data, total_bytes);

        // Применяем размытие указанное количество раз
        for iteration in 0..blur_params.iterations {
            println!("[blur] Iteration {}/{}", iteration + 1, blur_params.iterations);
            apply_box_blur(data, width, height, blur_params.radius());
        }
    }

    println!("[blur] Processing complete");
}


unsafe fn apply_box_blur(data: &mut [u8], width: u32, height: u32, radius: u32) {
    let width = width as usize;
    let height = height as usize;
    let radius = radius as usize;

    // Создаём копию исходных данных для чтения
    let original = data.to_vec();

    // Для каждого пикселя вычисляем среднее значение в окне размытия
    for y in 0..height {
        for x in 0..width {
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            let mut count = 0u32;

            // Проходим по окну размытия
            for dy in 0..=(radius * 2) {
                for dx in 0..=(radius * 2) {
                    let ny = y as i32 + dy as i32 - radius as i32;
                    let nx = x as i32 + dx as i32 - radius as i32;

                    // Проверяем границы изображения
                    if ny >= 0 && ny < height as i32 && nx >= 0 && nx < width as i32 {
                        let idx = ((ny as usize) * width + (nx as usize)) * 4;
                        sum_r += original[idx] as u32;
                        sum_g += original[idx + 1] as u32;
                        sum_b += original[idx + 2] as u32;
                        count += 1;
                    }
                }
            }

            // Вычисляем среднее значение
            if count > 0 {
                let idx = (y * width + x) * 4;
                data[idx] = (sum_r / count) as u8;
                data[idx + 1] = (sum_g / count) as u8;
                data[idx + 2] = (sum_b / count) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_blur_params_default() {
        let params = BlurParams::parse("");
        assert_eq!(params.kernel_size, 3);
        assert_eq!(params.iterations, 1);
        assert_eq!(params.radius(), 1);
    }

    #[test]
    fn test_blur_params_custom() {
        let params = BlurParams::parse("kernel_size=5,iterations=3");
        assert_eq!(params.kernel_size, 5);
        assert_eq!(params.iterations, 3);
        assert_eq!(params.radius(), 2);
    }

    #[test]
    fn test_blur_params_even_kernel() {
        // Чётное число должно стать нечётным
        let params = BlurParams::parse("kernel_size=4");
        assert_eq!(params.kernel_size, 5);
    }

    #[test]
    fn test_blur_params_small_kernel() {
        // Слишком маленькое число должно стать 3
        let params = BlurParams::parse("kernel_size=1");
        assert_eq!(params.kernel_size, 3);
    }

    #[test]
    fn test_blur_single_pixel() {
        let mut pixels = vec![100u8, 150u8, 200u8, 255u8];

            process_image(
                1,
                1,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        // Один пиксель не должен измениться
        assert_eq!(pixels[0], 100);
        assert_eq!(pixels[1], 150);
        assert_eq!(pixels[2], 200);
        assert_eq!(pixels[3], 255);
    }

    #[test]
    fn test_blur_uniform_image() {
        // Все пиксели одинаковые - размытие не должно изменить
        let mut pixels = vec![128u8, 128u8, 128u8, 255u8].repeat(9); // 3x3

            process_image(
                3,
                3,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        for i in 0..9 {
            assert_eq!(pixels[i * 4], 128);
            assert_eq!(pixels[i * 4 + 1], 128);
            assert_eq!(pixels[i * 4 + 2], 128);
        }
    }

    #[test]
    fn test_blur_gradient() {
        // Градиент: пиксели должны усредниться
        let mut pixels = vec![
            0u8, 0u8, 0u8, 255u8,
            128u8, 128u8, 128u8, 255u8,
            255u8, 255u8, 255u8, 255u8,
        ];

            process_image(
                1,
                3,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        // Средний пиксель должен остаться примерно таким же
        assert!(pixels[4] >= 100 && pixels[4] <= 150);
    }

    #[test]
    fn test_blur_zero_dimensions() {
        let mut pixels = vec![0u8, 0u8, 0u8, 255u8];

        // Не должно паниковать
            process_image(
                0,
                0,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        assert_eq!(pixels[0], 0);
    }
}