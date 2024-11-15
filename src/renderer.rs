use std::{collections::HashMap, hash::Hasher};

use crate::{
    camera::Camera,
    entity::Entity,
    graphics::{
        gl::{GlGraphics, GlResource, GlowModes, Graphics},
        mesh::{MeshData, RenderableMesh},
    },
};

use crate::shader::{Shader, Uniforms};

pub enum DrawingMode {
    OnlyLines,
    Normal,
    DisableCulling,
    EnableCulling,
}

pub struct Renderer {
    shader: Shader,
    graphics: GlGraphics,
    cache: HashMap<u32, RenderableMesh>,
    mode: DrawingMode,
}

impl Renderer {
    pub fn new(gl_resource: GlResource, shader_paths: (&str, &str)) -> Self {
        let shader = Shader::new(gl_resource.clone(), shader_paths.0, shader_paths.1)
            .expect("Could not create Shader object");
        let graphics = GlGraphics::new(gl_resource);
        log::debug!("From renderer");
        Renderer {
            shader,
            graphics,
            cache: HashMap::new(),
            mode: DrawingMode::OnlyLines,
        }
    }

    pub fn begin_frame(&self, camera: &Camera) {
        self.clear_with_color(0.1, 0.2, 0.3);

        //self.graphics.drawing_mode();
        let (view, projection) = (camera.get_view_matrix(), camera.get_projection_mat());
        self.shader.use_program();
        self.shader.set_uniform("view", Uniforms::Mat4(view));
        self.shader
            .set_uniform("projection", Uniforms::Mat4(projection));
    }

    pub fn render(&mut self, entity: &Entity) {
        let mesh_data = entity.mesh_data().as_ref().unwrap();
        let hash_key = self.calculate_mesh_hash(mesh_data);

        let render_data = self.cache.entry(hash_key as u32).or_insert_with(|| {
            log::debug!("Mesh data now found in cache! Adding now...");
            self.graphics.create_renderable_mesh(mesh_data)
        });

        let model = entity.transformation();
        self.shader
            .set_uniform("transformation", Uniforms::Mat4(model));

        self.graphics.draw_mesh(&render_data);
    }

    pub fn render_with_mesh(&mut self, mesh_data: &MeshData) {
        let hash_key = self.calculate_mesh_hash(mesh_data);

        let render_data = self.cache.entry(hash_key as u32).or_insert_with(|| {
            log::debug!("Mesh data now found in cache! Adding now...");
            self.graphics.create_renderable_mesh(mesh_data)
        });

        let transformation = glm::identity();
        self.shader
            .set_uniform("transformation", Uniforms::Mat4(transformation));

        self.graphics.draw_mesh(&render_data);
    }

    pub fn after_closing(&self) {
        for render_data in self.cache.values() {
            self.graphics.delete_renderable_mesh(render_data);
        }
    }

    fn calculate_mesh_hash(&self, mesh_data: &MeshData) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hash;

        let mut hasher = DefaultHasher::new();

        mesh_data.vertices().len().hash(&mut hasher);
        mesh_data.indices().len().hash(&mut hasher);
        hasher.finish()
    }

    pub fn clear_with_color(&self, red: f32, green: f32, blue: f32) {
        self.graphics.clear_with_color(red, green, blue);
    }

    pub fn resize(&self, width: f32, height: f32) {
        self.graphics.resize(width, height)
    }

    pub fn drawing_mode(&mut self, mode: DrawingMode) {
        match mode {
            DrawingMode::Normal => self.graphics.change_drawing_mode(GlowModes::CullFill),
            DrawingMode::DisableCulling => {
                self.graphics.change_drawing_mode(GlowModes::DisableCulling)
            }
            DrawingMode::EnableCulling => {
                self.graphics.change_drawing_mode(GlowModes::EnableCulling)
            }
            DrawingMode::OnlyLines => self.graphics.change_drawing_mode(GlowModes::LinesOnly),
        }
    }
}
