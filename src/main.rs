use fltk::{
    app, button::*, enums::{Align, Color, Font, FrameType}, frame::{self, Frame}, group::{Flex, Tabs}, text::{TextBuffer, TextDisplay},
     input::Input, menu::{Choice, MenuButton}, output::Output, prelude::{GroupExt, InputExt, MenuExt, WidgetBase, WidgetExt, WindowExt}, window::Window,

     button::Button,
     prelude::*,

};

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use hello_world::greeter_client::GreeterClient;
use std::cell::RefCell;
use std::rc::Rc;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

pub struct MyEvent;

impl MyEvent {
    const CHANGED: i32 = 40;
}

#[derive(Clone)]
pub struct Counter {
    count: Rc<RefCell<i32>>,
}

impl Counter {
    #[must_use]
    pub fn new(val: i32) -> Self {
        Counter {
            count: Rc::from(RefCell::from(val)),
        }
    }

    pub fn increment(&mut self) {
        *self.count.borrow_mut() += 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    pub fn decrement(&mut self) {
        *self.count.borrow_mut() -= 1;
        app::handle_main(MyEvent::CHANGED).unwrap();
    }

    #[must_use]
    pub fn value(&self) -> i32 {
        *self.count.borrow()
    }
}

const GRAY: Color = Color::from_hex(0x757575);

#[derive(Clone, Debug, Default)]
pub struct SharedData {
    shared: Arc<Shared>,
}

#[derive(Debug, Default)]
struct Shared {
    out: Mutex<TextBuffer>,
    frame: Mutex<Frame>
}


impl SharedData {
    pub fn new(oo: TextBuffer, ff: Frame) -> Self {
        Self {
            shared: Arc::new(Shared {
                out: Mutex::new(oo),
                frame: Mutex::new(ff)
            })
        }
    }

    pub fn update(&self, key: String)  {
        let mut oo3 = self.shared.out.lock().unwrap();
        oo3.append(&key);
        oo3.append("\n");
    }

    pub fn count(&self)  {
        let mut oo3 = self.shared.frame.lock().unwrap();
        let label = (oo3.label().parse::<i32>().unwrap() + 1).to_string();
        oo3.set_label(&label);
    }
}

#[derive(Debug, Default)]
pub struct MyGreeter {
    data: SharedData,
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
        self.data.update(format!("Got a request: {:?}", request));
        self.data.count();

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
async fn server(out1: TextBuffer, ff1: Counter) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let mut greeter = MyGreeter::default();

    greeter.data = SharedData::new(out1, ff1);

    let svc = GreeterServer::with_interceptor(greeter, intercept);

    Server::builder()
        .add_service(svc)
        .serve(addr)
        .await?;

    Ok(())
}

#[tokio::main]
async fn req_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic23123123".into(),
    });

    println!("request={:?}", request);

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

fn draw_gallery() -> (TextBuffer, Frame, Counter) {
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


    let mut _but6 = ReturnButton::default().with_label("Return");
    col.end();
    grp1.end();

    let mut grp2 = Flex::default_fill().with_label("Server\t\t").row();

    let mut col2 = Flex::default().column();
    grp2.fixed(&col2, 40);
    col2.set_pad(10);
    col2.set_margin(10);

    let counter = Counter::new(0);

    let mut count = frame::Frame::default()
    .with_label(&counter.value().to_string())
    .with_align(Align::RightTop);

    col2.end();

    let mut col2 = Flex::default().column();
    grp2.fixed(&col2, 400);
    col2.set_pad(10);
    col2.set_margin(10);

    count.set_label_size(36);
    count.set_label_color(GRAY);



    let mut disp = TextDisplay::new(5, 5, 390, 250, None);

    let mut buf = TextBuffer::default();
    buf.append("Initiating app\n");
    disp.set_buffer(buf.clone());

    // let out: Output = Output::default();
    

    _but6.set_callback(move |_| {

        let _ = thread::spawn(|| {
            let _= req_client();
        });
       
    });

    col2.end();
    grp2.end();
    tab.end();
    tab.auto_layout();

    (buf, count, counter)
}

fn main() {

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("fltk grps rust")
        .center_screen();

    let (oo4, mut fr5, cn6) = draw_gallery();

    fr5.handle(move |f, ev| {
        if ev == MyEvent::CHANGED.into() {
            f.set_label(&cn6.clone().value().to_string());
            true
        } else {
            false
        }
    });

    thread::spawn(move || {
        let _ = server(oo4, cn6);
    });



    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}