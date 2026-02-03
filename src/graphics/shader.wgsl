// WGSL Shader for Instanced Sprite Rendering
// Supports hardware instancing with texture atlas/sprite sheet UVs

// Camera uniform
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

// Texture and sampler
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(1) @binding(1)
var s_diffuse: sampler;

// Vertex input (shared quad geometry)
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

// Instance input (per-sprite data)
struct InstanceInput {
    // Model matrix (4x vec4 = mat4x4)
    @location(2) model_matrix_0: vec4<f32>,
    @location(3) model_matrix_1: vec4<f32>,
    @location(4) model_matrix_2: vec4<f32>,
    @location(5) model_matrix_3: vec4<f32>,

    // UV rectangle (x, y, width, height)
    @location(6) uv_rect: vec4<f32>,

    // Color tint
    @location(7) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;

    // Reconstruct model matrix from instance data
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    // Transform vertex position
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(vertex.position, 1.0);

    // Apply UV rectangle transformation
    // vertex.tex_coords is [0, 1] quad UVs
    // uv_rect is (x, y, width, height) in texture space
    out.tex_coords = instance.uv_rect.xy + vertex.tex_coords * instance.uv_rect.zw;

    // Pass color to fragment shader
    out.color = instance.color;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(t_diffuse, s_diffuse, in.tex_coords);

    // Apply color tint
    return tex_color * in.color;
}
