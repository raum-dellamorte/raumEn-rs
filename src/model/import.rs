use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std;

use util::rvertex::*;
use model::mesh::Mesh;

use nom::{IResult, space, alpha, alphanumeric, float_s, digit};

named!(u16_digit<&str, u16 >,
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

named!(get_f<&str, (u16, u16, u16, u16, u16, u16, u16, u16, u16) >,
  do_parse!(
    tag!("f") >>
    space >>
    idx1: u16_digit >> tag!("/") >> idx2: u16_digit >> tag!("/") >> idx3: u16_digit >>
    space >>
    idx4: u16_digit >> tag!("/") >> idx5: u16_digit >> tag!("/") >> idx6: u16_digit >>
    space >>
    idx7: u16_digit >> tag!("/") >> idx8: u16_digit >> tag!("/") >> idx9: u16_digit >>
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

pub fn loadObj(objname: &str) -> Result<Mesh, &str> {
  let filename = format!("./res/obj/{}.obj", objname);
  let path = Path::new(&filename);
  let display = path.display();
  let mut file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why.description()),
    Ok(file) => file,
  };
  let mut reader = BufReader::new(file);
  
  let mut verts: Vec<Vertex> = Vec::new();
  let mut txtrs: Vec<TextureMap> = Vec::new();
  let mut norms: Vec<Normal> = Vec::new();
  let mut indcs: Vec<u16> = Vec::new();
  
  for line in reader.lines() {
    match &(line.unwrap()) {
      l if &l[..2] == "v " => {
        let (_, position) = get_v(&l).unwrap();
        verts.push(Vertex {position: position});
      }
      l if &l[..3] == "vt " => {
        let (_, txtrmp) = get_vt(&l).unwrap();
        txtrs.push(TextureMap {txtrmp: txtrmp});
      }
      l if &l[..3] == "vn " => {
        let (_, normal) = get_vn(&l).unwrap();
        norms.push(Normal {normal: normal});
      }
      l if &l[..2] == "f " => {
        let (_, indices) = get_f(&l).unwrap();
        indcs.push(indices.0);
        indcs.push(indices.1);
        indcs.push(indices.2);
        indcs.push(indices.3);
        indcs.push(indices.4);
        indcs.push(indices.5);
        indcs.push(indices.6);
        indcs.push(indices.7);
        indcs.push(indices.8);
      }
      _ => {}
    }
  }
  
  Ok( Mesh { verts: verts, txtrs: txtrs, norms: norms, indcs: indcs, far_point: 0_u16} )
}
