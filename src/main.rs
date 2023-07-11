use iced::executor;
use iced::{Application, Command, Element, Settings, Theme, Color, Rectangle, Length};
use iced_aw::number_input::NumberInput;

use iced::widget::{column, row, text, toggler, pick_list, canvas };
use iced::widget::canvas::{Canvas, Program, Path, Cursor, Geometry, Frame};
use iced::widget::{Container};
use std::fmt::Formatter;
use std::fmt::Error;
use std::fmt::Display;

pub fn main() -> iced::Result {
    Weave::run(Settings::default())
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum StartMeshSpherical {
    Cube,
    Dodecahedron,
    Icosohedron,
    TruncatedIcosohedron,
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
    // start_mesh_sphere: StartMeshSpherical,
    iterations: usize,
    views: usize,
}

#[derive(Debug, Clone)]
enum Message {
    SetSurface(Surface),
    SetStartMeshPlanar(StartMeshPlanar),
    // SetStartMeshSpherical(StartMeshSpherical),
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
        let pick_list = pick_list(
            &StartMeshPlanar::ALL[..],
            Some(self.start_mesh_planar),
            Message::SetStartMeshPlanar,
        );

        let content = column![
            "Initial mesh:",
            pick_list,
        ];

        Container::new(
            content,
        ).into()
    }

    fn iter_controls(&self) -> Element<Message> {
        let input = NumberInput::new(self.iterations, 10, Message::SetIterations);

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
}

// Then, we implement the `Program` trait
impl Program<Message, Theme> for Weave {
    type State = ();

    fn draw(&self, _state: &(), _theme: &Theme, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry>{
        // We prepare a new `Frame`
        let mut frame = Frame::new(bounds.size());

        // We create a `Path` representing a simple circle
        let circle = Path::circle(frame.center(), 100.0);

        // And fill it with some color
        frame.fill(&circle, Color::BLACK);

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
                // start_mesh_spherical: StartMeshSpherical::Dodecahedron,
                iterations: 2,
                views: 0,
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



