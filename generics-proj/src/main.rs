enum DigitalContent {
    AudioFile,
    VideoFile,
}

struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        match self.content {
            DigitalContent::AudioFile => println!("Listening to the audio file"),
            DigitalContent::VideoFile => println!("Watching the video file"),
        }
    }
}

impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    let short_chat = ChatMessage {
        content: "Hello World",
        time: String::from("0hr 05m"),
    };
    println!("{}", short_chat.retrieve_time());

    let text_chat = ChatMessage {
        content: String::from("Super awesome file"),
        time: String::from("1hr 45m"),
    };
    println!("{}", text_chat.retrieve_time());

    let video_chat = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("0hr 32m"),
    };
    video_chat.consume_entertainment();
}
