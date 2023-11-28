use filegram::decode;
use gloo_file::{callbacks::FileReader, Blob, File, ObjectUrl};
use gloo_utils::document;
use std::{collections::HashMap, path::Path};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::prelude::*;

type FileName = String;
type Data = Vec<u8>;

pub enum Msg {
    LoadedBytes(FileName, Vec<u8>),
    Files(Vec<File>),
}

pub struct DecodeComponent {
    files: Vec<(FileName, Data)>,
    readers: HashMap<FileName, FileReader>,
}

impl Component for DecodeComponent {
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
            <div class="component decode">
                <div>
                    <h2>{"Choose a file to decode from an image:"}</h2>
                </div>
                <div>
                    <input type="file" accept="image/png" onchange={on_change} multiple=false/>
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
                let file_contents = Self::decode(data);
                self.files.push((file_name.clone(), file_contents));
                self.readers.remove(&file_name);
                true
            }
        }
    }
}

const DEF_IMG_PATH: &str = "images/emblem-documents.png";
const ZIP_EXTENSIONS: [&str; 6] = ["zip", "tar", "gz", "xz", "jar", "7z"];
const ZIP_IMG_PATH: &str = "images/application-x-zip.png";
const IMG_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "svg", "webp"];
const IMG_IMG_PATH: &str = "images/image-x-generic.png";
const PDF_EXTENSION: [&str; 1] = ["pdf"];
const PDF_IMG_PATH: &str = "images/application-pdf.png";

impl DecodeComponent {
    fn view_file(name: &str, data: &[u8]) -> Html {
        let name = name.to_owned();
        let file_name = if let Some(file_name) = name.strip_suffix(".png") {
            file_name.to_owned()
        } else {
            name
        };
        let label = file_name.clone();
        let file_img = get_image_for_file(file_name.clone());
        let blob = Blob::new(data);
        let blob_url = ObjectUrl::from(blob);
        let on_click = Callback::from(move |_| {
            Self::download_file(&file_name, &blob_url);
        });

        html! {
            <div class="img">
                <div class="center">
                    <p>{label}</p>
                </div>
                <div class="center">
                    <img src={file_img}/>
                </div>
                <div class="center">
                    <button onclick={on_click}>{"Download"}</button>
                </div>
            </div>
        }
    }

    fn download_file(file_name: &str, url: &ObjectUrl) {
        let download_element = document().create_element("a").unwrap();
        download_element
            .set_attribute("href", &url.to_string())
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

fn get_image_for_file(file_name: String) -> &'static str {
    let extension = Path::new(&file_name).extension().unwrap().to_str().unwrap();
    if ZIP_EXTENSIONS.contains(&extension) {
        ZIP_IMG_PATH
    } else if IMG_EXTENSIONS.contains(&extension) {
        IMG_IMG_PATH
    } else if PDF_EXTENSION.contains(&extension) {
        PDF_IMG_PATH
    } else {
        DEF_IMG_PATH
    }
}
