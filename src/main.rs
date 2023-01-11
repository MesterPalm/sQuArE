use std::f32::consts::PI;
use std::ops::{self, Add};

use sfml::graphics::{Color, RectangleShape, CircleShape, RenderWindow, RenderTarget, RenderStates, PrimitiveType, Vertex, Drawable};
use sfml::window::{mouse, ContextSettings, Event, Key, Style};
use sfml::system::{Clock, Vector2f};

#[derive(Debug, Clone, Copy)]
struct Box {
    center : Point3d,
    dimensions : Point3d,
    rotations : Point3d
}

impl Drawable for Box {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target : &mut dyn RenderTarget,
        states : &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let point1 = self.center + self.dimensions/2.;
        let point2 = point1 - Point3d{x : self.dimensions.x, y : 0., z : 0.};
        let point3 = point1 - Point3d{x : 0., y : self.dimensions.y, z : 0.};
        let point4 = point1 - Point3d{x : self.dimensions.x, y : self.dimensions.y, z : 0.};
        let point5 = point1 - Point3d{x : 0., y : 0., z : self.dimensions.z};
        let point6 = point1 - Point3d{x : self.dimensions.x, y : 0., z : self.dimensions.z};
        let point7 = point1 - Point3d{x : 0., y : self.dimensions.y, z : self.dimensions.z};
        let point8 = point1 - Point3d{x : self.dimensions.x, y : self.dimensions.y, z : self.dimensions.z};


        //let line1 = 
    }
}

#[derive(Debug, Clone, Copy)]
struct Point3d {
    x : f32,
    y : f32,
    z : f32
}

impl ops::Add<Point3d> for Point3d {
    type Output = Point3d;

    fn add(self, rhs : Point3d) -> Point3d {
        return Point3d{x : self.x + rhs.x, y : self.y + rhs.y, z : self.z + rhs.z};
    }
}

impl ops::Sub<Point3d> for Point3d {
    type Output = Point3d;

    fn sub(self, rhs : Point3d) -> Point3d {
        return Point3d{x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z};
    }
}

impl ops::Div<f32> for Point3d {
    type Output = Point3d;
    
    fn div(self, rhs : f32) -> Point3d {
        return Point3d{x : self.x/rhs, y : self.y/rhs, z : self.z/rhs};
    }

}

impl Point3d {
    fn origin() -> Point3d {
        return Point3d{x : 0., y : 0., z : 0.};
    }

    fn project_2d(&self, fov : f32) -> (f32, f32) {
        // return (x,y) as portion of view 0 in the middle, -1 to the left/up and 1 right/down 
        // Assume camera at (0,0,0) and looking straight into the z line
        let theta = (PI - fov) / 2.; // angle of edge of fov area to x axis
        println!("theta = {}", theta);
        let hypot = self.z/theta.sin(); // hypothenuse 
        println!("hypot = {}", hypot);
        let view = hypot*theta.cos();
        println!("view = {}", view);
        return (self.x/view, self.y/view);
    }

    fn rotate_around_x(&self, radians : f32, offset : &Point3d) -> Point3d {
        let off_y = self.y - offset.y;
        let off_z = self.z - offset.z;
        let rot_y = off_y*radians.cos() - off_z*radians.sin();
        let rot_z = off_y*radians.sin() + off_z*radians.cos();
        return Point3d{x : self.x, y : rot_y + offset.y, z : rot_z + offset.z};
    }
    fn rotate_around_y(&self, radians : f32, offset : &Point3d) -> Point3d {
        let off_x = self.x - offset.x;
        let off_z = self.z - offset.z;
        let rot_x = off_x*radians.cos() + off_z*radians.sin();
        let rot_z = -off_x*radians.sin() + off_z*radians.cos();
        return Point3d{x : rot_x + offset.x, y : self.y, z : rot_z + offset.z};
    }

    fn rotate_around_z(&self, radians : f32, offset : &Point3d) -> Point3d {
        let off_x = self.x - offset.x;
        let off_y = self.y - offset.y;
        let rot_x = off_x*radians.cos() - off_y*radians.sin();
        let rot_y = off_x*radians.sin() + off_y*radians.cos();
        return Point3d{x : rot_x - offset.x, y : rot_y - offset.y, z : self.z};
    }
}


