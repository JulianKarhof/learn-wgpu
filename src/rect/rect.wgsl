struct InstanceInput {
    [[location(1)]] position: vec2<f32>;
    [[location(2)]] rotation: f32;
    [[location(3)]] color: vec4<f32>;
    [[location(4)]] size: vec2<f32>;
    [[location(5)]] border_radius: vec4<f32>;
    [[location(6)]] border_width: f32;
    [[location(7)]] border_color: vec4<f32>;
};

struct CameraUniform {
    view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] v_position: vec2<f32>;
    [[builtin(vertex_index)]] index: u32;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] position: vec2<f32>;
    [[location(3)]] border_width: f32;
    [[location(4)]] border_color: vec4<f32>;
    [[location(5)]] border_radius: vec4<f32>;
    [[location(6)]] index: u32;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    let transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(instance.size.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, instance.size.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(instance.position, 0.0, 1.0),
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * transform * vec4<f32>(model.v_position, 0.0, 1.0);
    out.color = instance.color;
    out.size = instance.size;
    out.position = model.v_position;
    out.border_radius = instance.border_radius;
    out.border_width = instance.border_width;
    out.border_color = instance.border_color;
    out.index = model.index;
    return out;
}

fn distance_alg(
    frag_coord: vec2<f32>,
    position: vec2<f32>,
    size: vec2<f32>,
    radius: f32
) -> f32 {
    var inner_size: vec2<f32> = size - vec2<f32>(radius, radius) * 2.0;
    var top_left: vec2<f32> = position + vec2<f32>(radius, radius);
    var bottom_right: vec2<f32> = top_left + inner_size - 2.0;

    var top_left_distance: vec2<f32> = top_left - frag_coord;
    var bottom_right_distance: vec2<f32> = frag_coord - bottom_right;

    var dist: vec2<f32> = vec2<f32>(
        max(max(top_left_distance.x, bottom_right_distance.x), 0.0),
        max(max(top_left_distance.y, bottom_right_distance.y), 0.0)
    );

    return sqrt(dist.x * dist.x + dist.y * dist.y);
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color = in.color;
    let aspect = in.size.x / in.size.y; 

    let maxX = 1.0 - in.border_width;
    let minX = -1.0 + in.border_width;
    let minY = -1.0 + in.border_width * aspect;
    let maxY = 1.0 - in.border_width * aspect;

    if (in.position.x > maxX || in.position.x < minX || in.position.y < minY || in.position.y > maxY) { color = in.border_color; }
    
    let norm_pos = (in.position + 1.0) / 2.0;
    let border_radius = in.border_radius[in.index];
    let coords = norm_pos * in.size;

    var dist: f32 = distance_alg(
        coords,
        norm_pos,
        in.size,
        border_radius,
    );

    var radius_alpha: f32 = 1.0 - smoothStep(
        max(border_radius - 0.2, 0.0),
        border_radius + 0.2,
        dist
    );

    return vec4<f32>(color.xyz, color.w * radius_alpha);
}
 
