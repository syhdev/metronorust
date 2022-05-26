// Vertex shader

// struct InstanceInput {
//     [[location(5)]] model_matrix_0: vec4<f32>;
//     [[location(6)]] model_matrix_1: vec4<f32>;
//     [[location(7)]] model_matrix_2: vec4<f32>;
//     [[location(8)]] model_matrix_3: vec4<f32>;
// };

struct CameraUniform {
    view_proj: mat4x4<f32>;
    resolution: vec2<f32>;
};

[[group(0), binding(0)]] // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
    //instance: InstanceInput,
) -> VertexOutput {
    // let model_matrix = mat4x4<f32>(
    //     instance.model_matrix_0,
    //     instance.model_matrix_1,
    //     instance.model_matrix_2,
    //     instance.model_matrix_3,
    // );
    var out: VertexOutput;
    out.color = model.color;
    // out.clip_position = camera.view_proj* model_matrix *vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    //out.clip_position = vec4<f32>(model.position, 1.0);


    return out;
}

// Fragment shader


[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let st = in.clip_position;
    let center = vec2<f32>(camera.resolution.x / 2.0, camera.resolution.y / 2.0);
    // let center = vec2<f32>(100.0, 300.0);
    let pct = sqrt((st.x - center.x) * (st.x - center.x) + (st.y - center.y) * (st.y - center.y));
    let color = vec3<f32>(pct / 800.0);
    // var color: vec3<f32> = vec3<f32>(0.9, 0.9, 0.0);
    // if (in.clip_position.x > 100.0) {
    //     color = vec3<f32>(0.0, 0.9, 0.9);
    // } else {
    //     color = vec3<f32>(0.0, 0.0, 0.0);
    // }
    return vec4<f32>(color, 1.0);
}

