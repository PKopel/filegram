pub mod codec;

use std::io::Cursor;

use filegram::encode;
use futures::TryStreamExt;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_streams::ReadableStream;
use web_sys::{Blob, File};

#[derive(Serialize, Deserialize)]
pub struct FilegramInput {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub file: File,
}

#[derive(Serialize, Deserialize)]
pub struct FilegramOutput {
    #[serde(with = "serde_wasm_bindgen::preserve")]
    pub blob: Uint8Array,
}

pub struct FilegramWorker {}

impl Worker for FilegramWorker {
    type Input = FilegramInput;
    type Output = FilegramOutput;
    type Message = ();

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {}
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, _id: HandlerId) {}

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {}

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        let scope = scope.clone();

        spawn_local(async move {
            let mut contents = vec![];

            // We assume that this file is big and cannot be loaded into the memory in one chunk.
            // So we process this as a stream.
            let mut s = ReadableStream::from_raw(msg.file.stream().unchecked_into()).into_stream();

            while let Some(chunk) = s.try_next().await.unwrap() {
                contents.append(&mut chunk.unchecked_into::<Uint8Array>().to_vec());
            }

            let rgb = encode::from_slice(&contents);

            let mut bytes: Vec<u8> = Vec::new();
            rgb.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)
                .unwrap();

            let blob = Uint8Array::from(&bytes[..]);
            // let blob = Blob::new_with_u8_array_sequence(&file_contents).unwrap();

            scope.respond(id, FilegramOutput { blob });
        });
    }
}