struct Line {
    vertices : [Vertex; 4]
}

impl Line {
    fn new(x1 : f32, y1 : f32, x2 : f32, y2 : f32) -> Line {
        let mut line = Line{ vertices : [Vertex::default(); 4]};
        line.vertices = [
            Vertex::new(Vector2f::new(x1-1., y1-1.), Color::RED, Vector2f::new( 0.,  0.)),
            Vertex::new(Vector2f::new(x1+1., y1+1.), Color::RED, Vector2f::new( 0., 10.)),
            Vertex::new(Vector2f::new(x2+1., y2+1.), Color::RED, Vector2f::new(10., 10.)),
            Vertex::new(Vector2f::new(x2-1., y2-1.), Color::RED, Vector2f::new(10.,  0.))
        ];
        return line;
    }
}

impl Drawable for Line {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target : &mut dyn RenderTarget,
        states : &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        target.draw_primitives(&self.vertices, PrimitiveType::QUADS, &RenderStates::DEFAULT);
    }
}


fn process_window_events(window : &mut  RenderWindow) {
    // this function is litterally just for the purpose of
    // cleaning away this code, special designed should only 
    // be called in on place in main
    
    while let Some(event) = window.poll_event() {
        match event {
            // Resolve key presses and releases
            Event::Closed => {
                window.close();
            },
            Event::KeyReleased {code, ..} => {
            },
            Event::KeyPressed {code, ..} => {
                if code == Key::Q {
                    window.close()
                }
            },
            Event::MouseWheelScrolled {delta, ..} => {
            },
            _ => {}
        }
    }
}

