use crate::prelude::*;

const LEFT_KEY: KeyCode = ggez::event::KeyCode::Left;
const RIGHT_KEY: KeyCode = ggez::event::KeyCode::Right;
const UP_KEY: KeyCode = ggez::event::KeyCode::Up;
const DOWN_KEY: KeyCode = ggez::event::KeyCode::Down;

const RSHIFT_KEY: KeyCode = ggez::event::KeyCode::RightShift; // Run
const SPACE_KEY: KeyCode = ggez::event::KeyCode::Space; // Open
const CTRL_KEY: KeyCode = ggez::event::KeyCode::LeftControl; // Fire --> Jump
const ALT_KEY: KeyCode = ggez::event::KeyCode::LeftAlt; // Strafe --> NA

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    None = 0,
    LookLeft,
    LookRight,
    MoveForward,
    MoveBackward,
    Run,
    Open,
    Jump,
    Strafe,
}

pub struct Player {
    pub pos: Vector3f, // position (vector "pos")
    pub dir: Vector2f,  //  direction (vector "dir")
    pub plane: Vector2f, // camera plane (vector "plane")
    pub frame: u32,

    pub gravity: f32,
    pub velocity: Vector3f,
    pub rotation: Vector2f,
    pub action: Action,
}

impl Player {
    pub fn new(pos: Vector3f) -> Player {
        let mut player = Player {
            pos,
            dir: Vector2f::new(1.0, 0.0),
            plane: Vector2f::new(0.0, 0.66),
            frame: 0,
            gravity: -3.8,
            velocity: Vector3f::default(),
            rotation: Vector2f::default(),
            action: Action::None,
        };
        player.update_dir_plane(std::f32::consts::PI / 2.0, 1.0);
        player
    }

