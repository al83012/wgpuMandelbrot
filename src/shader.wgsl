
// Define a uniform buffer for time
struct Time {
    time: f32
};

struct CameraInfo {


    offset: vec2<f32>,
    zoom: f32,
    ratio: f32,
}

@group(0) @binding(0) var<uniform> time_uf: Time;
@group(1) @binding(0) var<uniform> cam: CameraInfo;

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
    //For pulsating
    //let it: i32 = i32(round(max((sin(time_uf.time) + 1) * 0.5 / 5, 0.001) * f32(ITERATIONS)));
    let it = ITERATIONS;
    let c: vec2<f32> = transform(in.coord);
    var x: f32 = 0.;
    var y: f32 = 0.;
    var i: i32 = 0;

    for (; i < it; i = i + 1) {
        if (x*x + y*y > 4  ) {
            break;
        }
        let xtemp: f32 = (x * x) - (y * y) + c.x;
        y = 2. * x * y + c.y;
        x = xtemp;
    }

    let frac: f32 = f32(i) / f32(it);

    return vec4<f32>(frac * 5., frac * 1., frac * 3., 1.0);
    //return vec4<f32>(in.coord, 0.0, 0.0);
    //return vec4<f32>(cam.offset.x / 10.0, cam.offset.y / 10.0, in.coord.x / 10.0, cam.zoom / 10.0);
}

const pi = radians(180.0);

fn transform(in: vec2<f32>) -> vec2<f32> {
let offset_strength : vec2<f32> = vec2<f32>(0.01, 0.01);
    var out: vec2<f32>;
    out = in;

    out.x = out.x * cam.ratio;
    out = out * vec2<f32>(1/exp(cam.zoom), 1/exp(cam.zoom));
    out = out + vec2<f32>(
    -0.5 + cam.offset.x * offset_strength.x,
    0 + cam.offset.y * offset_strength.y,
     );

     // For time- animation
     /*let t = time_uf.time;
out.x = out.x * cam.ratio;
let angle = (1 / (t + 2)) * 2 * pi;
let sine = sin(angle);
let cosine = cos(angle);
out = vec2<f32>(
    out.x * cosine - out.y * sine,
    out.x * sine + out.y * cosine
);
out  = out + vec2<f32>(
          0.5, 0.0
          );
     let scale : f32 = 1 /exp(0.5 * t);
     out = out * vec2<f32>(scale, scale);
     out  = out + vec2<f32>(
               -1.3999, 0.001
               );


*/


    return out;
}