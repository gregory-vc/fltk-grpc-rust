use fltk::{
    app,
    button::*,
    group::{Flex, Tabs},
    input::Input,
    menu::{Choice, MenuButton},
    output::Output,
    prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
    frame,
    enums::{Align, Color, Font, FrameType},
};

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

const GRAY: Color = Color::from_hex(0x757575);

#[derive(Clone, Debug, Default)]
pub struct Outputer {
    shared: Arc<Shared>,
}

#[derive(Debug, Default)]
struct Shared {
    out: Mutex<Output>
}


impl Outputer {
    pub fn new(oo: Output) -> Self {
        Self {
            shared: Arc::new(Shared {
                out: Mutex::new(oo)
            })
        }
    }

    pub fn update(&self, key: String)  {
        let mut oo3 = self.shared.out.lock().unwrap();
        oo3.set_value(&key)
    }
}

#[derive(Debug, Default)]
pub struct MyGreeter {
    out: Outputer,
}

// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, 
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
        self.out.update(format!("Got a request: {:?}", request));

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply)) 
    }
}

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

#[tokio::main]
async fn server(out1: Output) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut greeter = MyGreeter::default();

    greeter.out = Outputer::new(out1);

    let svc = GreeterServer::with_interceptor(greeter, intercept);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

fn draw_gallery() -> Output {
    let mut tab = Tabs::default_fill();

    let mut grp1 = Flex::default_fill().with_label("Client\t\t").row();
    let mut col = Flex::default().column();
    grp1.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);
    // let _but1 = Button::default().with_label("Button");
    // let _but2 = RoundButton::default().with_label("Round");
    // let _but3 = CheckButton::default().with_label("Check");
    // let _but4 = LightButton::default().with_label("Light");
    // let mut but5 = MenuButton::default().with_label("Menu");
    // but5.add_choice("Hello|World|From|Rust");
   
    // let mut chce = Choice::default();
    // chce.add_choice("Hello");
    // let _inp = Input::default();
    let mut count = frame::Frame::default()
        .with_label("0")
        .with_align(Align::Top | Align::Inside);

    count.set_label_size(36);
    count.set_label_color(GRAY);

    let mut _but6 = ReturnButton::default().with_label("Return");
    col.end();
    grp1.end();

    let mut grp2 = Flex::default_fill().with_label("Server\t\t").row();
    let mut col2 = Flex::default().column();
    grp2.fixed(&col2, 400);
    col2.set_pad(10);
    col2.set_margin(10);

    let out = Output::default();

   
    // out1.set(out);

    _but6.set_callback(move |_| {
        let label = (count.label().parse::<i32>().unwrap() + 1).to_string();
        count.set_label(&label);
    });

    col2.end();
    grp2.end();
    tab.end();
    tab.auto_layout();

    return out;
}

fn main() {

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("fltk grps rust")
        .center_screen();

    let oo4 = draw_gallery();

    thread::spawn(move || {
        let _ = server(oo4);
    });



    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}