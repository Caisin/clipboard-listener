use std::{borrow::Cow, time::Duration};

use arboard::Clipboard;
use md5::{Digest, Md5};
pub fn clipboard_lisen(lis_fn: fn(ClipboardData), speed: u64) {
    let mut clipboard = Clipboard::new().unwrap();
    tokio::spawn(async move {
        let mut pre_hash = String::new();
        loop {
            let md5 = Md5::new();
            match clipboard.get_text() {
                Ok(txt) => {
                    let finalize = md5.chain_update(txt.as_bytes()).finalize();
                    let encode = hex::encode(finalize);
                    if !pre_hash.eq(&encode) {
                        pre_hash = encode.clone();
                        lis_fn(ClipboardData {
                            tp: ClipboardType::Text,
                            bytes: Cow::from(txt.as_bytes()),
                            hash: &encode,
                        });
                    }
                }
                Err(_) =>
                {
                    #[cfg(feature = "image-data")]
                    if let Ok(img) = clipboard.get_image() {
                        let finalize = md5.chain_update(&img.bytes).finalize();
                        let encode = hex::encode(finalize);
                        if !pre_hash.eq(&encode) {
                            pre_hash = encode.clone();
                            lis_fn(ClipboardData {
                                tp: ClipboardType::Image,
                                bytes: Cow::from(img.bytes),
                                hash: &encode,
                            });
                        }
                    }
                }
            }
            tokio::time::sleep(Duration::from_millis(speed)).await;
        }
    });
}
pub struct ClipboardData<'a> {
    pub tp: ClipboardType,
    pub bytes: Cow<'a, [u8]>,
    pub hash: &'a str,
}

#[derive(Debug)]
pub enum ClipboardType {
    Text,
    Image,
}
