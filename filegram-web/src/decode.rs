use filegram::decode;
use gloo::utils::document;
use gloo_file::{callbacks::FileReader, Blob, File, ObjectUrl};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::prelude::*;

type FileName = String;

pub enum Msg {
    LoadedBytes(FileName, Vec<u8>),
    Files(Vec<File>),
}

pub struct DecodeComponent {
    readers: HashMap<FileName, FileReader>,
}

impl Component for DecodeComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
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
            <div class="component decode">
                <div>
                    <h2>{"Choose a file to decode from an image:"}</h2>
                </div>
                <div>
                    <input type="file" accept="image/png" onchange={on_change} multiple=false/>
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
                let file_contents = Self::decode(data);
                Self::download_file(&file_name, &file_contents);
                self.readers.remove(&file_name);
                true
            }
        }
    }
}

impl DecodeComponent {
    fn download_file(name: &str, data: &[u8]) {
        let name = name.to_owned();
        let file_name = if let Some(file_name) = name.strip_suffix(".png") {
            file_name
        } else {
            &name
        };
        let blob = Blob::new(data);
        let blob_url = ObjectUrl::from(blob);
        let download_element = document().create_element("a").unwrap();
        download_element
            .set_attribute("href", &blob_url.to_string())
            .unwrap();
        download_element
            .set_attribute("download", file_name)
            .unwrap();
        download_element.dyn_into::<HtmlElement>().unwrap().click();
    }

    fn decode(data: Vec<u8>) -> Vec<u8> {
        let cursor = std::io::Cursor::new(data);
        decode::from_file(cursor)
    }
}
