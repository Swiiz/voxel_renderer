#import maths
#import camera
#import skybox
#import traversal

struct Params {
    width: u32,
    height: u32,
    time: f32,
};

@group(0) @binding(0) var outputTex: texture_storage_2d<rgba8unorm, write>;
@group(0) @binding(1) var<uniform> params: Params;
@group(0) @binding(2) var<uniform> camera: Camera;

@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_ix: vec3u) {
    textureStore(
        outputTex,
        vec2i(global_ix.xy),
        color_at(global_ix.xy, params, camera)
    );
}

const VOXEL_SIZE = 0.1;
const VIEW_DISTANCE = 10.;

fn color_at(coords: vec2u, params: Params, camera: Camera) -> vec4f {
    let ray = camera_ray(camera, coords);

    var rgb = skybox(ray.dir);

    let voxel_record = voxel_traversal(ray, VOXEL_SIZE, params.time);
    if voxel_record.intersect {
        let voxel_color = (voxel_record.pos + vec3f(1.)) * 0.5;
        rgb = voxel_color * lighting(voxel_record.normal, ray.dir);
    }
    
    return vec4f(rgb, 1.0);
}

fn lighting(normal: vec3f, dir: vec3f) -> vec3f {
    let light_dir = normalize(vec3f(-1.0, -0.5, 1.0));
    let diffuse_attn = max(0.0, dot(normal, light_dir));
    let light = vec3f(0.9);

    let ambient = vec3f(0.3);

    let reflected = reflect(dir, normal);
    let specular_attn = max(dot(reflected, light_dir), 0.0);

    return diffuse_attn * light * 1.0 + specular_attn * light * 0.6 + ambient;
}

