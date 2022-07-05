use crate::types_and_constants::*;
use glam::*;

#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {
        {
            Vec2::new($x, $y)
        }
    };
}

#[macro_export]
macro_rules! iter_zip {
    ( $world:expr, $t1:ty, $t2:ty) => {
        {
            ($world).borrow_component_vec::<$t1>().unwrap().iter_mut()
                .zip(($world).borrow_component_vec::<$t2>().unwrap().iter_mut())
                .filter_map(|(x1, x2)| Some((x1.as_mut()?, x2.as_mut()?)))
        }
    }
}

#[cfg(test)]
mod tests{
    use glam::Vec2;
    #[test]
    fn v2_test(){
        let x = 5.0;
        let y = -5.0;
        let v1 = Vec2::new(x,y);
        let v2 = v2!(x, y);
        assert_eq!(v1, v2);
    }
}

/// World and screen positions

pub fn world_to_screen_coords(screen_size: Screen2, point: Vec2) -> Vec2 {
    let x = point.x + screen_size.0 / 2.0;
    let y = screen_size.1 - (point.y + screen_size.1 / 2.0);
    Vec2::new(x, y)
}

// fn screen_bound(screen_size: Screen2, bound: BoundType) -> Vec2{
//     let screen_hsize = (screen_size.0 / 2.0, screen_size.1 / 2.0);
//     Collider::box_bound_offs(Vec2::from(screen_hsize), bound)
// }