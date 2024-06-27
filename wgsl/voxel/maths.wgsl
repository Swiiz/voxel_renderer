struct Ray {
  origin: vec3f,
  dir: vec3f,
}

fn ray_at(ray: Ray, t: f32) -> vec3f {
    return ray.origin + t * ray.dir;
}

fn hit_sphere(center: vec3f, radius: f32, ray: Ray) -> bool {
    let oc = center - ray.origin;
    let a = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(ray.dir, oc);
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    return discriminant >= 0.0;
}

fn vec3_floor(v: vec3f) -> vec3f {
    return vec3f(floor(v.x), floor(v.y), floor(v.z));
}

const PI = 3.1415926535897932384626433832795;

fn rot_mat_yaw(a: f32) -> mat3x3f {
    return mat3x3f(
        cos(a), -sin(a), 0.0,
        sin(a), cos(a), 0.0,
        0.0, 0.0, 1.0
    );
}

fn rot_mat_pitch(a: f32) -> mat3x3f {
    return mat3x3f(
        cos(a), 0.0, sin(a),
        0.0, 1.0, 0.0,
        -sin(a), 0.0, cos(a)
    );
}

fn rot_mat_roll(a: f32) -> mat3x3f {
    return mat3x3f(
        1.0, 0.0, 0.0,
        0.0, cos(a), -sin(a),
        0.0, sin(a), cos(a)
    );
}