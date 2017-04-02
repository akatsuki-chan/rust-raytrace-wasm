use std::cmp::Ordering;
use std::rc::Rc;
use std::ops::{Mul, Add, Sub, Div};
use std::slice;
use std::string::String;

#[derive(Copy, Clone, Debug)]
struct Vector {
  x: f32,
  y: f32,
  z: f32,
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector { x: self.x + other.x , y: self.y + other.y, z: self.z + other.z }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector { x: self.x - other.x , y: self.y - other.y, z: self.z - other.z }
    }
}

impl Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector {
        Vector { x: self.x / other.x , y: self.y / other.y, z: self.z / other.z }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, other: f32) -> Vector {
        Vector { x: self.x / other , y: self.y / other, z: self.z / other }
    }
}

impl Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector { x: self.x * other.x , y: self.y * other.y, z: self.z * other.z }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector { x: self.x * other , y: self.y * other, z: self.z * other }
    }
}

impl Vector {
  fn new(x: f32, y: f32, z: f32) -> Vector {
    Vector {
      x: x, y: y, z: z,
    }
  }

  fn magnitude(&self) -> f32 {
    self.magnitude2().sqrt()
  }
  
  fn magnitude2(&self) -> f32 {
    self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)
  }

  fn update<F>(&self, f: F, v: f32) -> Vector 
    where F: Fn(f32, f32) -> f32 {

    Vector::new(f(self.x, v), f(self.y, v), f(self.z, v))
  }

  fn normalize(&self) -> Vector {
      let m = self.magnitude();

      let f = |v1, v2| v1 / v2;
      self.update(f, m)
  }

  fn dot(&self, v: Vector) -> f32 {
    self.x * v.x + self.y * v.y + self.z * v.z
  }
}

#[derive(Copy, Clone, Debug)]
pub struct RGB {
  r: f32,
  g: f32,
  b: f32,
}

impl RGB {
  fn black() -> RGB {
    RGB { r: 0.0, g: 0.0, b: 0.0}
  }
  
  fn white() -> RGB {
    RGB { r: 1.0, g: 1.0, b: 1.0}
  }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, other: RGB) -> RGB {
        RGB { r: self.r + other.r , g: self.g + other.g, b: self.b + other.b }
    }
}

impl Mul<RGB> for RGB {
    type Output = RGB;

    fn mul(self, other: RGB) -> RGB {
        RGB { r: self.r * other.r, g: self.g * other.g, b: self.b * other.b }
    }
}

impl Mul<f32> for RGB {
    type Output = RGB;

    fn mul(self, other: f32) -> RGB {
        RGB { r: self.r * other , g: self.g * other, b: self.b * other }
    }
}

#[derive(Clone, Debug)]
struct Sphere {
  pos: Vector,
  rad: f32,
  color: RGB,
}

impl Sphere {
  fn new() -> Sphere {
    Sphere {
      pos: Vector{ x: 0.0, y: 0.0, z: 0.0 },
      rad: 1.0,
      color: RGB::white()
    }
  }
}

#[derive(Clone, Debug)]
struct Light {
  pos: Vector, 
  color: RGB,
}

#[derive(Clone, Debug)]
struct Ray {
  dir: Vector, // -1.0 <=> 1.0
  pos: Vector
}

impl Ray {
  fn new(w: i32, h: i32, x: i32, y: i32) -> Ray {
    let (rx, ry) = Ray::coord(w, h, x, y);

    Ray {
      dir: Vector { x: rx, y: ry, z: 5.0 }.normalize(),
      pos: Vector { x: 0.0, y: 0.0, z: -5.0 }
    }
  }

  fn coord(w: i32, h: i32, x: i32, y: i32) -> (f32, f32) {
    let rx: f32 = ((x * 2 - w) as f32) / (w as f32);
    let ry: f32 = ((y * 2 - h) as f32) / (h as f32);

    (rx, ry)
  }

