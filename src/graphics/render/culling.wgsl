struct FrustumPlanes {
    left: vec4<f32>,
    right: vec4<f32>,
    bottom: vec4<f32>,
    top: vec4<f32>,
    near: vec4<f32>,
    far: vec4<f32>,
};

@group(0) @binding(0) var<uniform> frustum: FrustumPlanes;

struct SpriteInstance {
    model_matrix_0: vec4<f32>,
    model_matrix_1: vec4<f32>,
    model_matrix_2: vec4<f32>,
    model_matrix_3: vec4<f32>,
    uv_rect: vec4<f32>,
    color: vec4<f32>,
};

@group(0) @binding(1) var<storage, read> all_instances: array<SpriteInstance>;
@group(0) @binding(2) var<storage, read_write> visible_instances: array<SpriteInstance>;

struct IndirectDrawArgs {
    index_count: u32,
    instance_count: atomic<u32>,
    first_index: u32,
    base_vertex: i32,
    first_instance: u32,
};
@group(0) @binding(3) var<storage, read_write> draw_args: IndirectDrawArgs;

fn is_aabb_inside_frustum(center: vec3<f32>, extents: vec3<f32>) -> bool {
    let planes = array<vec4<f32>, 6>(
        frustum.left, frustum.right,
        frustum.bottom, frustum.top,
        frustum.near, frustum.far
    );

    for (var i = 0u; i < 6u; i = i + 1u) {
        let plane = planes[i];

        let r = extents.x * abs(plane.x) + extents.y * abs(plane.y) + extents.z * abs(plane.z);

        let d = dot(plane.xyz, center) + plane.w;

        if (d < -r) {
            return false;
        }
    }
    return true;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    let total_instances = arrayLength(&all_instances);

    if (index >= total_instances) {
        return;
    }

    let instance = all_instances[index];

    let center = vec3<f32>(instance.model_matrix_3.x, instance.model_matrix_3.y, instance.model_matrix_3.z);

    let scale_x = length(instance.model_matrix_0.xyz);
    let scale_y = length(instance.model_matrix_1.xyz);
    let extents = vec3<f32>(scale_x * 0.5, scale_y * 0.5, 0.01);

    if (is_aabb_inside_frustum(center, extents)) {
        let write_index = atomicAdd(&draw_args.instance_count, 1u);

        visible_instances[write_index] = instance;
    }
}