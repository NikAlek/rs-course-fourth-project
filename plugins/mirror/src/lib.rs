use std::os::raw::c_char;
use std::ffi::CStr;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MirrorDirection {
    // Отражение по вертикальной оси (лево | право)
    Horizontal,
    // Отражение по горизонтальной оси (верх | низ)
    Vertical,
    // Отражение по обеим осям
    Both,
}

impl Default for MirrorDirection {
    fn default() -> Self {
        Self::Horizontal
    }
}

impl MirrorDirection {
    /// Парсит направление из строки
    fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "vertical" | "v" | "flip_vertical" => Self::Vertical,
            "both" | "b" | "flip_both" => Self::Both,
            _ => Self::Horizontal,
        }
    }
}

struct MirrorParams {
    direction: MirrorDirection,
}

impl Default for MirrorParams {
    fn default() -> Self {
        Self {
            direction: MirrorDirection::default(),
        }
    }
}

impl MirrorParams {
    fn parse(params: &str) -> Self {
        let mut result = Self::default();

        for param in params.split([',', '\n', '\r']) {
            let parts: Vec<&str> = param.split('=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim();
                let value = parts[1].trim();

                if key == "direction" {
                    result.direction = MirrorDirection::from_str(value);
                }
            }
        }

        result
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

    println!("[mirror] Called with dimensions: {}x{}", width, height);
    println!("[mirror] Raw params: {}", params_str.replace('\n', " "));


    if width == 0 || height == 0 {
        eprintln!("[mirror] Error: Invalid dimensions");
        return;
    }

    let mirror_params = MirrorParams::parse(&params_str);

    unsafe {
        let total_bytes = match (width as usize)
            .checked_mul(height as usize)
            .and_then(|pixels| pixels.checked_mul(4))
        {
            Some(size) => size,
            None => {
                eprintln!("[mirror] Error: Buffer size overflow");
                return;
            }
        };

        let data = std::slice::from_raw_parts_mut(rgba_data, total_bytes);

        // Применяем зеркальное отражение
        apply_mirror(data, width as usize, height as usize, mirror_params.direction);
    }

    println!("[mirror] Processing complete");
}

unsafe fn apply_mirror(
    data: &mut [u8],
    width: usize,
    height: usize,
    direction: MirrorDirection,
) {
    // Создаём копию исходных данных для чтения
    let  original = data.to_vec();

    match direction {
        MirrorDirection::Horizontal => {
            println!("[mirror] Applying horizontal flip");
            for y in 0..height {
                for x in 0..(width / 2) {
                    let left_idx = (y * width + x) * 4;
                    let right_idx = (y * width + (width - 1 - x)) * 4;

                    // Меняем местами пиксели (RGBA)
                    for c in 0..4 {
                        data[left_idx + c] = original[right_idx + c];
                        data[right_idx + c] = original[left_idx + c];
                    }
                }
            }
        }

        MirrorDirection::Vertical => {
            println!("[mirror] Applying vertical flip");
            for x in 0..width {
                for y in 0..(height / 2) {
                    let top_idx = (y * width + x) * 4;
                    let bottom_idx = ((height - 1 - y) * width + x) * 4;

                    // Меняем местами пиксели (RGBA)
                    for c in 0..4 {
                        data[top_idx + c] = original[bottom_idx + c];
                        data[bottom_idx + c] = original[top_idx + c];
                    }
                }
            }
        }

        MirrorDirection::Both => {
            // Меняем пиксели местами по диагонали
            for y in 0..height {
                for x in 0..width {
                    let src_idx = (y * width + x) * 4;
                    let dst_idx = ((height - 1 - y) * width + (width - 1 - x)) * 4;

                    // Копируем пиксели (RGBA)
                    for c in 0..4 {
                        data[src_idx + c] = original[dst_idx + c];
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;


    #[test]
    fn test_mirror_horizontal_2x2() {
        // Исходное изображение 2x2:
        // [0, 1]
        // [2, 3]
        let mut pixels = vec![
            0u8, 0, 0, 255,   // (0,0)
            1u8, 1, 1, 255,   // (1,0)
            2u8, 2, 2, 255,   // (0,1)
            3u8, 3, 3, 255,   // (1,1)
        ];

            process_image(
                2,
                2,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        // После горизонтального отражения:
        // [1, 0]
        // [3, 2]
        assert_eq!(pixels[0], 1);   // (0,0) был (1,0)
        assert_eq!(pixels[4], 0);   // (1,0) был (0,0)
        assert_eq!(pixels[8], 3);   // (0,1) был (1,1)
        assert_eq!(pixels[12], 2);  // (1,1) был (0,1)
    }

    #[test]
    fn test_mirror_vertical_2x2() {
        let mut pixels = vec![
            0u8, 0, 0, 255,   // (0,0)
            1u8, 1, 1, 255,   // (1,0)
            2u8, 2, 2, 255,   // (0,1)
            3u8, 3, 3, 255,   // (1,1)
        ];

        let params = CString::new("direction=vertical").unwrap();

            process_image(
                2,
                2,
                pixels.as_mut_ptr(),
                params.as_ptr(),
            );

        // После вертикального отражения:
        // [2, 3]
        // [0, 1]
        assert_eq!(pixels[0], 2);   // (0,0) был (0,1)
        assert_eq!(pixels[4], 3);   // (1,0) был (1,1)
        assert_eq!(pixels[8], 0);   // (0,1) был (0,0)
        assert_eq!(pixels[12], 1);  // (1,1) был (1,0)
    }

    #[test]
    fn test_mirror_both_2x2() {
        let mut pixels = vec![
            0u8, 0, 0, 255,   // (0,0)
            1u8, 1, 1, 255,   // (1,0)
            2u8, 2, 2, 255,   // (0,1)
            3u8, 3, 3, 255,   // (1,1)
        ];

        let params = CString::new("direction=both").unwrap();

            process_image(
                2,
                2,
                pixels.as_mut_ptr(),
                params.as_ptr(),
            );

        // После отражения по обеим осям:
        // [3, 2]
        // [1, 0]
        assert_eq!(pixels[0], 3);   // (0,0) был (1,1)
        assert_eq!(pixels[4], 2);   // (1,0) был (0,1)
        assert_eq!(pixels[8], 1);   // (0,1) был (1,0)
        assert_eq!(pixels[12], 0);  // (1,1) был (0,0)
    }

    #[test]
    fn test_mirror_1x1_no_change() {
        let mut pixels = vec![100u8, 150u8, 200u8, 255u8];

            process_image(
                1,
                1,
                pixels.as_mut_ptr(),
                std::ptr::null(),
            );

        assert_eq!(pixels[0], 100);
        assert_eq!(pixels[1], 150);
        assert_eq!(pixels[2], 200);
        assert_eq!(pixels[3], 255);
    }



}