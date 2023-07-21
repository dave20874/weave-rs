mod mesh;

use iced::executor;
use iced::{Application, Command, Element, Settings, Theme, Color, Rectangle, Length};
use iced_aw::number_input::NumberInput;

use iced::widget::{column, row, pick_list };
use iced::widget::canvas::{Canvas, Program, Path, Cursor, Geometry, Frame, Stroke, LineJoin};
use iced::widget::Container;
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;

use mesh::Mesh2D;

pub fn main() -> iced::Result {
    let mut s = Settings::default();
    s.antialiasing = true;
    Weave::run(s)
}



#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Surface {
    Plane,
    Sphere,
}

impl Surface {
    const ALL: [Surface; 2] = [
        Surface::Plane,
        Surface::Sphere,
    ];
}

impl Display for Surface {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            Surface::Plane => {
                write!(f, "Plane")
            },
            Surface::Sphere => {
                write!(f, "Sphere")
            },
        } 
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartMeshPlanar {
    Square,
    Pentagon,
    Hexagon,
    SquareGrid,
}

impl StartMeshPlanar {
    const ALL: [StartMeshPlanar; 4] = [
        StartMeshPlanar::Square,
        StartMeshPlanar::Pentagon,
        StartMeshPlanar::Hexagon,
        StartMeshPlanar::SquareGrid,
    ];
}

impl Display for StartMeshPlanar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            StartMeshPlanar::Square => {
                write!(f, "Square")
            },
            StartMeshPlanar::Pentagon => {
                write!(f, "Pentagon")
            },
            StartMeshPlanar::Hexagon => {
                write!(f, "Hexagon")
            },
            StartMeshPlanar::SquareGrid => {
                write!(f, "Square grid")
            },
        } 
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StartMeshSpherical {
    Cube,
    Dodecahedron,
    Icosohedron,
    TruncatedIcosohedron,
}

impl Display for StartMeshSpherical {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            StartMeshSpherical::Cube => {
                write!(f, "Cube")
            },
            StartMeshSpherical::Dodecahedron => {
                write!(f, "Dodecahedron")
            },
            StartMeshSpherical::Icosohedron => {
                write!(f, "Icosohedron")
            },
            StartMeshSpherical::TruncatedIcosohedron => {
                write!(f, "Truncated icosohedron")
            },
        } 
    }
}

impl StartMeshSpherical {
    const ALL: [StartMeshSpherical; 4] = [
        StartMeshSpherical::Cube,
        StartMeshSpherical::Dodecahedron,
        StartMeshSpherical::Icosohedron,
        StartMeshSpherical::TruncatedIcosohedron,
    ];
}

#[derive(Debug, Clone)]
struct Weave {
    surface: Surface,
    start_mesh_planar: StartMeshPlanar,
    start_mesh_sphere: StartMeshSpherical,
    iterations: usize,
}

#[derive(Debug, Clone)]
enum Message {
    SetSurface(Surface),
    SetStartMeshPlanar(StartMeshPlanar),
    SetStartMeshSpherical(StartMeshSpherical),
    SetIterations(usize),
}



impl Weave {
    fn surface_control(&self) -> Element<Message> {
        let pick_list = pick_list(
            &Surface::ALL[..],
            Some(self.surface),
            Message::SetSurface,
        );

        let content = column![
            "Surface geometry:",
            pick_list,
        ];

        Container::new(
            content,
        ).into()
    }

    fn initial_shape_control(&self) -> Element<Message> {
        let pick_list: Element<Message> = match self.surface {
            Surface::Plane => {
                pick_list(
                    &StartMeshPlanar::ALL[..],
                    Some(self.start_mesh_planar),
                    Message::SetStartMeshPlanar,
                ).into()
            }
            Surface::Sphere => {
                pick_list(
                    &StartMeshSpherical::ALL[..],
                    Some(self.start_mesh_sphere),
                    Message::SetStartMeshSpherical,
                ).into()
            }
        };

        let content = column![
            "Initial mesh:",
            pick_list,
        ];

        Container::new(
            content,
        ).into()
    }

