#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {
        Vec2::new($x, $y)
    };
    () => {
        Vec2::ZERO
    };
}

#[macro_export]
macro_rules! iter_zip {
    ( $ecs:expr, $t1:ty, $t2:ty) => {
        ($ecs).borrow_component_vec::<$t1>().unwrap().iter()
            .zip(($ecs).borrow_component_vec::<$t2>().unwrap().iter())
            .filter_map(|(x1, x2)| Some((x1.as_ref()?, x2.as_ref()?)))
    }
}

#[macro_export]
macro_rules! iter_zip_mut {
    ( $ecs:expr, $t1:ty, $t2:ty) => {
        ($ecs).borrow_component_vec_mut::<$t1>().unwrap().iter_mut()
            .zip(($ecs).borrow_component_vec_mut::<$t2>().unwrap().iter_mut())
            .filter_map(|(x1, x2)| Some((x1.as_mut()?, x2.as_mut()?)))
    }
}

#[macro_export]
macro_rules! update {
    { [$ecs: expr, $assets: expr, $rng: expr, $time: expr, $dt: expr]
    $($t: ty: $($id: expr),*);*$(;)? } => {
        $(
            $(
                <$t>::update($ecs, $assets, $rng, $id, $time, $dt);
            )*
        )*
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