/// Workgroup size for `compute.wsgl#mandelbrot`.
pub const MANDELBROT_WORKGROUP_SIZE_Y: u32 = 64;

/// Corresponds to `compute.wsgl#MANDELBROT_DISPATCH_SIZE_Y`.
pub const MANDELBROT_DISPATCH_SIZE_Y: u32 = 1024;

pub fn mandelbrot_dispatch_size(total_work: usize) -> (u32, u32, u32) {
    let x = (total_work / (MANDELBROT_DISPATCH_SIZE_Y * MANDELBROT_WORKGROUP_SIZE_Y) as usize + 1)
        .try_into()
        .unwrap();
    (x, MANDELBROT_DISPATCH_SIZE_Y, 1)
}