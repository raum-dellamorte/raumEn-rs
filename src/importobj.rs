use {
  std::{
    // error::Error,
    io::{
      prelude::*,
      BufReader,
    },
    fs::File,
    path::Path,
    str,
    str::FromStr,
  },
  // nalgebra::{
  //   // Vector3,
  //   // Matrix4,
  // },
  nom::{
    bytes::complete::{ tag, },
    character::complete::{
      space1 as space,
      digit1 as digit,
      // alpha1, 
      // alphanumeric1, 
    },
    combinator::{ map_res, },
    number::complete::float,
    IResult,
  },
  crate::{
    eof,
    util::rvertex::*,
  },
};

pub struct Mesh {
  pub verts: Vec<RVertex>,
  pub indcs: Vec<u16>,
  pub far_point: u16,
  pub buffers: Option<MeshBuffers>,
}

pub struct MeshBuffers;

pub fn usize_digit(i: &str) -> IResult<&str, usize> {
  map_res(digit, FromStr::from_str )(i)
}

fn get_v(tstr: &str) -> (f32, f32, f32) {
  let eofs = eof(tstr);
  let (_, x) = _get_v(&eofs)
    .expect("Failed at crate::importobj::get_v");
  x
}
fn _get_v(i: &str) -> IResult<&str, (f32, f32, f32)> {
  let (i, _) = tag("v")(i)?;
  let (i, _) = space(i)?;
  let (i, x) = float(i)?;
  let (i, _) = space(i)?;
  let (i, y) = float(i)?;
  let (i, _) = space(i)?;
  let (i, z) = float(i)?;
  Ok(( i, (x,y,z) ))
}

fn get_vt(tstr: &str) -> (f32, f32) {
  let eofs = eof(tstr);
  let (_, x) = _get_vt(&eofs)
    .expect("Failed at crate::importobj::get_vt");
  x
}
// named!(_get_vt<&str, (f32, f32) >,
//   do_parse!(
//     tag!("vt") >>
//     space >>
//     x: float >>
//     space >>
//     y: float >>
//     (x, y)
//   )
// );
fn _get_vt(i: &str) -> IResult<&str, (f32, f32)> {
  let (i, _) = tag("vt")(i)?;
  let (i, _) = space(i)?;
  let (i, x) = float(i)?;
  let (i, _) = space(i)?;
  let (i, y) = float(i)?;
  Ok(( i, (x,y) ))
}

fn get_vn(tstr: &str) -> (f32, f32, f32) {
  let eofs = eof(tstr);
  let (_, x) = _get_vn(&eofs)
    .expect("Failed at crate::importobj::get_vn");
  x
}
fn _get_vn(i: &str) -> IResult<&str, (f32, f32, f32)> {
  let (i, _) = tag("vn")(i)?;
  let (i, _) = space(i)?;
  let (i, x) = float(i)?;
  let (i, _) = space(i)?;
  let (i, y) = float(i)?;
  let (i, _) = space(i)?;
  let (i, z) = float(i)?;
  Ok(( i, (x,y,z) ))
}

fn get_f_chunk(i: &str) -> IResult<&str, (usize,usize,usize)> {
  let x = _get_f_chunk_a(i);
  if x.is_ok() { return x }
  _get_f_chunk_b(i)
}
fn _get_f_chunk_a(i: &str) -> IResult<&str, (usize,usize,usize)> {
  let (i, x) = usize_digit(i)?;
  let (i, _) = tag("/")(i)?;
  let (i, y) = usize_digit(i)?;
  let (i, _) = tag("/")(i)?;
  let (i, z) = usize_digit(i)?;
  Ok(( i, (x,y,z) ))
}
fn _get_f_chunk_b(i: &str) -> IResult<&str, (usize,usize,usize)> {
  let (i, x) = usize_digit(i)?;
  let (i, _) = tag("//")(i)?;
  let (i, z) = usize_digit(i)?;
  Ok(( i, (x,1,z) ))
}

