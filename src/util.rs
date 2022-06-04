#[macro_export]
macro_rules! v2 {
    ( $x:expr, $y:expr ) => {
        {
            Vec2::new($x, $y)
        }
    };
}

#[derive(Clone, Copy)]
pub enum BoundType{
    Left,
    Right,
    Up,
    Down,
}
impl BoundType{
    pub fn horizontal(&self) -> bool{
        match self{
            BoundType::Left | BoundType::Right => true,
            _ => false,
        }
    }
    pub fn vertical(&self) -> bool{
        !self.horizontal()
    }
    pub fn opposite(&self) -> BoundType{
        match self{
            BoundType::Up    => BoundType::Down,
            BoundType::Down  => BoundType::Up,
            BoundType::Left  => BoundType::Right,
            BoundType::Right => BoundType::Left,
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