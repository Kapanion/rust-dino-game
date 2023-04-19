use ggez::graphics::DrawParam;
use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxCollider{
    half_size: Vec2,
    offset: Vec2,
}

impl BoxCollider{
    pub fn new(half_size: Vec2) -> BoxCollider {
        BoxCollider{
            half_size,
            offset: v2!(0.0, 0.0),
        }
    }

    pub fn with_offset(&self, offset: Vec2) -> BoxCollider{
        let mut new_collider = self.clone();
        new_collider.offset = offset;
        new_collider
    }

    pub fn contains_point(&self, pos: Vec2, point: Vec2) -> bool {
        point.x >= pos.x - self.half_size.x &&
        point.x <= pos.x + self.half_size.x &&
        point.y >= pos.y - self.half_size.y &&
        point.y <= pos.y + self.half_size.y
    }

    fn get_corners(&self, pos: Vec2) -> [Vec2; 4] {
        let mut arr = [pos; 4];
        arr[0].x -= self.half_size.x;  arr[0].y -= self.half_size.y;
        arr[1].x -= self.half_size.x;  arr[1].y += self.half_size.y;
        arr[2].x += self.half_size.x;  arr[2].y -= self.half_size.y;
        arr[3].x += self.half_size.x;  arr[3].y += self.half_size.y;
        arr
    }

    pub fn check_collision(col1: BoxCollider, col2: BoxCollider, pos1: Vec2, pos2: Vec2) -> bool{
        let pos1 = pos1 + col1.offset;
        let pos2 = pos2 + col2.offset;
        for corner in col1.get_corners(pos1){
            if col2.contains_point(pos2, corner) {
                return true;
            }
        }
        for corner in col2.get_corners(pos2){
            if col1.contains_point(pos1, corner) {
                return true;
            }
        }
        false
    }

    pub fn check_entity_collision(ecs: &ECS, entity1: usize, entity2: usize) -> bool {
        let col1 = ecs.get_component::<BoxCollider>(entity1);
        if col1 == None {return false}
        let col1 = col1.unwrap();
        let col2 = ecs.get_component::<BoxCollider>(entity2);
        if col2 == None {return false}
        let col2 = col2.unwrap();
        let pos1 = Collider::get_pos(ecs, entity1);
        let pos2 = Collider::get_pos(ecs, entity2);
        BoxCollider::check_collision(col1, col2, pos1, pos2)
    }

    pub fn get_bound_offset(&self, bound: BoundType) -> Vec2{
        self.offset +
        match bound {
            BoundType::Up    => v2!(0.0,  self.half_size.y),
            BoundType::Down  => v2!(0.0, -self.half_size.y),
            BoundType::Left  => v2!(-self.half_size.x, 0.0),
            BoundType::Right => v2!( self.half_size.x, 0.0),
        }
    }

    pub fn get_bound(&self, pos: Vec2, bound: BoundType) -> Vec2{
        pos + self.get_bound_offset(bound)
    }
}

impl Draw for BoxCollider{
    fn draw(&self, ctx: &mut Context, _ecs: &ECS, _assets: &Assets, _entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult {
        let pos = world_to_screen_coords(screen_size, pos + self.offset) - self.half_size;
        
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::stroke(1.5),
            graphics::Rect::new(pos.x, pos.y, self.half_size.x * 2.0, self.half_size.y * 2.0),
            Color::RED
        )?;
        
        graphics::draw(ctx, &rectangle, DrawParam::new())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Collider {
    col: [Option<BoxCollider>; NUM_OF_COLLIDERS],
}

impl Collider {
    pub fn new(col: [Option<BoxCollider>; NUM_OF_COLLIDERS]) -> Collider {
        Collider {
            col,
        }
    }
    pub fn new_single(col1: BoxCollider) -> Collider {
        let mut col = [None; NUM_OF_COLLIDERS];
        col[0] = Some(col1);
        Collider {
            col,
        }
    }
    pub fn new_double(col1: BoxCollider, col2: BoxCollider) -> Collider {
        let mut col = [None; NUM_OF_COLLIDERS];
        col[0] = Some(col1);
        col[1] = Some(col2);
        Collider {
            col,
        }
    }

    pub fn get_pos(ecs: &ECS, entity_id: usize) -> Vec2{
        ecs.get_component::<Movable>(entity_id).unwrap().pos
    }

    pub fn check_entity_collision(ecs: &ECS, entity1: usize, entity2: usize) -> bool {
        let col1 = ecs.get_component::<Collider>(entity1);
        if col1 == None {return false}
        let col1 = col1.unwrap();
        let col2 = ecs.get_component::<Collider>(entity2);
        if col2 == None {return false}
        let col2 = col2.unwrap();
        let pos1 = Collider::get_pos(ecs, entity1);
        let pos2 = Collider::get_pos(ecs, entity2);
        for c1 in col1.col{
            if c1 == None {continue}
            let c1 = c1.unwrap();
            for c2 in col2.col{
                if c2 == None {continue}
                let c2 = c2.unwrap();
                if BoxCollider::check_collision(c1, c2, pos1, pos2) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_bound_offset(&self, bound_type: BoundType) -> Vec2{
        let mut ans = v2!(0., 0.);
        let mut ans_set = false;
        let comp =
            if bound_type == BoundType::Left || bound_type == BoundType::Down {
                |val1, val2| {val1 < val2}
            }
            else{
                |val1, val2| {val1 > val2}
            };
        for col in self.col{
            if col == None {continue}
            let bound =  col.unwrap().get_bound_offset(bound_type);
            if !ans_set{
                ans = bound;
                ans_set = true;
                continue;
            }
            if bound_type.horizontal() {if comp(bound.x, ans.x) {ans = bound;}}
            else {if comp(bound.y, ans.y) {ans = bound;}}
        }
        ans
    }

    pub fn get_bound(&self, ecs: &ECS, entity_id: usize, bound_type: BoundType) -> Vec2{
        let pos = Collider::get_pos(ecs, entity_id);
        pos + self.get_bound_offset(bound_type)
    }
}

impl Draw for Collider{
    fn draw(&self, ctx: &mut Context, _ecs: &ECS, _assets: &Assets, _entity_id: usize, pos: Vec2, screen_size: Screen2) -> GameResult {
        for col in self.col{
            if col == None {continue}

            col.unwrap().draw(ctx, _ecs, _assets, _entity_id, pos, screen_size)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
mod tests {
    use super::*;
    use crate::v2;
    #[test]
    fn bound_test(){
        let box_col = BoxCollider::new(v2!(10., 10.)).with_offset(v2!(-5., 0.));
        let col = Collider::new_single(box_col);
        assert_eq!(col.get_bound_offset(BoundType::Left), v2!(-15.0, 0.0));
    }
    #[test]
    fn bound_test_2(){
        let box_col_1 = BoxCollider::new(v2!(5., 5.));//.with_offset(v2!(-5., 0.));
        let box_col_2 = BoxCollider::new(v2!(10., 10.));//.with_offset(v2!(-5., 0.));
        let col = Collider::new_double(box_col_1, box_col_2);
        assert_eq!(col.get_bound_offset(BoundType::Right), v2!(10.0, 0.0));
    }
}