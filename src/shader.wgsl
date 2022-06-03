// Vertex shader

// struct InstanceInput {
//     [[location(5)]] model_matrix_0: vec4<f32>;
//     [[location(6)]] model_matrix_1: vec4<f32>;
//     [[location(7)]] model_matrix_2: vec4<f32>;
//     [[location(8)]] model_matrix_3: vec4<f32>;
// };

struct SoundUniform {
    data: array<f32,16>;
};

[[group(1), binding(0)]]
var<uniform> sound: SoundUniform;

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


// -----------------------------------------------
// Convert r, g, b to normalized vec3
// -----------------------------------------------
fn rgb(r: f32, g: f32, b: f32) -> vec3<f32> {
    return vec3<f32>(r / 255.0, g / 255.0, b / 255.0);
}

// -----------------------------------------------
//  Draw a circle at vec2 `pos` with radius `rad` and
//  color `color`.
//  -----------------------------------------------
fn circle(uv: vec2<f32>, pos: vec2<f32>, rad: f32, color: vec3<f32>) -> vec4<f32> {
    let d = length(pos - uv) - rad;
    let t = clamp(d, 0.0, 1.0);
    return vec4<f32>(color, 1.0 - t);
}

// float circle(vec2 st, vec2 center, float radius) {
//     return smoothstep(1., 1.-0.025, distance(st, center) / radius);
// }

fn ring(st: vec2<f32>, center: vec2<f32>, radius: f32, color: vec3<f32>) -> vec4<f32> {
    return circle(st, center, radius, color) - circle(st, center, radius - 0.020, color);
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
//==========================================================================================

    // let st = in.clip_position;
    // let center = vec2<f32>(camera.resolution.x / 2.0, camera.resolution.y / 2.0);
    // let pct = sqrt((st.x - center.x) * (st.x - center.x) + (st.y - center.y) * (st.y - center.y));
    // let color = vec3<f32>(pct / 800.0);
    
    
    // let resolution = vec2<f32>(camera.resolution.x, camera.resolution.y);

    // let color = circle(in.clip_position.xy / resolution, vec2<f32>(200.0, 200.0) / resolution, 0.1);

    // return vec4<f32>(vec3<f32>(color), 1.0);
//==========================================================================================


    // let two_pi = 2.0 * 3.14159265359;

    // let toto: vec4<f32> = textureLoad(t_sound, vec2<i32>(0, 0), 0);
    // // let tata: vec4<f32> = textureLoad(t_sound, vec2<i32>(1, 0), 0);

    // let resolution = vec2<f32>(camera.resolution.x, camera.resolution.y);

    // let uv = in.clip_position.xy;

    // let screen_center = resolution.xy * 0.5;

    // let radius = 30.0;
    // let metr_radius = 100.0;

    // let nb = 5;

    // // Background layer
    // let grey = 20.0;
    // let background_layer = vec4<f32>(rgb(grey, grey, grey), 1.0);
	
	// // Circle
    // let blue = rgb(20.0, 0.0, 150.0);
    // let angle = two_pi / f32(nb);
    // let offset = two_pi / 4.0;
    // var center = vec2<f32>(cos(offset - angle * 0.0), -1.0 * sin(offset - angle * 0.0)) * metr_radius + screen_center;
    // let main_circle_layer = circle(uv, center, radius, blue);
    
	
	// // Blend the two
    // var f = mix(background_layer, main_circle_layer, main_circle_layer.a);

    // var i:  i32 = 1;
    // loop {
    //     if (i >= nb) { 
    //         break;
    //     }

    //     center = vec2<f32>(cos(offset - angle * f32(i)), -1.0 * sin(offset - angle * f32(i))) * metr_radius + screen_center;

    //     let circle_layer = circle(uv, center, radius, blue);
    //     f = mix(f, circle_layer, circle_layer.a) ;
    //     i = i + 1;
    // }

    // return f + toto;

    // Normalized pixel coordinates (from 0 to 1)
    let resolution = vec2<f32>(camera.resolution.x, camera.resolution.y);
    let uv = in.clip_position.xy / resolution;

    // float data = texture(iChannel0, vec2(uv.x, 0.0)).x;
    // let data = (textureLoad(t_sound, vec2<i32>(0, 0), 0).y + textureLoad(t_sound, vec2<i32>(0, 0), 0).x) / 2.0 * 100.0;
    let data = sound.data[0];

    let y = mix(0.05, .95, (data + 1.0) / 2.0) - 0.5;// from [-1;1] to [0;1]

    var col = vec3<f32>(21., 20., 36.) / 255.;

    if (y >= 0.5) {
        col = vec3<f32>(0.0, 255.0, 0.0) / 255.0;
    }


    let st = in.clip_position.xy - resolution / 2.0;

    if (length(st) / 1000.0 <= 0.095) {
        if (length(st) / 1000.0 >= 0.090) {

            col = vec3<f32>(14., 15., 230.) / 255.0;
        }
    }


    if (length(st) / 1000.0 <= 0.1 + y / 2.0) {
        if (length(st) / 1000.0 >= 0.095) {

            col = vec3<f32>(20., 136., 138.) / 255.0;
        }
    }
       
    
    // Output to screen
    return vec4<f32>(col, 1.0);
}