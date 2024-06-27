fn skybox(dir: vec3f) -> vec3f {
    let t = 0.5 * (normalize(dir).y + 1.0);
    return mix(vec3f(1.0, 1.0, 1.0), vec3f(0.5, 0.7, 1.0), t);
}