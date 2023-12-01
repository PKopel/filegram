use base64::{engine::general_purpose, Engine as _};
use filegram::encode;
use filegram::encryption::{Cipher, Key};
use gloo_file::{callbacks::FileReader, File};
use gloo_file::{Blob, ObjectUrl};
use gloo_utils::document;
use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::prelude::*;

type FileName = String;
type Data = Vec<u8>;

pub enum Msg {
    LoadedBytes(FileName, Vec<u8>, bool),
    Files(Vec<File>, bool),
}

pub struct EncodeComponent {
    encrypt_ref: NodeRef,
    files: Vec<(FileName, Data, Option<Key>)>,
    readers: HashMap<FileName, FileReader>,
}

impl Component for EncodeComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            encrypt_ref: NodeRef::default(),
            files: Vec::new(),
            readers: HashMap::default(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let encrypt_ref = self.encrypt_ref.clone();
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
            let encrypt = encrypt_ref.cast::<HtmlInputElement>().unwrap().checked();
            Msg::Files(selected_files, encrypt)
        });

        html! {
            <div class="component encode">
                <div>
                    <h2>{"Choose a file to encode as an image:"}</h2>
                </div>
                <div>
                    <label class="container" for="encrypt">
                        {"Encrypt"}
                        <input type="checkbox" id="encrypt" ref={self.encrypt_ref.clone()}/>
                        <span class="checkmark"></span>
                    </label>
                </div>
                <div>
                    <label class="custom-file-upload">
                        {"Select file"}
                        <input type="file" onchange={on_change} multiple=false/>
                    </label>
                </div>
                <div>
                { for self.files.iter().rev().map(|(n,d,k)| Self::view_file(n,d,k))}
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Files(files, encrypt) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let task = {
                        let file_name = file_name.clone();
                        let link = ctx.link().clone();

                        gloo_file::callbacks::read_as_bytes(&file, move |res| {
                            link.send_message(Msg::LoadedBytes(
                                file_name,
                                res.expect("failed to read file"),
                                encrypt,
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
            Msg::LoadedBytes(file_name, data, encrypt) => {
                let (image, key) = if encrypt {
                    let cipher = Cipher::new();
                    let data = cipher.encrypt(&data);
                    (Self::encode(data), Some(cipher.get_key_struct()))
                } else {
                    (Self::encode(data), None)
                };
                self.files.push((file_name.clone(), image, key));
                self.readers.remove(&file_name);
                true
            }
        }
    }
}

impl EncodeComponent {
    fn view_file(name: &str, data: &[u8], key: &Option<Key>) -> Html {
        let image_data = general_purpose::STANDARD_NO_PAD.encode(data);
        let img = format!("data:image/png;base64,{}", image_data);

        let image_file_name = name.to_owned() + ".png";
        let image_blob = Blob::new_with_options(data, Some("image/png"));
        let image_blob_url = ObjectUrl::from(image_blob);

        let (on_click, donwload_text) = if let Some(key) = key {
            let key_file_name = name.to_owned() + ".key";
            let key_json = serde_json::to_string(key).unwrap();
            let key_data = general_purpose::STANDARD_NO_PAD.encode(key_json);
            let key_blob = Blob::new(key_data.as_str());
            let key_blob_url = ObjectUrl::from(key_blob);
            let on_click = Callback::from(move |_| {
                Self::download_file(&key_file_name, &key_blob_url);
                Self::download_file(&image_file_name, &image_blob_url);
            });
            let download_text = format!("{}.png\n{}.key", name, name);
            (on_click, download_text)
        } else {
            let on_click = Callback::from(move |_| {
                Self::download_file(&image_file_name, &image_blob_url);
            });
            let download_text = format!("{}.png", name);
            (on_click, download_text)
        };

        html! {
            <div class="img">
                <button onclick={on_click}>
                    <div class="center">
                        <p>{donwload_text}</p>
                    </div>
                    <div class="center">
                        <img src={img}/>
                    </div>
                </button>
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
