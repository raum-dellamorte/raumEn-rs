
use gfx;

pub use gfx_app::{ColorFormat, DepthFormat};
pub use util::rvertex::*;

gfx_defines!{
  vertex Vertex {
    pos: [f32; 3] = "a_Pos",
    // normal: [f32; 3] = "a_Norm",
    tex_coord: [f32; 2] = "a_TexCoord",
  }
  
  constant Locals {
    transform: [[f32; 4]; 4] = "u_Transform",
  }
  
  pipeline pipe {
    vbuf: gfx::VertexBuffer<Vertex> = (),
    tex: gfx::TextureSampler<[f32; 4]> = "t_Texture",
    transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
    locals: gfx::ConstantBuffer<Locals> = "Locals",
    out_color: gfx::RenderTarget<ColorFormat> = "Target0",
    out_depth: gfx::DepthTarget<DepthFormat> =
        gfx::preset::depth::LESS_EQUAL_WRITE,
  }
}

pub fn convertices(verts: &[RVertex]) -> Vec<Vertex> {
  let mut out = Vec::new();
  for vert in verts {
    out.push(convertex(vert));
  }
  out
}

pub fn convertex(vert: &RVertex) -> Vertex {
  Vertex {
    pos: vert.position.clone(),
    // normal: vert.normal.clone(),
    tex_coord: vert.tex_coords.clone(),
  }
}

// texture loading boilerplate
pub fn load_texture<R, F>(factory: &mut F, name: &str) -> Result<gfx::handle::ShaderResourceView<R, [f32; 4]>, String>
        where R: gfx::Resources, F: gfx::Factory<R> {
  use image;
  use std::path::Path;
  use gfx::format::Rgba8;
  use gfx::texture as t;
  let path: &str = &format!("src/res/img/{}.png", name);
  let img = match image::open(&Path::new(path)) {
    Ok(image) => {
      println!("Image loaded");
      image.to_rgba()
    },
    _ => panic!("Failed to load image")
  };
  let (width, height) = img.dimensions();
  let kind = t::Kind::D2(width as t::Size, height as t::Size, t::AaMode::Single);
  let (_, view) = match factory.create_texture_immutable_u8::<Rgba8>(kind, t::Mipmap::Provided, &[&img]) {
    Ok(tex_view) => {
      println!("Texture view created");
      tex_view
    }
    _ => panic!("Failed to create texture view.")
  };
  Ok(view)
}
