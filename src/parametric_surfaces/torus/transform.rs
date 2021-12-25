use js_sys::Number;
use nalgebra_glm as glm;
use std::f32::consts::PI;

pub fn model_matrix(dtheta: Number) -> Vec<f32> {
    let theta = ((PI / 4.0) + (dtheta.as_f64().unwrap() as f32)) % (2.0 * PI);
    let identity = glm::TMat4::identity();
    let rotate = glm::rotate(&identity, theta, &glm::vec3(0.0, 1.0, 1.0));
    let transl = glm::translate(&identity, &glm::vec3(0.0, 0.0, -2.0));
    let mat = transl * rotate;

    let model: [[f32; 4]; 4] = *mat.as_ref();

    model.into_iter().flatten().collect::<Vec<f32>>()
}

pub fn view_matrix() -> Vec<f32> {
    let cam_position = glm::vec3(0.0, 0.0, 0.0);
    let cam_target = glm::vec3(0.0, 0.0, -1.0);
    let cam_up = glm::vec3(0.0, 1.0, 0.0);

    let mat = glm::look_at(&cam_position, &cam_target, &cam_up);
    let view: [[f32; 4]; 4] = *mat.as_ref();

    view.into_iter().flatten().collect::<Vec<f32>>()
}

pub fn projection_matrix(canvas_width: f64, canvas_height: f64) -> Vec<f32> {
    let aspect_ratio = canvas_width as f32 / canvas_height as f32;
    let fov = PI / 4.0;
    let near = 0.1;
    let far = 100.0;
    let mat = glm::perspective(aspect_ratio, fov, near, far);
    let projection: [[f32; 4]; 4] = *mat.as_ref();

    projection.into_iter().flatten().collect::<Vec<f32>>()
}
