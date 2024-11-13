use crate::{
    camera::Camera,
    component::Entity,
    graphics::{
        gl::{GlGraphics, GlResource, Graphics},
        mesh::RenderableMesh,
    },
};

use crate::shader::Shader;

pub struct Renderer {
    shader: Shader,
    graphics: GlGraphics,
}

impl Renderer {
    pub fn new(gl_resource: GlResource, shader_paths: (&str, &str)) -> Self {
        let shader = Shader::new(gl_resource.clone(), shader_paths.0, shader_paths.1)
            .expect("Could not create Shader object");
        let graphics = GlGraphics::new(gl_resource);
        log::debug!("From renderer");
        Renderer { shader, graphics }
    }

    pub fn begin_frame(&self, camera: &Camera) {
        self.clear_with_color(0.1, 0.2, 0.3);
        let (view, projection) = (camera.get_view_matrix(), camera.get_projection_mat());
        self.shader.use_program();
        self.shader.set_mat4("view", &view);
        self.shader.set_mat4("projection", &projection);
    }

    pub fn render(&self, entity: &Entity) {
        let mesh_data = entity.mesh_data().as_ref().unwrap();
        let r_mesh_data = self.graphics.create_renderable_mesh(&mesh_data);

        let model = entity.transformation();
        self.shader.set_mat4("model", &model);

        self.graphics.draw_mesh(&r_mesh_data);
    }
    fn draw_mesh_data(&self, mesh: &RenderableMesh) {
        self.graphics.draw_mesh(mesh);
    }

    pub fn clear_with_color(&self, red: f32, green: f32, blue: f32) {
        self.graphics.clear_with_color(red, green, blue);
    }

    pub fn resize(&self, width: f32, height: f32) {
        self.graphics.resize(width, height)
    }
}
