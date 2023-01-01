use iced::{
    widget::{button, column, Container},
    Application, Command, Length, Settings,
};
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

struct Audio {
    _stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
    sink: Arc<rodio::Sink>,
}

impl Default for Audio {
    fn default() -> Self {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = Arc::new(rodio::Sink::try_new(&handle).unwrap());
        Self {
            _stream,
            handle,
            sink,
        }
    }
}

#[derive(Default)]
struct App {
    audio: Audio,
}

#[derive(Debug, Clone)]
enum Message {
    Ping,
    Ignore,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Sound example")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Ping => {
                return Command::perform(play(self.audio.sink.clone()), |_| Message::Ignore);
            }
            Message::Ignore => (),
        };
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        Container::new(column![button("Sound").on_press(Message::Ping)])
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .padding(15)
            .into()
    }
}

async fn play(sink: Arc<rodio::Sink>) {
    tokio::task::spawn_blocking(move || {
        // using tokio::fs::File will not work
        let file = File::open("sfx/sfx_1.wav").unwrap();

        sink.append(Decoder::new(BufReader::new(file)).unwrap());
        sink.set_volume(1.0);

        println!("Sound playing...");

        sink.sleep_until_end();

        println!("Done!");
    })
    .await
    .unwrap();
}

fn main() {
    App::run(Settings::default()).unwrap();
}
