use base64::{engine::general_purpose, Engine as _};
use filegram::encode;
use gloo_file::{callbacks::FileReader, File};
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use web_sys::{Event, HtmlInputElement};
use yew::prelude::*;

pub enum Msg {
    LoadedBytes(String, Vec<u8>),
    Files(Vec<File>),
}

type FileName = String;
type Data = String;

pub struct FilegramComponent {
    files: Vec<(FileName, Data)>,
    readers: HashMap<FileName, FileReader>,
}

impl Component for FilegramComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: Vec::new(),
            readers: HashMap::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(move |e: Event| {
            let mut selected_files = Vec::new();
            let input: HtmlInputElement = e.target_unchecked_into();
            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                selected_files.extend(files);
            }
            Msg::Files(selected_files)
        });

        html! {
            <div>
                <div>
                    <h2>{"Choose a file to encode as an image:"}</h2>
                </div>
                <div>
                    <input type="file" onchange={on_change} multiple=false/>
                </div>
                <div>
                { for self.files.iter().map(|(n,d)| Self::view_file(n,d))}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::LoadedBytes(file_name, data) => {
                let image = Self::encode(data);
                let image_data = general_purpose::STANDARD_NO_PAD.encode(image);
                self.files.push((file_name.clone(), image_data));
                self.readers.remove(&file_name);
                true
            }
        }
    }
}

impl FilegramComponent {
    fn view_file(name: &str, data: &str) -> Html {
        let img = format!("data:image/png;base64,{}", data.to_string());
        html! {
            <div class="img">
                <p>{name}</p>
                <img src={img}/>
            </div>
        }
    }

    fn encode(data: Vec<u8>) -> Vec<u8> {
        let img = encode::from_slice(&data);

        let mut cursor = std::io::Cursor::new(Vec::new());
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

        cursor.seek(SeekFrom::Start(0)).unwrap();
        let mut out = Vec::new();
        cursor.read_to_end(&mut out).unwrap();

        out
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
        <div>
            <h1>{"Filegram image app"}</h1>
            <FilegramComponent/>
        </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
