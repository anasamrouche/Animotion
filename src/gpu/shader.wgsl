struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct TimeUniform {
    time: f32,
    fps: u32,
};

@group(1) @binding(0)
var<uniform> time: TimeUniform;

struct VertexInput {
    @location(0) position : vec4f,
    @location(1) color : vec4f,
};

struct VertexOutput {
    @builtin(position) clip_position : vec4f,
    @location(0) color : vec4f,
    @location(1) uv : vec2f,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = camera.view_proj * (in.position + vec4f(sin(time.time) * 0.25, 0.0, 0.0, 0.0));
    out.color = in.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4f {
    return in.color;
}
