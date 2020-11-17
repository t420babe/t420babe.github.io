mod log;
use log::log;

use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
  web_sys::console::log_1(&"Hi".into());
  log("yes created");
  let context = get_rendering_context()?;

  let vertex_shader_source = r#"
    void main() {
      gl_Position = vec4(0.0, 0.0, 0.0, 1.0);
      gl_PointSize = 64.0;
    }
  "#;

  let fragment_shader_source = r#"
    void main(void) {
      gl_FragColor = vec4(0.5, 0.1, 0.8, 1.0);
    }
  "#;
  let vertex_shader =
    compile_shader(&context, WebGlRenderingContext::VERTEX_SHADER, &vertex_shader_source)?;
  let fragment_shader =
    compile_shader(&context, WebGlRenderingContext::FRAGMENT_SHADER, &fragment_shader_source)?;

  let program =
    context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;
  context.attach_shader(&program, &vertex_shader);
  context.attach_shader(&program, &fragment_shader);
  context.link_program(&program);

  context.enable_vertex_attrib_array(0);
  let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
  context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
  context.vertex_attrib_pointer_with_i32(0, 1, WebGlRenderingContext::FLOAT, false, 0, 0);
  context.enable_vertex_attrib_array(0);
  context.use_program(Some(&program));
  context.draw_arrays(WebGlRenderingContext::POINTS, 0, 1);

  Ok(())
}

pub fn compile_shader(
  context: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = context
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  context.shader_source(&shader, source);
  context.compile_shader(&shader);

  if context
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      context
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

pub fn link_program(
  context: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program =
    context.create_program().ok_or_else(|| String::from("Unable to create shader object"))?;

  context.attach_shader(&program, vert_shader);
  context.attach_shader(&program, frag_shader);
  context.link_program(&program);

  if context
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      context
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}

fn get_rendering_context() -> Result<WebGlRenderingContext, JsValue> {
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id("canvas").unwrap();
  let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
  canvas.set_width(canvas.client_width() as u32);
  canvas.set_height(canvas.client_height() as u32);

  let context =
    canvas.get_context("experimental-webgl")?.unwrap().dyn_into::<WebGlRenderingContext>()?;
  context.viewport(0, 0, context.drawing_buffer_width(), context.drawing_buffer_height());
  context.enable(WebGlRenderingContext::DEPTH_TEST);
  context.clear_color(1.0, 1.0, 0.0, 1.0);
  context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

  Ok(context)
}
