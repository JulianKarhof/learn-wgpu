struct InstanceInput {
    [[location(1)]] position: vec2<f32>;
    [[location(2)]] rotation: f32;
    [[location(3)]] color: vec4<f32>;
    [[location(4)]] size: vec2<f32>;
    [[location(5)]] border_radius: vec4<f32>;
};

struct CameraUniform {
    view_proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] v_position: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
    [[location(1)]] local: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    instance: InstanceInput
) -> VertexOutput {
    var transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(instance.size.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, instance.size.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(instance.position, 0.0, 1.0),
    );

    var out: VertexOutput;
    out.clip_position = camera.view_proj * transform * vec4<f32>(model.v_position, 1.0);
    out.color = instance.color;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color.xyz, in.color.w);
}
 
