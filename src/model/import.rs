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

named!(get_f_chunk<&str, (usize,  usize,  usize) >,
  alt!(
    do_parse!(
      idx1: usize_digit >>
      tag!("/") >>
      idx2: usize_digit >>
      tag!("/") >>
      idx3: usize_digit >>
      ( idx1, idx2, idx3 )
    ) | do_parse!(
      idx1: usize_digit >>
      tag!("//") >>
      idx3: usize_digit >>
      ( idx1, 1, idx3 )
    )
  )
);

named!(get_f<&str, ( (usize,  usize,  usize), (usize, usize, usize), (usize, usize, usize) ) >,
  do_parse!(
    tag!("f") >>
    space >> v1: get_f_chunk >> space >> v2: get_f_chunk >> space >> v3: get_f_chunk >>
    ( v1, v2, v3 )
  )
);

pub fn test_nom() {
  {
    let v: String = "v -0.866025 0.000000 -0.500000".to_string();
    let (_, (x, y, z)) = get_v(&v).unwrap();
    println!("x: {}, y: {}, z: {}", x, y, z);
  }
  {
    let vt: String = "vt 0.523785 0.851270".to_string();
    let (_, (xt, yt)) = get_vt(&vt).unwrap();
    println!("x: {}, y: {}", xt, yt);
  }
  {
    let vn: String = "vn 0.499985 0.000000 0.866024".to_string();
    let (_, (xn, yn, zn)) = get_vn(&vn).unwrap();
    println!("x: {}, y: {}, z: {}", xn, yn, zn);
  }
  {
    let f: String = "f 183/1/1 6/2/1 12/3/1".to_string();
    let (_, (v1, v2, v3)) = get_f(&f).unwrap();
    println!("vertex 1: {} {} {}\nvertex 2: {} {} {}\nvertex 3: {} {} {}", v1.0, v1.1, v1.2, v2.0, v2.1, v2.2, v3.0, v3.1, v3.2);
  }
  {
    let f2: String = "f 183//1 6//1 12//1".to_string();
    let (_, (v4, v5, v6)) = get_f(&f2).unwrap();
    println!("vertex 1: {} {} {}\nvertex 2: {} {} {}\nvertex 3: {} {} {}", v4.0, v4.1, v4.2, v5.0, v5.1, v5.2, v6.0, v6.1, v6.2);
  }
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
        let (v1, v2, v3) = get_f(&l).unwrap().1;
        let index1 = v1.0 - 1;
        let index2 = v2.0 - 1;
        let index3 = v3.0 - 1;
        indcs.push(index1 as u16);
        indcs.push(index2 as u16);
        indcs.push(index3 as u16);
        {
          let vert1 = &mut verts[index1];
          if !vert1.is_set {
            vert1.tex_coords = txtrs[v1.1 - 1];
            vert1.normal = norms[v1.2 - 1];
            vert1.is_set = true;
          }
        }
        {
          let vert2 = &mut verts[index2];
          if !vert2.is_set {
            vert2.tex_coords = txtrs[v2.1 - 1];
            vert2.normal = norms[v2.2 - 1];
            vert2.is_set = true;
          }
        }
        {
          let vert3 = &mut verts[index3];
          if !vert3.is_set {
            vert3.tex_coords = txtrs[v3.1 - 1];
            vert3.normal = norms[v3.2 - 1];
            vert3.is_set = true;
          }
        }
      }
      _ => {}
    }
  }
  
  Ok( Mesh { verts: verts, indcs: indcs, far_point: 0_u16} )
}

//fn processVertex(verts: &mut Vec<Vertex>, index)
