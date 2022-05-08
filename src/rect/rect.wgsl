struct InstanceInput {
    [[location(1)]] position: vec2<f32>;
    [[location(2)]] rotation: f32;
    [[location(3)]] color: vec4<f32>;
    [[location(4)]] size: vec2<f32>;
    [[location(5)]] border_radius: vec4<f32>;
    [[location(6)]] border: f32;
    [[location(7)]] border_color: vec4<f32>;
};

struct CameraUniform {
    view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] v_position: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] position: vec2<f32>;
    [[location(6)]] border: f32;
    [[location(7)]] border_color: vec4<f32>;
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
    out.border = instance.border;
    out.border_color = instance.border_color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color = in.color;
    let aspect = in.size.x / in.size.y; 

    let maxX = 1.0 - in.border;
    let minX = -1.0 + in.border;
    let minY = -1.0 + in.border * aspect;
    let maxY = 1.0 - in.border * aspect;

    if (in.position.x > maxX || in.position.x < minX || in.position.y < minY || in.position.y > maxY) { color = in.border_color; }

    return vec4<f32>(color.xyzw);
}
 