fn main() {
    println!("In my talons i shape clay, crafting lifeforms as i please");
        
    let mut window = RenderWindow::new(
        (900, 900),
        "Confusing Platforms",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    window.set_framerate_limit(60);
    window.set_vertical_sync_enabled(true);


    // Theta, angle from x axis, phi angle from z axis
    //let radius_1 = 5.;
    //let mut theta_1 : f32 = PI/4.;
    //let mut phi_1 : f32 = PI/2.;
    //let radius_2 = 5.;
    //let mut theta_2 : f32 = 3.*PI/4.;
    //let mut phi_2 : f32 = PI/2.;
    //let radius_3 = 5.;
    //let mut theta_3 : f32 = 0.;
    //let mut phi_3 : f32 = 5.*PI/4.;
    //let radius_4 = 5.;
    //let mut theta_4 : f32 = 0.;
    //let mut phi_4 : f32 = 7.*PI/4.;
    //let depth = 10.;


    let mut p1 = Point3d{x : -5., y : -5., z : 20.};
    let mut p2 = Point3d{x : 5., y : -5., z : 20.};
    let mut p3 = Point3d{x : 5., y : 5., z : 20.};
    let mut p4 = Point3d{x : -5., y : 5., z : 20.};

    let mut left_held = false;
    let mut w_held = false;
    let mut a_held = false;
    let mut s_held = false;
    let mut d_held = false;
    let mut up_held = false;
    let mut down_held = false;
    let mut right_held = false;
    let mut pgd_held = false;
    let mut pgu_held = false;
    let mut angle_z : f32 = 0.;
    let mut angle_x : f32 = 0.;
    let mut camera = Point3d{x : 0., y : 0., z : 0.};
    // draw it
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                // Resolve key presses and releases
                Event::Closed => {
                    window.close();
                },
                Event::KeyReleased {code, ..} => {
                    if code == Key::W {
                        w_held = false;
                    }
                    if code == Key::A {
                        a_held = false;
                    }
                    if code == Key::S {
                        s_held = false;
                    }
                    if code == Key::D {
                        d_held = false;
                    }
                    if code == Key::LEFT {
                        left_held = false;
                    }
                    if code == Key::UP {
                        up_held = false;
                    }
                    if code == Key::DOWN {
                        down_held = false;
                    }
                    if code == Key::RIGHT {
                        right_held = false;
                    }
                    if code == Key::PAGEUP {
                        pgu_held = false;
                    }
                    if code == Key::PAGEDOWN {
                        pgd_held = false;
                    }
                },
                Event::KeyPressed {code, ..} => {

                    if code == Key::W {
                        w_held = true;
                    }
                    if code == Key::A {
                        a_held = true;
                    }
                    if code == Key::S {
                        s_held = true;
                    }
                    if code == Key::D {
                        d_held = true;
                    }
                    if code == Key::Q {
                        window.close()
                    }
                    if code == Key::LEFT {
                        left_held = true;
                    }
                    if code == Key::UP {
                        up_held = true;
                    }
                    if code == Key::DOWN {
                        down_held = true;
                    }
                    if code == Key::RIGHT {
                        right_held = true;
                    }
                    if code == Key::PAGEUP {
                        pgu_held = true;
                    }
                    if code == Key::PAGEDOWN {
                        pgd_held = true;
                    }
                },
                Event::MouseWheelScrolled {delta, ..} => {
                },
                _ => {}
            }
        }
        let depth = Point3d{x:0. , y:0., z : 0.};
        if w_held {
             camera = camera + Point3d{x : 0., y : 0., z : 0.5};
        }
        if s_held {
             camera = camera - Point3d{x : 0., y : 0., z : 0.5};
        }
        if a_held {
             camera = camera - Point3d{x : 0.5, y : 0., z : 0.};
        }
        if d_held {
             camera = camera + Point3d{x : 0.5, y : 0., z : 0.};
        }

        if left_held {
            p1 = p1.rotate_around_y(0.02, &camera);
            p2 = p2.rotate_around_y(0.02, &camera);
            p3 = p3.rotate_around_y(0.02, &camera);
            p4 = p4.rotate_around_y(0.02, &camera);
        }        

        if right_held {
            p1 = p1.rotate_around_y(-0.02, &camera);
            p2 = p2.rotate_around_y(-0.02, &camera);
            p3 = p3.rotate_around_y(-0.02, &camera);
            p4 = p4.rotate_around_y(-0.02, &camera);
        }

        if up_held {
            p1 = p1.rotate_around_x(0.02, &camera);
            p2 = p2.rotate_around_x(0.02, &camera);
            p3 = p3.rotate_around_x(0.02, &camera);
            p4 = p4.rotate_around_x(0.02, &camera);
        }

        if down_held {
            p1 = p1.rotate_around_x(-0.02, &camera);
            p2 = p2.rotate_around_x(-0.02, &camera);
            p3 = p3.rotate_around_x(-0.02, &camera);
            p4 = p4.rotate_around_x(-0.02, &camera);
        }

        if pgd_held {
            p1 = p1.rotate_around_z(0.02, &camera);
            p2 = p2.rotate_around_z(0.02, &camera);
            p3 = p3.rotate_around_z(0.02, &camera);
            p4 = p4.rotate_around_z(0.02, &camera);
        }

        if pgu_held {
            p1 = p1.rotate_around_z(-0.02, &camera);
            p2 = p2.rotate_around_z(-0.02, &camera);
            p3 = p3.rotate_around_z(-0.02, &camera);
            p4 = p4.rotate_around_z(-0.02, &camera);
        }

        //convert too screen coordinates
        let (x1, y1) = (p1-camera).project_2d(PI/2.);
        let (x1, y1) = (x1*(900./2.) + 900./2., y1*(900./2.) + 900./2.);
        let (x2, y2) = (p2-camera).project_2d(PI/2.);
        let (x2, y2) = (x2*(900./2.) + 900./2., y2*(900./2.) + 900./2.);
        let (x3, y3) = (p3 - camera).project_2d(PI/2.);
        let (x3, y3) = (x3*(900./2.) + 900./2., y3*(900./2.) + 900./2.);
        let (x4, y4) = (p4-camera).project_2d(PI/2.);
        let (x4, y4) = (x4*(900./2.) + 900./2., y4*(900./2.) + 900./2.);

        // setup lines
        let line1 : Line = Line::new(x1,y1,x2,y2);
        let line2: Line = Line::new(x2,y2,x3,y3);
        let line3 : Line = Line::new(x3,y3,x4,y4);
        let line4 : Line = Line::new(x4,y4,x1,y1);
        println!("{}, {}", x1, y1);
        println!("{}, {}", x2, y2);
        window.clear(Color::BLACK);
        window.draw(&line1);
        window.draw(&line2);
        window.draw(&line3);
        window.draw(&line4);
        window.display();
    }

}
