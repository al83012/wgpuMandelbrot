

#[repr(C)]
#[derive( bytemuck::Pod, bytemuck::Zeroable, Debug, Copy, Clone)]
pub struct CamValues {

    offset: [f32; 2],
    zoom: f32,
    ratio: f32, // width / height
}

impl Default for CamValues {
    fn default() -> Self {
        Self { zoom: 1.0, offset: [0.0, 0.0], ratio: 0.0 }
    }
}

impl CamValues {
    pub fn offset(&mut self, offset: [f32; 2]) {
        self.offset = [self.offset[0] + offset[0], self.offset[1] + offset[1]];
    }
    pub fn set_offset(&mut self, offset: [f32; 2]) {
        self.offset = offset;
    }
    pub fn set_ratio(&mut self, ratio: f32) {
        if ratio < 0.1 {
            return;
        }
        self.ratio = ratio;
    }
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn zoom(&mut self, zoom: f32) {
        self.zoom += zoom * 0.3;
        self.zoom = self.zoom.max(0.5);
    }
}