    pub fn handle_inputs(&mut self, keycode: KeyCode, pressed: bool) {
        println!("---------------------------------------");
        println!("pos x: {}", self.pos.x);
        println!("pos y: {}", self.pos.y);
        println!("pos z: {}", self.pos.z);
        println!("dir x: {}", self.dir.x);
        println!("dir y: {}", self.dir.y);
        println!("plane x: {}", self.plane.x);
        println!("plane y: {}", self.plane.y);

        if keycode == UP_KEY {
            if pressed {
                if self.action == Action::None {
                    self.velocity.x = 4.0;
                    self.action = Action::MoveForward;
                }
            } else {
                self.velocity.x = 0.0;
                self.action = Action::None;
            }
        } else if keycode == DOWN_KEY {
            if pressed {
                if self.action == Action::None {
                    self.velocity.x = -4.0;
                    self.action = Action::MoveBackward;
                }
            } else {
                self.velocity.x = 0.0;
                self.action = Action::None;
            }
        } else if keycode == LEFT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.rotation.x = -3.5;
                    self.action = Action::LookLeft;
                }
            } else {
                self.rotation.x = 0.0;
                self.action = Action::None;
            }
        } else if keycode == RIGHT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.rotation.x = 3.5;
                    self.action = Action::LookRight;
                }
            } else {
                self.rotation.x = 0.0;
                self.action = Action::None;
            }
        } else if keycode == RSHIFT_KEY {
            if pressed {
                if self.action == Action::MoveForward || self.action == Action::MoveBackward {
                    self.velocity.x *= 2.0;
                    self.action = Action::Run;
                } else if self.action == Action::LookLeft || self.action == Action::LookRight {
                    self.rotation.x *= 2.0;
                    self.action = Action::Run;
                }
            } else {
                self.velocity.x /= 2.0;
                self.rotation.x /= 2.0;
                self.action = Action::None;
            }
        } else if keycode == ALT_KEY {
            if pressed {
                //if self.action == Action::LookLeft {
                    self.rotation.x = -1.0;
                    self.velocity.x = 1.0;
                //} else if self.action == Action::LookRight {
                //    self.rotation.x = 1.0;
                //    self.velocity.x = 1.0;
                //}
            } else {
                self.velocity.x = 0.0;
                self.rotation.x = 0.0;
                self.action = Action::None;
            }
        } else if keycode == CTRL_KEY {
            if pressed && self.velocity.z == 0.0 {
                self.velocity.z = 1.65;
                self.action = Action::Jump;
            } else {
                self.action = Action::None;
            }
        } else if keycode == SPACE_KEY {
            self.action = Action::Open;
        }
    }

    fn update_gravity(&mut self, map: &Map, delta: f32) {
        let mut future_z = self.pos.z + self.velocity.z * delta;
        let inside_wall = map.get(&self.pos);
        let future_under_wall = map.get(&Vector3f::new(self.pos.x, self.pos.y, future_z));

        self.velocity.z += self.gravity * delta;
        self.pos.z = if future_z < 0.0 {
            self.velocity.z = 0.0;
            0.0
        } else {
            if let Cell::Wall { value: _, height } = inside_wall {
                if future_z <= self.pos.z.floor() + *height {
                    future_z = self.pos.z.floor() + *height;
                    self.velocity.z = 0.0;
                }
            } else if let Cell::Empty = inside_wall {
                if future_z <= future_z.floor() + future_under_wall.height() {
                    future_z = future_z.floor() + future_under_wall.height();
                    self.velocity.z = 0.0;
                }
            }
            future_z
        }
    }

    fn move_x_in_thin_wall(
        &mut self,
        x_coord: f32,
        new_x: f32,
        dir: Direction,
        slide: f32,
        depth: f32,
    ) {
        if dir.is_under_light() {
            self.pos.x = new_x;
        } else {
            let limit = x_coord
                + if dir == Direction::East {
                    depth
                } else {
                    1.0 - depth
                };
            let relative_pos_y = self.pos.y - self.pos.y.floor();

            if (self.pos.x > limit && new_x < limit || self.pos.x < limit && new_x > limit)
                && relative_pos_y < slide
            {
                return;
            }
            self.pos.x = new_x;
        }
    }

    fn move_y_in_thin_wall(
        &mut self,
        y_coord: f32,
        new_y: f32,
        dir: Direction,
        slide: f32,
        depth: f32,
    ) {
        if !dir.is_under_light() {
            self.pos.y = new_y;
        } else {
            let limit = y_coord
                + if dir == Direction::North {
                    depth
                } else {
                    1.0 - depth
                };
            let relative_pos_x = self.pos.x - self.pos.x.floor();

            if (self.pos.y > limit && new_y < limit || self.pos.y < limit && new_y > limit)
                && relative_pos_x < slide
            {
                return;
            }
            self.pos.y = new_y;
        }
    }

    fn update_pos(&mut self, map: &mut Map, delta: f32) {
        let speed = self.velocity.x * delta;
        let new_x = self.pos.x + self.dir.x * speed;
        let new_y = self.pos.y + self.dir.y * speed;

        match map.portals_at(
            Vector3f::new(new_x.floor(), new_y.floor(), self.pos.z.floor()),
            Direction::None,
        ) {
            None => {
                match map.get(&Vector3f::new(new_x, self.pos.y, self.pos.z)) {
                    Cell::Empty => match map.get(&self.pos) {
                        Cell::Thin(thin) => self.move_x_in_thin_wall(
                            self.pos.x.floor(),
                            new_x,
                            thin.dir(),
                            thin.slide(),
                            thin.depth(),
                        ),
                        _ => self.pos.x = new_x,
                    },
                    Cell::Wall { value: _, height } => {
                        if self.pos.z >= self.pos.z.floor() + *height {
                            self.pos.x = new_x
                        }
                    }
                    Cell::Thin(thin) => self.move_x_in_thin_wall(
                        new_x.floor(),
                        new_x,
                        thin.dir(),
                        thin.slide(),
                        thin.depth(),
                    ),
                }

                match map.get(&Vector3f::new(self.pos.x, new_y, self.pos.z)) {
                    Cell::Empty => match map.get(&self.pos) {
                        Cell::Thin(thin) => self.move_y_in_thin_wall(
                            self.pos.y.floor(),
                            new_y,
                            thin.dir(),
                            thin.slide(),
                            thin.depth(),
                        ),
                        _ => self.pos.y = new_y,
                    },
                    Cell::Wall { value: _, height } => {
                        if self.pos.z >= self.pos.z.floor() + *height {
                            self.pos.y = new_y
                        }
                    }
                    Cell::Thin(thin) => self.move_y_in_thin_wall(
                        new_y.floor(),
                        new_y,
                        thin.dir(),
                        thin.slide(),
                        thin.depth(),
                    ),
                }
            }
            Some((first, second)) => {
                let source = first.unwrap();
                let dest = second.unwrap();

                let tmp = self.pos;
                self.update_dir_plane(dest.link_dir(source), 1.0);
                self.pos.x = dest.link_x(source, &tmp) + self.dir.x * speed;
                self.pos.y = dest.link_y(source, &tmp) + self.dir.y * speed;
                self.pos.z = dest.pos.z;
            }
        }
    }

    fn update_dir_plane(&mut self, mut new_rotation: f32, delta: f32) {
        new_rotation *= delta;
        self.dir.rotate(new_rotation);
        self.plane.rotate(new_rotation);
    }
    fn update_dir_only(&mut self, mut new_rotation: f32, delta: f32) {
        if self.action != Action::Strafe {
            self.action = Action::Strafe;
            new_rotation *= delta;
            self.dir.rotate(new_rotation);
            //self.plane.rotate(new_rotation);
        }
    }
    pub fn update(&mut self, map: &mut Map, delta: f32) {
        if self.action == Action::Open {
            let hit = crate::engine::rayobject::Ray::new(&self, Vector2f::default()).cast(map);

            if let Some(_) = hit.value {
                if hit.dist <= 1.5 {
                    map.get_mut(&hit.pos).trigger();
                }
            }
            self.action = Action::None;
        }
        if self.velocity.z != 0.0 || self.pos.z > 0.0 {
            self.update_gravity(map, delta);
        }
        if self.velocity.x != 0.0 && self.rotation.x == 0.0 {
            self.update_pos(map, delta);
        }
        else if self.velocity.x == 0.0 && self.rotation.x != 0.0 {
            self.update_dir_plane(self.rotation.x, delta);
        }
        else if self.velocity.x != 0.0 && self.rotation.x != 0.0 {
            self.update_pos(map, delta);
            self.update_dir_only(std::f32::consts::PI / 4.0, 1.0);
        }
    }
}