fn get_f(tstr: &str) -> ( (usize,  usize,  usize), (usize, usize, usize), (usize, usize, usize) ) {
  let eofs = eof(tstr);
  let (_, x) = _get_f(&eofs)
    .expect("Failed at crate::importobj::get_f");
  x
}
fn _get_f(i: &str) -> IResult<&str, ( (usize,usize,usize), (usize,usize,usize), (usize,usize,usize) )> {
  let (i, _) = tag("f")(i)?;
  let (i, _) = space(i)?;
  let (i, x) = get_f_chunk(i)?;
  let (i, _) = space(i)?;
  let (i, y) = get_f_chunk(i)?;
  let (i, _) = space(i)?;
  let (i, z) = get_f_chunk(i)?;
  Ok(( i, (x,y,z) ))
}

pub fn test_nom() {
  {
    let v = "v -0.866025 0.000000 -0.500000";
    let (x, y, z) = get_v(v);
    println!("x: {}, y: {}, z: {}", x, y, z);
  }
  {
    let vt = "vt 0.523785 0.851270";
    let (xt, yt) = get_vt(vt);
    println!("x: {}, y: {}", xt, yt);
  }
  {
    let vn = "vn 0.499985 0.000000 0.866024";
    let (xn, yn, zn) = get_vn(&vn);
    println!("x: {}, y: {}, z: {}", xn, yn, zn);
  }
  {
    let f = "f 183/1/1 6/2/1 12/3/1";
    let (v1, v2, v3) = get_f(f);
    println!("vertex 1: {} {} {}\nvertex 2: {} {} {}\nvertex 3: {} {} {}", v1.0, v1.1, v1.2, v2.0, v2.1, v2.2, v3.0, v3.1, v3.2);
  }
  {
    let f2 = "f 183//1 6//1 12//1";
    let (v4, v5, v6) = get_f(f2);
    println!("vertex 1: {} {} {}\nvertex 2: {} {} {}\nvertex 3: {} {} {}", v4.0, v4.1, v4.2, v5.0, v5.1, v5.2, v6.0, v6.1, v6.2);
  }
}

pub fn load_obj(objname: &str) -> Result<Mesh, &str> {
  let filename = format!("res/obj/{}.obj", objname);
  let path = Path::new(&filename);
  let display = path.display();
  let file = match File::open(&path) {
    Err(why) => panic!("couldn't open {}: {}", display, why),
    Ok(file) => file,
  };
  let reader = BufReader::new(file);
  
  let mut verts: Vec<RVertex> = Vec::new();
  let mut txtrs: Vec<[f32; 2]> = Vec::new();
  let mut norms: Vec<[f32; 3]> = Vec::new();
  let mut indcs: Vec<u16> = Vec::new();
  for line in reader.lines() {
    match &(line.unwrap()) {
      l if &l[..2] == "v " => {
        let vert = &mut RVertex::new();
        vert.position = t3f_array(get_v(l));
        verts.push(*vert);
      }
      l if &l[..3] == "vt " => { txtrs.push(t2f_array(get_vt(l))); }
      l if &l[..3] == "vn " => { norms.push(t3f_array(get_vn(l))); }
      l if &l[..2] == "f " => {
        let (v1, v2, v3) = get_f(l);
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
  
  Ok( Mesh { verts, indcs, far_point: 0_u16, buffers: None} )
}

fn t3f_array(tpl: (f32, f32, f32)) -> [f32; 3] {
  let mut out = [0_f32; 3];
  out[0] = tpl.0;
  out[1] = tpl.1;
  out[2] = tpl.2;
  out
}

fn t2f_array(tpl: (f32, f32)) -> [f32; 2] {
  let mut out = [0_f32; 2];
  out[0] = tpl.0;
  out[1] = 1.0_f32 - tpl.1;
  out
}

//fn processVertex(verts: &mut Vec<RVertex>, index)
