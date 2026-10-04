struct Camera {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,

    @location(3) model_0: vec4<f32>,
    @location(4) model_1: vec4<f32>,
    @location(5) model_2: vec4<f32>,
    @location(6) model_3: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var diffuse_texture: texture_2d<f32>;

@group(1) @binding(1)
var diffuse_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let model = mat4x4<f32>(
        input.model_0,
        input.model_1,
        input.model_2,
        input.model_3,
    );

    let world_position = model * vec4<f32>(input.position, 1.0);

    var output: VertexOutput;
    output.position = camera.proj * camera.view * world_position;
    output.uv = input.uv;

    output.normal = normalize((model * vec4<f32>(input.normal, 0.0)).xyz);

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(
        diffuse_texture,
        diffuse_sampler,
        input.uv,
    );

    return color;
}

