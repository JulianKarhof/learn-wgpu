struct InstanceInput {
    [[location(1)]] position1: vec2<f32>;
    [[location(2)]] position2: vec2<f32>;
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
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] thiccness: f32;
    [[location(2)]] line_position1: vec2<f32>;
    [[location(3)]] line_position2: vec2<f32>;
    [[location(4)]] index: u32;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;

    let line_vector: vec2<f32> = vec2<f32>(
        instance.position2.x - instance.position1.y,
        instance.position2.x - instance.position1.y
    );

    var normal_vector: vec2<f32>;
    if (i32(model.index) < 2) {
        normal_vector = normalize(vec2<f32>(-line_vector.y, line_vector.x));
    } else {
        normal_vector = normalize(vec2<f32>(line_vector.y, -line_vector.x));
    }

    let delta: vec4<f32> = vec4<f32>(normal_vector * instance.thiccness, 0.0, 0.0);
    // TODO: model.v_position is not at the instance position?
    out.clip_position = camera.view_proj * (vec4<f32>(model.v_position, 0.0, 1.0) + delta);
    out.color = instance.color;
    out.thiccness = instance.thiccness;
    out.line_position1 = instance.position1;
    out.line_position2 = instance.position2;
    out.index = model.index;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}

