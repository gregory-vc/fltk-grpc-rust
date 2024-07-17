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

fn draw_gallery()  {
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

    let wrapped_inp = Arc::new(inp);
    _but6.set_callback(move |_| {
        let inp = wrapped_inp.clone();
        
        let _ = thread::spawn(|| {
            let _= req_client(inp);
        });
       
    });

    col.end();
    grp1.end();

    
    tab.end();
    tab.auto_layout();

}

fn main() {

    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(700, 450)
        .with_label("fltk grps rust")
        .center_screen();

    draw_gallery();



    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}