  fn intersect(&self, sphere: &Sphere) -> Option<f32> {
    let m = self.pos.sub(sphere.pos);
    let b = m.dot(self.dir);
    let c = b.powf(2.0) - m.magnitude2() + sphere.rad.powf(2.0);

    if c < 0.0 {
      return None
    }

    let t = -b - c.sqrt();
    if t > 0.0 {
      Some(t)
    } else {
      None
    }
  }
}

#[derive(Clone, Debug)]
struct Model {
  width: i32,
  height: i32,
  sphere: Sphere,
  lights: Vec<Light>
}

impl Model {
  fn new(w: i32, h: i32) -> Model {
    let light1 = Light { 
      pos: Vector { x: 3.0, y: -2.0, z: -2.0, },
      color: RGB { r: 0.0, g: 1.0, b: 0.0, },
    };
    let light2 = Light { 
      pos: Vector { x: -3.0, y: 2.0, z: -2.0, },
      color: RGB { r: 1.0, g: 0.0, b: 0.0, },
    };
    let light3 = Light { 
      pos: Vector { x: 3.0, y: 2.0, z: -2.0, },
      color: RGB { r: 0.0, g: 0.0, b: 1.0, },
    };
    Model {
      width: w, 
      height: h,
      sphere: Sphere::new(),
      lights: vec![light1, light2, light3]
    }
  }

  // Rayと球の距離で色計算
  fn simulate(dist: f32, light: &Light, ray: Rc<Ray>, sph: &Sphere) -> RGB {
    let r = ray.pos + (ray.dir * dist) - sph.pos;
    let n = r / sph.rad;
    let l = (light.pos - r).normalize();
    let b = n.dot(l);

    match 0.0.partial_cmp(&b) {
      Some(Ordering::Greater) => RGB::black(),
      Some(Ordering::Less) => sph.color * light.color * b,
      Some(Ordering::Equal) => sph.color * light.color * b,
      None => RGB::black()
    }
  }

  fn render(&self, ray: Rc<Ray>)  -> RGB {
    match ray.intersect(&self.sphere) {
      Some(dist) => {
        let mut black = RGB::black();

        for light in &self.lights {
          let t = Model::simulate(dist, light, ray.clone(), &self.sphere);

          black = black + t;
        }

        black
      },

      None => RGB::black()
    }
  }
}

extern {}

fn main() {}

fn raytrace(width: i32, height: i32) -> Vec<f32> {
  let model = Model::new(width, height);
  let mut data = vec![];

  for y in 0..(model.height as i32) {
    for x in 0..(model.width as i32) {
      let ray = Ray::new(width, height, x, y);
      let scene = model.render(Rc::new(ray));

      data.push(scene.r);
      data.push(scene.g);
      data.push(scene.b);
      data.push(1.0);
    }
  };

  data
}

#[no_mangle]
pub extern fn hello(p: *mut u8, len: usize) -> *const *const f32 {
  let input = unsafe {
      String::from_raw_parts(p, len, len)
  };
  println!("Hello {:?}.", input);

  let a = [[0.1,0.2,0.3].as_ptr(), [0.4].as_ptr()];

  a.as_ptr()
}

#[no_mangle]
pub extern fn raytrace1(width: i32, height: i32) -> *const f32 {
  let data = raytrace(width, height);
  let len = (width * height * 4) as usize;

  let mut tmp: [f32; 1_000_000] = [0.0; 1_000_000];
  let s = data.as_slice() as &[f32];
  tmp[..len].clone_from_slice(&s[..len]);

  tmp.as_ptr()
}

#[no_mangle]
pub extern fn raytrace2(width: i32, height: i32, p: *mut f32) {
  let mut buffer = unsafe {
    let len = (width*height*4) as usize;
    slice::from_raw_parts_mut(p, len) // 幅＊高さ＊RGBA
  };

  let data = raytrace(width, height);
  let s: &[f32] = data.as_slice() as &[f32];

  buffer.clone_from_slice(&s);
}