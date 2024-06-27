struct VoxelRecord {
    intersect: bool,
    normal: vec3f,
    pos: vec3f,
}

fn voxel_traversal(ray: Ray, voxel_size: f32, time: f32) -> VoxelRecord {
    var current_voxel = vec3_floor(ray.origin / voxel_size);

    var step = vec3f(1.);
    if ray.dir.x < 0.0 { step.x = -1.0; }
    if ray.dir.y < 0.0 { step.y = -1.0; }
    if ray.dir.z < 0.0 { step.z = -1.0; }

    let next_voxel_bound = (current_voxel + step) * voxel_size;
    var tMax = (next_voxel_bound - ray.origin) / ray.dir;
    let tDelta = voxel_size / ray.dir * step;
    var normal = vec3f(0.0, 0.0, 0.0);

    let current_voxel_rec = visit_voxel(current_voxel * voxel_size, normal, time);
    if current_voxel_rec.intersect {
        return current_voxel_rec; // Camera inside solid voxel
    }

    var neg_dir = false;
    if ray.dir.x < 0.0 {
        current_voxel.x -= 1.0;
        neg_dir = true;
    }
    if ray.dir.y < 0.0 {
        current_voxel.y -= 1.0;
        neg_dir = true;
    }
    if ray.dir.z < 0.0 {
        current_voxel.z -= 1.0;
        neg_dir = true;
    }
    if neg_dir {
        let current_voxel_rec = visit_voxel(current_voxel * voxel_size, normal, time);
        if current_voxel_rec.intersect {
            return current_voxel_rec; // Camera inside solid voxel after neg fix?
        }
    }

    for (var i = 0; i < i32(VIEW_DISTANCE / voxel_size); i++) {
        if tMax.x < tMax.y {
            if tMax.x < tMax.z {
                current_voxel.x += step.x;
                tMax.x += tDelta.x;
                normal = vec3f(-step.x, 0.0, 0.0);
            } else {
                current_voxel.z += step.z;
                tMax.z += tDelta.z;
                normal = vec3f(0.0, 0.0, -step.z);
            }
        } else {
            if tMax.y < tMax.z {
                current_voxel.y += step.y;
                tMax.y += tDelta.y;
                normal = vec3f(0.0, -step.y, 0.0);
            } else {
                current_voxel.z += step.z;
                tMax.z += tDelta.z;
                normal = vec3f(0.0, 0.0, -step.z);
            }
        }

        let record = visit_voxel(current_voxel * voxel_size, normal, time);
        if record.intersect {
            return record;
        }
    }
    return VoxelRecord(false, vec3f(0.), vec3f(0.));
}

fn visit_voxel(voxel_pos: vec3f, normal: vec3f, time: f32) -> VoxelRecord {
    let intersect = intersect_sphere(voxel_pos, time) || intersect_torus(voxel_pos, time);

    return VoxelRecord(intersect, normal, voxel_pos);
}

fn intersect_torus(voxel_pos: vec3f, time: f32) -> bool {
    let a = fract(time * 0.3) * 2.0 * PI;
    let rot = rot_mat_pitch(a) * rot_mat_roll(a * 0.5);

    let o = vec3f(-1.5, 0.0, -1.0) * rot;

    let major_r = 0.8;
    let minor_r = 0.2;
    
    let p0 = (dot(rot.x, voxel_pos) - o.x);
    let p1 = (dot(rot.y, voxel_pos) - o.y);
    let p2 = (dot(rot.z, voxel_pos) - o.z);
    
    let p3 = sqrt(p0 * p0 + p1 * p1) - major_r;

    return p3 * p3 + p2 * p2 <= minor_r;
}

fn intersect_sphere(voxel_pos: vec3f, time: f32) -> bool {
    let o = vec3f(1.5, -.5, -1.0);
    let r = (sin(time * 1.5) + 1.3) / 2.;

    return length(voxel_pos - o) < r;
}