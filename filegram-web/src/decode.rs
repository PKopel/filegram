use base64::{engine::general_purpose, Engine};
use filegram::{
    decode,
    encryption::{Cipher, Key},
};
use gloo_file::{callbacks::FileReader, Blob, File, ObjectUrl};
use gloo_utils::document;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::utils;

type FileName = String;
type Data = Vec<u8>;

pub enum Msg {
    Key(Option<Key>),
    Decrypt(bool),
    LoadedBytes(FileName, Vec<u8>),
    Files(Vec<File>),
}

pub struct DecodeComponent {
    files: Vec<(FileName, Data)>,
    readers: HashMap<FileName, FileReader>,
    hide_key_input: bool,
    key: Option<Key>,
}

impl Component for DecodeComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            files: Vec::new(),
            readers: HashMap::default(),
            hide_key_input: true,
            key: None,
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

        let on_input = ctx.link().callback(move |e: InputEvent| {
            let key_ref: HtmlInputElement = e.target_unchecked_into();
            let key = key_ref.value();
            let key = general_purpose::STANDARD_NO_PAD.decode(key).unwrap();
            if let Ok(key) = serde_json::from_slice(&key) {
                Msg::Key(Some(key))
            } else {
                Msg::Key(None)
            }
        });

        let on_check = ctx.link().callback(move |e: MouseEvent| {
            let decrypt_ref: HtmlInputElement = e.target_unchecked_into();
            let decrypt = decrypt_ref.checked();
            Msg::Decrypt(decrypt)
        });

        html! {
            <div class="component decode">
                <div>
                    <h2>{"Choose a file to decode from an image:"}</h2>
                </div>
                <div>
                    <label class="container" for="decrypt">{"Decrypt"}
                        <input type="checkbox" id="decrypt" onclick={on_check}/>
                        <span class="checkmark"></span>
                    </label>
                    <input type="text" placeholder={"Key string"} hidden={self.hide_key_input} oninput={on_input}/>
                </div>
                <div>
                    <label class="custom-file-upload">
                        {"Select file"}
                        <input type="file" accept="image/png" onchange={on_change} multiple=false/>
                    </label>
                </div>
                <div>
                { for self.files.iter().rev().map(|(n,d)| Self::view_file(n,d))}
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
                let data = Self::decode(data);
                let file_contents = if let Some(key) = &self.key {
                    let cipher = Cipher::load(key);
                    cipher.decrypt(&data)
                } else {
                    data
                };
                self.files.push((file_name.clone(), file_contents));
                self.readers.remove(&file_name);
                true
            }
            Msg::Key(key) => {
                self.key = key;
                false
            }
            Msg::Decrypt(decrypt) => {
                self.hide_key_input = !decrypt;
                true
            }
        }
    }
}

impl DecodeComponent {
    fn view_file(name: &str, data: &[u8]) -> Html {
        let name = name.to_owned();
        let file_name = if let Some(file_name) = name.strip_suffix(".png") {
            file_name.to_owned()
        } else {
            name
        };
        let label = file_name.clone();
        let file_img = utils::get_image_for_file(file_name.clone());
        let blob = Blob::new(data);
        let blob_url = ObjectUrl::from(blob);
        let on_click = Callback::from(move |_| {
            Self::download_file(&file_name, &blob_url);
        });

        html! {
            <div class="img">
                <button onclick={on_click}>
                    <div class="center">
                        <p>{label}</p>
                    </div>
                    <div class="center">
                        <img src={file_img}/>
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

    fn decode(data: Vec<u8>) -> Vec<u8> {
        let cursor = std::io::Cursor::new(data);
        decode::from_file(cursor)
    }
}
