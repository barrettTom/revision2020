
use luminance::context::GraphicsContext;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{BoundTexture, PipelineState};
use luminance::render_state::RenderState;
use luminance::tess::{Mode, Tess, TessBuilder};
use luminance::texture::{Dim2, GenMipmaps, Sampler, Texture};
use luminance::pixel::{NormRGB8UI, NormUnsigned};
use luminance_derive::UniformInterface;
use luminance_derive::{Semantics, Vertex};
use luminance::linear::M44;


use luminance::shader::program::{Program, Uniform};
use luminance_glfw::{GlfwSurface};
use std::time::Instant;
use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
use std::path::Path;

use crate::constants;


const VS: &'static str = include_str!("./shaders/wave-vs.glsl");
const FS: &'static str = include_str!("./shaders/wave-fs.glsl");

#[derive(UniformInterface)]
struct ShaderInterface {
  // the 'static lifetime acts as “anything” here
  tex: Uniform<&'static BoundTexture<'static, Dim2, NormUnsigned>>,
  position: Uniform<[f32; 2]>,
  intensity: Uniform<f32>,
  time: Uniform<f32>,

  #[uniform(unbound)]
  projection: Uniform<M44>,
  #[uniform(unbound)]
  view: Uniform<M44>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
  #[sem(name = "pos", repr = "[f32; 2]", wrapper = "VertexPosition")]
  Position,
  #[sem(name = "uv", repr = "[f32; 2]", wrapper = "UVCoordinate")]
  UV,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
  pos: VertexPosition,
  uv: UVCoordinate,
}

#[repr(C)]
struct Beat {
  time: f32,
  position: [f32; 2],
  intensity: f32
}

const BEATS: [Beat; 15] = [
  Beat {
    time: 1.03,
    position: [0.5, -0.5],
    intensity: 0.06,
  },
  Beat {
    time: 3.47,
    position: [-0.5, -0.5],
    intensity: 0.08,
  },
  Beat{
    time: 5.84,
    position: [0.5, 0.5],
    intensity: 0.03,
  },
  Beat{
    time: 8.30,
    position: [0.5, -0.5],
    intensity: 0.04,
  },
  Beat{
    time: 10.76,
    position: [-0.5, 0.5],
    intensity: 0.06,
  },
  Beat{
    time: 13.17,
    position: [-0.5, -0.5],
    intensity: 0.08,
  },
  Beat{
    time: 15.60,
    position: [0.5, -0.5],
    intensity: 0.09,
  },
  Beat{
    time: 18.03,
    position: [-0.5, -0.5],
    intensity: 0.03,
  },
  Beat{
    time: 20.48,
    position: [-0.5, 0.5],
    intensity: 0.07,
  },
  Beat{
    time: 22.9,
    position: [0.5, -0.5],
    intensity: 0.02,
  },
  Beat{
    time: 25.30,
    position: [-0.5, -0.5],
    intensity: 0.12,
  },
  Beat{
    time: 27.76,
    position: [0.5, 0.5],
    intensity: 0.05,
  },
  Beat{
    time: 30.20,
    position: [-0.5, 0.5],
    intensity: 0.07,
  },
  Beat{
    time: 32.6,
    position: [0.5, 0.5],
    intensity: 0.08,
  },
  Beat{
    time: 35.07,
    position: [0.5, -0.5],
    intensity: 0.04,
  },
];

const TRI_VERTICES: [Vertex; 4] = [
  // First triangle – an RGB one.
  Vertex::new(
    VertexPosition::new([-1.0, 1.0]),
    UVCoordinate::new([0.0, 1.0]),
  ),
  Vertex::new(
    VertexPosition::new([1.0, 1.0]),
    UVCoordinate::new([1.0, 1.0]),
  ),
  Vertex::new(
    VertexPosition::new([-1.0, -1.0]),
    UVCoordinate::new([0.0, 0.0]),
  ),
  Vertex::new(
    VertexPosition::new([1.0, -1.0]),
    UVCoordinate::new([1.0, 0.0]),
  ),
];




