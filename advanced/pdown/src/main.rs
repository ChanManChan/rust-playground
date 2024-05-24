use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    sync::{Arc, RwLock},
    thread,
};

// const META_FILE_FORMATS: [&str; 2] = [".m3u8", ".mpd"];
//
// fn remove_suffix<'a>(primary: &'a str, pattern: &str) -> Option<&'a str> {
//     if primary.ends_with(pattern) {
//         Some(&primary[..primary.len() - pattern.len()])
//     } else {
//         None
//     }
// }

fn fetch(seg_url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer: Vec<u8> = vec![];
    reqwest::blocking::get(seg_url)?.copy_to(&mut buffer)?;
    Ok(buffer)
}

// Example meta url
// https://video-nss.xhcdn.com/LKng6BPSXepTwbHZcxI2Hw==,1716537600/media=hls4/multi=256x144:144p:,426x240:240p:,854x480:480p:,1280x720:720p:/022/474/474/720p.av1.mp4.m3u8
// Example segment url
// https://video-nss.xhcdn.com/LKng6BPSXepTwbHZcxI2Hw==,1716537600/media=hls4/multi=256x144:144p:,426x240:240p:,854x480:480p:,1280x720:720p:/022/474/474/720p.av1.mp4/seg-5-v1-a1.m4s

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let meta_url = &args[1];
    let output_filename = &args[2];

    let base_url: &str = {
        let meta_url = meta_url.as_str();
        let mut last_slash_index = usize::min_value();
        for (index, item) in meta_url.chars().enumerate() {
            if item == '/' {
                last_slash_index = index;
            }
        }
        &meta_url[..last_slash_index]
    };

    let meta_data_prefix = "#EXTINF:";
    let meta_file_result = reqwest::blocking::get(meta_url);

    if let Ok(meta_data) = meta_file_result {
        let meta_data = meta_data.text().expect("Meta information download issue");
        let mut file = File::create(output_filename).expect("Error creating file");
        let mut lines = meta_data.split('\n');
        let mut urls = vec![];
        let mut iter_index: u16 = 0;

        while let Some(line) = lines.next() {
            if line.starts_with(meta_data_prefix) {
                let segment_line = lines.next().expect("Next segment line does not exists");
                urls.push((iter_index, segment_line));
                iter_index += 1;
            }
        }

        let segment_data: Arc<RwLock<HashMap<u16, Vec<u8>>>> =
            Arc::new(RwLock::new(HashMap::new()));

        let threads: Vec<_> = urls
            .iter()
            .map(|(index, segment_line)| {
                let map_data = Arc::clone(&segment_data);
                let url = format!("{}/{}", base_url, segment_line);
                let index = *index;
                thread::spawn(move || {
                    let buffer_val = fetch(&url);
                    if let Ok(data) = buffer_val {
                        let mut hash_data = map_data.write().expect("thread write error");
                        hash_data.insert(index, data);
                        println!("{}", url);
                    } else {
                        eprintln!("Something went wrong");
                    }
                })
            })
            .collect();

        for thread in threads {
            thread.join().expect("Thread panicked");
        }

        let cumulative_data = segment_data.read().expect("thread read error");

        iter_index = 0;

        while cumulative_data.contains_key(&iter_index) {
            if let Some(data) = cumulative_data.get(&iter_index) {
                file.write_all(data.as_slice()).expect("Data not written");
            } else {
                eprintln!("Something went wrong");
            }
            iter_index += 1;
        }
    } else {
        eprintln!("Meta file download error")
    }
}
