struct Camera {
    position: vec3f,
    upper_left: vec3f,
    pixel_delta_u: vec3f,
    pixel_delta_v: vec3f,
}

fn camera_ray(camera: Camera, coords: vec2u) -> Ray {
    let pixel_center = camera.upper_left + (f32(coords.x) * camera.pixel_delta_u) + (f32(coords.y) * camera.pixel_delta_v);
    let ray_dir = pixel_center - camera.position;
    return Ray(camera.position, ray_dir);
}