    fn iter_controls(&self) -> Element<Message> {
        let input = NumberInput::new(self.iterations, 7, Message::SetIterations);

        let content = column![
            "Iterations",
            input,
        ];

        Container::new(
            content,
        ).into()
    }

    fn controls(&self) -> Element<Message> {
        Container::new(
            column![]
                .spacing(20)
                // Surface pick list
                .push(self.surface_control())

                // Initial polygon
                .push(self.initial_shape_control())

                // Iterations number input
                .push(self.iter_controls())
        )
            .max_width(200)
            .into()
    }

    fn planar_view(&self) -> Element<Message> {
        // return a canvas.
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn make_mesh(&self) -> Mesh2D {
        let mut mesh = match self.start_mesh_planar {
            StartMeshPlanar::Square => Mesh2D::square_grid(1, 1),
            StartMeshPlanar::Pentagon => Mesh2D::regular_polygon(5),
            StartMeshPlanar::Hexagon => Mesh2D::regular_polygon(6),
            StartMeshPlanar::SquareGrid => Mesh2D::square_grid(3, 3),
        };
        
        for _ in 0..self.iterations {
            mesh = mesh.penta_decomp();
        }

        mesh
    }
}

// Then, we implement the `Program` trait
impl Program<Message, Theme> for Weave {
    type State = ();

        // TODO: This gets called every time mouse moves?  Do I need to optimize it?  Supress that?
    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{

        // create start mesh
        let mesh = self.make_mesh();
        // println!("The mesh has {} polygons.", mesh.num_polygons());

        let (min_x, min_y, max_x, max_y) = mesh.extents();

        // We prepare a new `Frame`
        let mut frame = Frame::new(bounds.size());

        // scale and translate the frame so (0, 0) is a the center, +X is right, +Y is up
        // and the mesh fits inside 90% of the frame.
        let scale_x: f32 = if bounds.size().width > bounds.size().height {
            0.9 * bounds.size().height/(max_x - min_x)
        }
        else {
            0.9 * bounds.size().width/(max_y - min_y)
        };
        let scale_y = -scale_x;

        let cx = (bounds.size().width/2 as f32) - scale_x * (max_x + min_x)/2.0;
        let cy = (bounds.size().height/2 as f32) -  scale_y * (max_y + min_y)/2.0;

        let poly_path = Path::new(|b| {
            mesh.build(b, scale_x, scale_y, cx, cy);
        });

        let stroke = Stroke::default()
        .with_color(Color::from_rgb(0.0, 0.0, 1.0))  // Blue
        .with_width(1.0)
        .with_line_join(LineJoin::Round);

        frame.stroke(&poly_path, stroke);

        // Finally, we produce the geometry
        vec![frame.into_geometry()]
    }
}

impl Application for Weave {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Weave, Command<Self::Message>) {
        (
            Weave {
                surface: Surface::Plane,
                start_mesh_planar: StartMeshPlanar::Pentagon,
                start_mesh_sphere: StartMeshSpherical::Dodecahedron,
                iterations: 0,
            }, 
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Geometric weave generator")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::SetSurface(surface) => {
                self.surface = surface;
            }
            Message::SetIterations(n) => {
                self.iterations = n;
            }
            Message::SetStartMeshPlanar(start_mesh_planar) => {
                self.start_mesh_planar = start_mesh_planar;
            }
            Message::SetStartMeshSpherical(start_mesh_sphere) => {
                self.start_mesh_sphere = start_mesh_sphere;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {

        // TODO : Clarify this logic and add real controls for weaver.
        row![]
            .spacing(50)
            .padding(20)
                .push(self.controls())
                .push(self.planar_view(),
            )
            .into()
    }
}



