use glam::Vec3;

pub struct Segment{
   pub start: Vec3,
   pub end: Vec3,
   pub len: f32,
   pub angle_y: f32,
   pub angle_z: f32
}

//Should we use angles?
impl Segment{
    pub fn new(start: Vec3, len: f32, angle_y:f32, angle_z:f32) -> Segment{
        let dx = len*angle_z.cos()*angle_y.sin();
        let dy = len*angle_z.sin();
        let dz = len*angle_z.cos()*angle_y.cos();
        let end = Vec3::new(start.x+dx, start.y+dy, start.z+dz);

        Segment { start: start, end: end, len: len, angle_y: angle_y, angle_z: angle_z}
    }

    pub fn new_child(prev: &Segment, len: f32, angle_y:f32, angle_z:f32) -> Segment{
        let dx = len*angle_z.cos()*angle_y.sin();
        let dy = len*angle_z.sin();
        let dz = len*angle_z.cos()*angle_y.cos();
        let end = Vec3::new(prev.end.x+dx, prev.end.y+dy, prev.end.z+dz);

        Segment { start: prev.end, end: end, len: len, angle_y: angle_y, angle_z: angle_z }
    }
}