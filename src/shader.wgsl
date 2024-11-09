
// Define a uniform buffer for time
struct Time {
    time: f32
};

struct CameraInfo {
    zoom: f32,
    offset: vec2<f32>,
}

@group(0) @binding(0) var<uniform> time_uf: Time;
//@group(0) @binding(1) var<uniform> cam: CameraInfo;

@vertex
fn vs_main(@builtin(vertex_index) index : u32) -> VertexOutput {
  var out: VertexOutput;
  var ul = vec2<f32>(-1.0, 1.0);
  var ur = vec2<f32>(1.0, 1.0);
  var bl = vec2<f32>(-1.0, -1.0);
  var br = vec2<f32>(1.0, -1.0);
  var verticies = array<vec2<f32>, 6>(
    ul, bl, ur, bl, br, ur
  );
  out.position = vec4<f32>(verticies[index], 0.0, 1.0);
  out.coord = vec2<f32>(verticies[index]);
  return out;
}

// This is a work in progress.

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

const ITERATIONS: i32 = 1000;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let c: vec2<f32> = (in.coord + vec2<f32>(-0.5, 0.)) * (1.3 / time_uf.time);
    var x: f32 = 0.;
    var y: f32 = 0.;
    var i: i32 = 0;

    for (; i < ITERATIONS; i = i + 1) {
        if (x*x + y*y > 4.) {
            break;
        }
        let xtemp: f32 = (x * x) - (y * y) + c.x;
        y = 2. * x * y + c.y;
        x = xtemp;
    }

    let frac: f32 = f32(i) / f32(ITERATIONS);
    return vec4<f32>(frac * 5., frac * 1., frac * 3., 1.0);
    //return vec4<f32>(in.coord, 0.0, 0.0);
}
