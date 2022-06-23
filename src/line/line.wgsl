struct InstanceInput {
    [[location(1)]] position1: vec2<f32>;
    [[location(1)]] position2: vec2<f32>;
    [[location(3)]] color: vec4<f32>;
    [[location(4)]] thiccness: f32;
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
    [[builtin(position)]] clip_position1: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] thiccness: f32;
    [[location(2)]] position1: vec2<f32>;
    [[location(3)]] position2: vec2<f32>;
    [[location(4)]] index: u32;
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
    out.clip_position = camera.view_proj * instance.position * vec4<f32>(model.v_position, 0.0, 1.0);
    out.color = instance.color;
    out.thiccness = instance.thiccness;
    out.position = model.v_position;
    out.index = model.index;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}

