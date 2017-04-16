use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std;

use util::rvertex::*;
use model::mesh::Mesh;

use nom::{space, float_s, digit}; // IResult, alpha, alphanumeric,

named!(usize_digit<&str, usize >,
    map_res!(
        digit,
        std::str::FromStr::from_str
    )
);

named!(get_v<&str, (f32, f32, f32) >,
  do_parse!(
    tag!("v") >>
    space >>
    x: float_s >>
    space >>
    y: float_s >>
    space >>
    z: float_s >>
    (x, y, z)
  )
);

named!(get_vt<&str, (f32, f32) >,
  do_parse!(
    tag!("vt") >>
    space >>
    x: float_s >>
    space >>
    y: float_s >>
    (x, y)
  )
);

named!(get_vn<&str, (f32, f32, f32) >,
  do_parse!(
    tag!("vn") >>
    space >>
    x: float_s >>
    space >>
    y: float_s >>
    space >>
    z: float_s >>
    (x, y, z)
  )
);

named!(get_f<&str, ( usize,  usize,  usize, usize, usize, usize, usize, usize, usize) >,
  do_parse!(
    tag!("f") >>
    space >>
    idx1: usize_digit >> tag!("/") >> idx4: usize_digit >> tag!("/") >> idx5: usize_digit >>
    space >>
    idx2: usize_digit >> tag!("/") >> idx6: usize_digit >> tag!("/") >> idx7: usize_digit >>
    space >>
    idx3: usize_digit >> tag!("/") >> idx8: usize_digit >> tag!("/") >> idx9: usize_digit >>
    (idx1, idx2, idx3, idx4, idx5, idx6, idx7, idx8, idx9)
  )
);

pub fn test_nom() {
  let v: String = "v -0.866025 0.000000 -0.500000".to_string();
  let vt: String = "vt 0.523785 0.851270".to_string();
  let vn: String = "vn 0.499985 0.000000 0.866024".to_string();
  let f: String = "f 183/1/1 6/2/1 12/3/1".to_string();
  
  let (_, (x, y, z)) = get_v(&v).unwrap();
  let (_, (xt, yt)) = get_vt(&vt).unwrap();
  let (_, (xn, yn, zn)) = get_vn(&vn).unwrap();
  let (_, (idx1, idx2, idx3, idx4, idx5, idx6, idx7, idx8, idx9)) = get_f(&f).unwrap();
  println!("x: {}, y: {}, z: {}", x, y, z);
  println!("x: {}, y: {}", xt, yt);
  println!("x: {}, y: {}, z: {}", xn, yn, zn);
  println!("indices: {} {} {} {} {} {} {} {} {}", idx1, idx2, idx3, idx4, idx5, idx6, idx7, idx8, idx9);
}

pub fn load_obj(objname: &str) -> Result<Mesh, &str> {
  let filename = format!("./res/obj/{}.obj", objname);
  let path = Path::new(&filename);
  let display = path.display();
  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };
  let reader = BufReader::new(file);
  
  let mut verts: Vec<Vertex> = Vec::new();
  let mut txtrs: Vec<(f32, f32)> = Vec::new();
  let mut norms: Vec<(f32, f32, f32)> = Vec::new();
  let mut indcs: Vec<u16> = Vec::new();
  for line in reader.lines() {
    match &(line.unwrap()) {
      l if &l[..2] == "v " => {
        let vert = &mut Vertex::new();
        vert.position = get_v(&l).unwrap().1;
        verts.push(*vert);
      }
      l if &l[..3] == "vt " => { txtrs.push(get_vt(&l).unwrap().1); }
      l if &l[..3] == "vn " => { norms.push(get_vn(&l).unwrap().1); }
      l if &l[..2] == "f " => {
        let indices = get_f(&l).unwrap().1;
        let index1 = indices.0 - 1;
        let index2 = indices.1 - 1;
        let index3 = indices.2 - 1;
        indcs.push(index1 as u16);
        indcs.push(index2 as u16);
        indcs.push(index3 as u16);
        {
          let v1 = &mut verts[index1];
          if !v1.is_set {
            v1.tex_coords = txtrs[indices.3 - 1];
            v1.normal = norms[indices.4 - 1];
            v1.is_set = true;
          }
        }
        {
          let v2 = &mut verts[index2];
          if !v2.is_set {
            v2.tex_coords = txtrs[indices.5 - 1];
            v2.normal = norms[indices.6 - 1];
            v2.is_set = true;
          }
        }
        {
          let v3 = &mut verts[index3];
          if !v3.is_set {
            v3.tex_coords = txtrs[indices.7 - 1];
            v3.normal = norms[indices.8 - 1];
            v3.is_set = true;
          }
        }
      }
      _ => {}
    }
  }
  
  Ok( Mesh { verts: verts, indcs: indcs, far_point: 0_u16} )
}
