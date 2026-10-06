struct Camera {
    view: mat4x4<f32>,
    proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: Camera;

// Light Bind Group @group(2) -> will also need to increment bind groups

struct Material {
    base_color: vec4<f32>,
    metallic_roughness: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: Material;
@group(1) @binding(1) var base_color_texture: texture_2d<f32>;
@group(1) @binding(2) var normal_texture: texture_2d<f32>;
@group(1) @binding(3) var metallic_roughness_texture: texture_2d<f32>;
@group(1) @binding(4) var occlusion_texture: texture_2d<f32>;
@group(1) @binding(5) var material_sampler: sampler;

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
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) normal: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    
    let model_matrix = mat4x4<f32>(
        input.model_0,
        input.model_1,
        input.model_2,
        input.model_3,
    );

    out.uv = input.uv;
    
    out.normal = (model_matrix * vec4<f32>(input.normal, 0.0)).xyz; 
    
    let world_position = model_matrix * vec4<f32>(input.position, 1.0);
    out.clip_position = camera.proj * camera.view * world_position;
    
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let sampled_color = textureSample(base_color_texture, material_sampler, input.uv);
    let final_color = sampled_color * material.base_color;
    return final_color;
}
