struct TransitionUniform {
    progress: f32,
    type_id: u32,
    dir_x: f32,
    dir_y: f32,
};

@group(0) @binding(0) var t_old: texture_2d<f32>;
@group(0) @binding(1) var s_old: sampler;
@group(0) @binding(2) var t_new: texture_2d<f32>;
@group(0) @binding(3) var s_new: sampler;
@group(0) @binding(4) var<uniform> params: TransitionUniform;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32((in_vertex_index & 1u) << 1u);
    let y = f32((in_vertex_index & 2u));
    out.uv = vec2<f32>(x * 0.5, 1.0 - y * 0.5);
    out.position = vec4<f32>(x * 2.0 - 1.0, y * 2.0 - 1.0, 0.0, 1.0);
    return out;
}

fn hash(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let old_color = textureSample(t_old, s_old, in.uv);
    let new_color = textureSample(t_new, s_new, in.uv);
    let t = params.progress;

    if (params.type_id == 0u) {
        return new_color;
    }
=
    if (params.type_id == 1u) {
        return mix(old_color, new_color, t);
    }

    if (params.type_id == 2u) {
        if (t < 0.5) {
            return mix(old_color, vec4<f32>(0.0, 0.0, 0.0, 1.0), t * 2.0);
        } else {
            return mix(vec4<f32>(0.0, 0.0, 0.0, 1.0), new_color, (t - 0.5) * 2.0);
        }
    }
    
    if (params.type_id == 3u) {
        let noise = hash(floor(in.uv * 100.0)); 
        if (noise < t) {
            return new_color;
        } else {
            return old_color;
        }
    }

    if (params.type_id == 4u) {
        let p = t;

        var val = in.uv.x;
        if (params.dir_x < 0.0) { val = 1.0 - in.uv.x; }
        if (params.dir_y > 0.0) { val = in.uv.y; }
        if (params.dir_y < 0.0) { val = 1.0 - in.uv.y; }

        if (val < t) {
            return new_color;
        } else {
            return old_color;
        }
    }

    return new_color;
}