//#[derive(Default)]
pub struct N1ck {
    index: usize,
    program: Program<Semantics, (), ShaderInterface>,
    projection: Matrix4<f32>,
    start_time: Instant,
    //surface: GraphicsContext,
    tess:  Vec<Tess>,
    tex: Texture<luminance::texture::Dim2, luminance::pixel::NormRGB8UI>,
    view: Matrix4::<f32>,
}

impl N1ck {
    pub fn new(  surface: &mut GlfwSurface,
    ) -> N1ck {
        let img = read_image(Path::new("data/djbt.jpg")).expect("error while reading image on disk");
        let tex = load_from_disk(surface, img);

        let program = Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("program creation")
        .ignore_warnings();

        const FOVY: Rad<f32> = Rad(std::f32::consts::PI / 2.);
        const Z_NEAR: f32 = 0.1;
        const Z_FAR: f32 = 10.;
      
        let projection = perspective(
          FOVY,
          constants::WIDTH as f32 / constants::WIDTH as f32,
          Z_NEAR,
          Z_FAR,
        );
      
        let view = Matrix4::<f32>::look_at(Point3::new(0., 0., 1.), Point3::origin(), Vector3::unit_y());
      
        N1ck {
            index: 0,
            program,
            projection,
            start_time: Instant::now(),
            tess: Vec::new(),
            tex,
            view,
        }
    }

    pub fn update<T: GraphicsContext>(&mut self, mut surface: T) -> T {
        //let elapsed = &self.start_time.elapsed();
        //let time = elapsed.as_secs() as f64 + (f64::from(elapsed.subsec_millis()) * 1e-3);
        self.tess.clear();
    
          self.tess.push(
            TessBuilder::new(&mut surface)
            .add_vertices(TRI_VERTICES)
            .set_mode(Mode::TriangleStrip)
            .build()
            .unwrap()
        );
        let elapsed = self.start_time.elapsed();
        let time = elapsed.as_secs() as f64 + (f64::from(elapsed.subsec_millis()) * 1e-3);
        if time as f32 > BEATS[self.index].time {
          self.index = if self.index < 14 { self.index + 1 } else {14};
        }

        surface
    }

    pub fn draw<T: GraphicsContext>(
        &self,
        mut surface: T,
        back_buffer: &Framebuffer<Dim2, (), ()>,
        pipeline_state: &PipelineState,
    ) -> T {
        surface.pipeline_builder().pipeline(
            &back_buffer,
            &pipeline_state,
            |_pipeline, mut shd_gate| {
              let bound_tex = _pipeline.bind_texture(&self.tex);

              shd_gate.shade(&self.program, |interface, mut rdr_gate| {

                let elapsed = self.start_time.elapsed();
                let time = elapsed.as_secs() as f64 + (f64::from(elapsed.subsec_millis()) * 1e-3);

                interface.tex.update(&bound_tex);
                interface.position.update(BEATS[self.index].position);
                interface.intensity.update(BEATS[self.index].intensity);
                interface.time.update(time as f32);
                interface.projection.update(self.projection.into());
                interface.view.update(self.view.into());

                rdr_gate.render(&RenderState::default(), |mut tess_gate| {
                  for tesselation in self.tess.iter() {
                    tess_gate.render(tesselation);
                  }                    
                 });
              });
            },
        );

        surface
    }
}


// read the texture into memory as a whole bloc (i.e. no streaming)
fn read_image(path: &Path) -> Option<image::RgbImage> {
  image::open(path).map(|img| img.flipv().to_rgb()).ok()
}

fn load_from_disk(
  surface: &mut GlfwSurface,
  img: image::RgbImage,
) -> Texture<Dim2, NormRGB8UI> {
  let (width, height) = img.dimensions();
  let texels = img.into_raw();

  // create the luminance texture; the third argument is the number of mipmaps we want (leave it
  // to 0 for now) and the latest is the sampler to use when sampling the texels in the
  // shader (we’ll just use the default one)
  let tex = Texture::new(surface, [width, height], 0, Sampler::default())
    .expect("luminance texture creation");

  // the first argument disables mipmap generation (we don’t care so far)
  tex.upload_raw(GenMipmaps::No, &texels).unwrap();

  tex
}