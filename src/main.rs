mod audio_setup;
mod click_widget;
mod gui_canvas;
mod kp_sound;
mod metronome_core;
mod ui_widgets;
use clap::Parser;

use audio_setup::{host_device_setup, make_strem};
use cpal::traits::StreamTrait;
use gui_canvas::GUICanvas;

extern crate gl;

// use crate::ui_widgets::Point;
use crate::click_widget::Point;

extern crate sdl2;
use click_widget::ClickState;
use click_widget::ClickWidget;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels;
// use ui_widgets::ButtonUp;

pub mod render_gl;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

#[derive(Parser)]
#[clap(name = "Metronorust")]
#[clap(author = "syh.dev")]
#[clap(version = "0.1")]
#[clap(about = "A simple metronome", long_about = None)]
struct Cli {
    #[clap(short, long, default_value_t = 60)]
    bpm: usize,
    #[clap(short, long, default_value_t = 4)]
    time_signature: usize,
    #[clap(short, long, default_value_t = 1)]
    subdiv: usize,
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let (device, config, format) = host_device_setup();

    let stream = match make_strem(
        &device,
        &config,
        &format,
        cli.bpm,
        cli.time_signature,
        cli.subdiv,
    ) {
        Ok(stream) => stream,
        Err(e) => {
            println!("error {}", e);
            return Err("Stream not built".to_string());
        }
    };

    // stream.play().unwrap();
    // std::thread::sleep(std::time::Duration::from_millis(3000));

    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let gl_attr = video_subsys.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let gl_context = window.gl_create_context().unwrap();

    let gl = gl::load_with(|s| video_subsys.gl_get_proc_address(s) as *const std::os::raw::c_void);

    use std::ffi::CString;
    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    // set up vertex buffer object

    // let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let right: f32 = 0.5;
    let top = 0.5;
    let left: f32 = -0.5;
    let bottom: f32 = -0.5;
    let quad: Vec<f32> = vec![
        right, bottom, 0.0, right, top, 0.0, left, top, 0.0, left, bottom, 0.0,
    ];

    let mut vbo: gl::types::GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut vbo);
    }

    unsafe {
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                   // target
            (quad.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            quad.as_ptr() as *const gl::types::GLvoid,                          // pointer to data
            gl::STATIC_DRAW,                                                    // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // set up vertex array object

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
    }

    unsafe {
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader

        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 1)" in vertex shader
        let uniform_projectionMatrix =  glGetUniformLocation(mProgram, "projectionMatrix");
        float aspect = (float) mWidth / mHeight;
        glm::mat4 projectionM = glm::ortho(-aspect, aspect, -1.0f, 1.0f, -1.0f, 1.0f);
        // glUniformMatrix4fv(UNIFORM_projectionMatrix, 1, GL_FALSE, glm::value_ptr(projectionM));
        gl::UniformMatrix4fv(0, 1, 0, value);



        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    //let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    //canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
    //canvas.clear();

    // let mut btn1: ButtonUp = ButtonUp {
    //     top_left_corner: Point { x: 150, y: 150 },
    //     width: 50,
    //     height: 50,
    //     color: pixels::Color::RGB(255, 20, 20),
    // };
    // let mut btn2: ButtonUp = ButtonUp {
    //     top_left_corner: Point {
    //         x: 150 + 100,
    //         y: 150 + 100,
    //     },
    //     width: 50,
    //     height: 50,
    //     color: pixels::Color::RGB(255, 20, 20),
    // };
    // let mut btn3: ButtonUp = ButtonUp {
    //     top_left_corner: Point {
    //         x: 150 + 200,
    //         y: 150 + 200,
    //     },
    //     width: 50,
    //     height: 50,
    //     color: pixels::Color::RGB(255, 20, 20),
    // };

    let mut click1: ClickWidget = ClickWidget {
        center: Point { x: 300, y: 300 },
        radius: 50,
        color: pixels::Color::RGB(255, 20, 20),
        state: ClickState::Sound0,
    };

    let mut click2: ClickWidget = ClickWidget {
        center: Point { x: 100, y: 100 },
        radius: 50,
        color: pixels::Color::RGB(255, 20, 20),
        state: ClickState::Sound0,
    };

    let mut gui_canvas: GUICanvas = GUICanvas {
        click_widgets: vec![click1, click2],
    };

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    // btn1.render(&mut canvas);
    //btn2.render(&mut canvas);
    //btn3.render(&mut canvas);
    // click1.render(&mut canvas);
    //gui_canvas.render_canvas(&mut canvas);
    //canvas.present();

    let mut events = sdl_context.event_pump()?;

    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    }
                }

                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => match mouse_btn {
                    MouseButton::Left => {
                        // if btn1.is_mouse_inside(x, y) {
                        //     btn1.on_click(&mut canvas)
                        // }
                        gui_canvas.on_click(x, y);
                        //canvas.set_draw_color(pixels::Color::RGB(20, 20, 20));
                        //canvas.clear();
                        //gui_canvas.render_canvas(&mut canvas);
                        //canvas.present();
                    }
                    _ => {}
                },

                Event::MouseMotion { x, y, .. } => {
                    // if btn1.is_mouse_inside(x, y) {
                    //     btn1.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn1.mouse_is_not_over(&mut canvas);
                    // }
                    // if btn2.is_mouse_inside(x, y) {
                    //     btn2.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn2.mouse_is_not_over(&mut canvas);
                    // }
                    // if btn3.is_mouse_inside(x, y) {
                    //     btn3.mouse_is_over(&mut canvas);
                    // } else {
                    //     btn3.mouse_is_not_over(&mut canvas);
                    // }
                }

                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLE_FAN, // mode
                0,                // starting index in the enabled arrays
                4,                // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }

    Ok(())
}
