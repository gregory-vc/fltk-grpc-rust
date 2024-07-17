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

pub mod hello_world {
    tonic::include_proto!("helloworld");
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
async fn server(out1: TextBuffer, ff1: Frame) -> Result<(), Box<dyn std::error::Error>> {
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
async fn req_client(ss2: Arc<Input>) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: ss2.value().into(),
    });

    println!("request={:?}", request);

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

fn draw_gallery() -> (TextBuffer, Frame) {
    let mut tab = Tabs::default_fill();

    let mut grp1 = Flex::default_fill().with_label("Client\t\t").row();
    let mut col = Flex::default().column();
    grp1.fixed(&col, 160);
    col.set_pad(10);
    col.set_margin(10);
    // let _but1 = Button::default().with_label("Button");
    // let _but2 = RoundButton::default().with_label("Round");
    // let _but3 = CheckButton::default().with_label("Check");
    
    // let mut but5 = MenuButton::default().with_label("Type");
    // but5.add_choice("Text|Bool|Enum");
   
    let mut chce = Choice::default();
    chce.add_choice("Text");
    chce.add_choice("Bool");
    chce.add_choice("Enum");

    // let _inp = Input::default();


    let mut _but6 = ReturnButton::default().with_label("Return");
    col.end();


    let mut col = Flex::default().column();
    grp1.fixed(&col, 400);
    col.set_pad(10);
    col.set_margin(10);

    let mut inp = Input::default();
    inp.set_value("rust");

    col.end();
    grp1.end();

    

    let mut grp2 = Flex::default_fill().with_label("Server\t\t").row();

    let mut col2 = Flex::default().column();
    grp2.fixed(&col2, 70);
    col2.set_pad(10);
    col2.set_margin(10);

    let mut count = frame::Frame::default()
    .with_label("0")
    .with_align(Align::Center);

    col2.end();

    let mut col2 = Flex::default().column();
    grp2.fixed(&col2, 400);
    col2.set_pad(50);
    col2.set_margin(10);

    count.set_label_size(36);
    count.set_label_color(GRAY);

    let mut disp = TextDisplay::new(5, 5, 390, 250, None);

    let mut buf = TextBuffer::default();
    buf.append("Initiating app\n");
    disp.set_buffer(buf.clone());

    // let out: Output = Output::default();
    let wrapped_inp = Arc::new(inp);


    _but6.set_callback(move |_| {
        let inp = wrapped_inp.clone();
        
        let _ = thread::spawn(|| {
            let _= req_client(inp);
        });
       
    });

    col2.end();
    grp2.end();
    tab.end();
    tab.auto_layout();

    (buf, count)
}

fn main() {

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(700, 450)
        .with_label("fltk grps rust")
        .center_screen();

    let (oo4, fr5) = draw_gallery();

    thread::spawn(move || {
        let _ = server(oo4, fr5);
    });



    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}