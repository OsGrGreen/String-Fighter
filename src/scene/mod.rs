use std::{collections::HashMap, hash::Hash};

use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};
use glium::{framebuffer::SimpleFrameBuffer, glutin::surface::WindowSurface, texture::DepthTexture2d, Display, Surface, Texture2d};
use light::Light;
use objects::{physics::{physics_object_factory, PhysicsObject}, renderable::renderobjects::RenderObject, WorldObject};
use rand::Rng;
use winit::{keyboard, window::Window};
use crate::{kinematics::inverse::{segment::Segment, segment_to_straight_spline}, rendering::{render::{self, Renderer}, render_camera::RenderCamera}, spline::Spline, util::{create_fbo, create_render_textures, input_handler::{self, InputHandler}, load_texture}};

pub mod bezier_surface;
pub mod objects;
pub mod light;

const G:f32 = 6.67e-5;//e-11;

pub struct Scene<'a>{
    world_objects: Vec<WorldObject>,
    lights: Vec<Light>,
    renderers: HashMap<String,Renderer<'a>>,
    textures: HashMap<String,Texture2d>,
    pub camera: RenderCamera,
    pub scene_tex: Texture2d,
    scene_depth: DepthTexture2d,
}

impl<'a> Scene<'a>{
    pub fn new(camera: RenderCamera, lights: Option<Vec<Light>>, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);

        Scene { world_objects: Vec::new(), lights: lights.unwrap_or(Vec::new()), renderers: HashMap::new(), textures: HashMap::new(),camera, scene_tex: world_texture, scene_depth: depth_world_texture}
    }

    pub fn draw(&mut self, display: &Display<WindowSurface>){
        let mut fbo = create_fbo(&display, &self.scene_tex, &self.scene_depth);
        fbo.clear_color_and_depth((0.05, 0.05, 0.14, 1.0), 1.0);
        for render in &mut self.world_objects{
            let render_id = &render.render_object.render_id;
            if render_id.is_some(){
                render.draw(&mut fbo, &self.camera, self.renderers.get(render_id.as_ref().unwrap().as_str()).unwrap(), &mut self.textures.get(&render.id));
            }
            
        }
        //let mut spline = Spline::new_empty();
        //spline.insert([Vec3::new(2.0, 0.0, 0.5), Vec3::new(0.0, 1.0, 2.1), Vec3::new(1.0, 2.0, 0.0), Vec3::new(1.0, 0.0, -0.5)]);
        //spline.insert_c2(Vec3::new(2.0, 0.0, 0.5));
        //let t = std::time::Instant::now();
        let s1 = Segment::new(Vec3::new(0.0, 0.0, 0.0),5.0,0.0, 0.0);
        let s2: Segment = Segment::new_child(&s1, 2.0, 0.0,0.5);
        let s3: Segment = Segment::new_child(&s2, 2.0, 0.0,2.0);

        let segment_list = vec![s1,s2, s3];
        let spline = segment_to_straight_spline(segment_list);
        //println!("Time to create segments and convert {:?}", t.elapsed());
        let (vbo, inds, rend) = spline.spline_renderer(display);
       
        fbo.draw(&vbo, &inds, &rend.program, &uniform! {u_screenSize: [self.scene_tex.dimensions().0 as f32, self.scene_tex.dimensions().1 as f32], u_thickness: 50.0 as f32, steps: 48.0 as f32, model: Mat4::IDENTITY.to_cols_array_2d(), projection: self.camera.perspective.to_cols_array_2d(), view:self.camera.camera_matrix.to_cols_array_2d()}, &rend.draw_params).unwrap();
    }

    pub fn add_generic_renderer(&mut self, name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]){
        self.renderers.insert(name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
    }

    pub fn get_renderer(&self, name: &str) -> Option<&Renderer>{
        self.renderers.get(name)
    }

    pub fn new_object(&mut self, object_name: &str, render_name: &str, display: &Display<WindowSurface>,vertex_data: &[u8], fragment_data: &[u8], obj_data: &[u8]) -> usize{
        self.renderers.insert(render_name.to_string(), Renderer::init(display, vertex_data, fragment_data, obj_data).unwrap());
        let obj = WorldObject::new(object_name.to_string(),RenderObject::new(Some(render_name.to_string())), physics_object_factory(0, vec![1.0,1.0]));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    

    pub fn new_object_instance(&mut self, object_name: &str,render_name: &str) -> usize{
        let obj = WorldObject::new(object_name.to_string(),RenderObject::new(Some(render_name.to_string())), physics_object_factory(0, vec![1.0,1.0]));
        self.world_objects.push(obj);
        return self.world_objects.len() - 1;
    }
    
    pub fn add_object(&mut self, obj: WorldObject) -> usize{
        let index = self.world_objects.len();
        self.world_objects.push(obj);
        return index
    }

    pub fn add_texture(&mut self, object_name: &str, display: &Display<WindowSurface>, tex_data: &[u8]){
        self.textures.insert(object_name.to_string(), load_texture(display, tex_data));
    }

    pub fn update_camera(&mut self, dt: f32, input_handler: &InputHandler){
        //println!("mouse pos: {}", input_handler.pos());
        if input_handler.is_pressed(keyboard::KeyCode::ControlLeft){
            if input_handler.pos().x.abs() > 0.2{
                let rotation_matrix = Mat4::from_axis_angle(self.camera.get_up(), 5.0*dt*input_handler.pos().x);
                self.camera.update(rotation_matrix);
            }
            if input_handler.pos().y.abs() > 0.3{
                //println!("Right angle is {}", self.camera.get_right());
                let rotation_matrix = Mat4::from_axis_angle(self.camera.get_right(), 5.0*dt*input_handler.pos().y);
                self.camera.update(rotation_matrix);
            }
        }
    }

    pub fn update_physics(&mut self, dt: f32){
        let len = self.world_objects.len();
        for i in 0..len {

        }
        for obj in &mut self.world_objects{
            obj.update_physics(dt);
        }


    }

    pub fn init_gravity_scene(window: &Window, display: &Display<WindowSurface>, size: (u32,u32)) -> Scene<'a>{
        //let planet_renderer  = Renderer::init(display, include_bytes!(r"../../shaders/planet/vert.glsl"), include_bytes!(r"../../shaders/planet/frag.glsl"), include_bytes!(r"../../objects/planet.obj")).unwrap();
        let mut render_map = HashMap::new();        


        let (world_texture, depth_world_texture) = create_render_textures(&display,size.0, size.1);

        let mut return_scene = Scene { world_objects: Vec::new(), lights: Vec::new(), renderers: render_map,camera: RenderCamera::new(Vec3::new(0.0, 0.0, 15.0), Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), window.inner_size().into()), textures: HashMap::new(), scene_tex: world_texture, scene_depth: depth_world_texture};
    
        
        return return_scene;
    }
}