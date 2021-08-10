//
// geometry.rs
//

use glam::Vec3;

pub trait Positions {
    fn iter_pos(&self) -> Box<dyn Iterator<Item = &Vec3> + '_>;
    fn iter_pos_mut(&mut self) -> Box<dyn Iterator<Item = &mut Vec3> + '_>;
}

pub trait Bounds {
    fn bbox(&self) -> (Vec3, Vec3);
}

impl<T> Bounds for T
where
    T: Positions,
{
    fn bbox(&self) -> (Vec3, Vec3) {
        self.iter_pos().fold(
            (Vec3::splat(std::f32::MAX), Vec3::splat(std::f32::MIN)),
            |a, b| (a.0.min(*b), a.1.max(*b)),
        )
    }
}

impl<T> Bounds for Vec<T>
where
    T: Positions,
{
    fn bbox(&self) -> (Vec3, Vec3) {
        self.iter().map(|b| b.bbox()).fold(
            (Vec3::splat(std::f32::MAX), Vec3::splat(std::f32::MIN)),
            |a, b| (a.0.min(b.0), a.1.max(b.1)),
        )
    }
}

pub trait Centered {
    fn centered(&self) -> Self;
}

impl<T> Centered for T
where
    T: Positions + Clone,
{
    fn centered(&self) -> Self {
        let bbox = self.bbox();
        let diff = (bbox.0 + bbox.1) / 2.0;
        let mut v = self.clone();
        v.iter_pos_mut().for_each(|p| *p -= diff);
        v
    }
}

impl<T> Centered for Vec<T>
where
    T: Positions + Clone,
{
    fn centered(&self) -> Self {
        let bbox = self.bbox();
        let diff = (bbox.0 + bbox.1) / 2.0;
        let mut v = self.clone();
        v.iter_mut()
            .flat_map(|p| p.iter_pos_mut())
            .for_each(|p| *p -= diff);
        v
    }
}
