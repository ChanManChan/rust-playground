// Topic: Channels
//
// Summary:
//   Using the existing code, create a program that simulates an internet-of-things
//   remote control light bulb. The color of the light can be changed remotely.
//   Use threads and channels to communicate what color the light bulb should display.
//
// Requirements:
// * Create a separate thread representing the light bulb
// * Use a channel to communicate with the thread
// * Display a color change message using the println! macro
// * The light bulb must also be able to turn on and off
//   * Display whether the light is on or off on each color change
// * Turn off the light when disconnecting from it
//
// Notes:
// * Remember to add `crossbeam-channel` to your Cargo.toml file
// * Use the `colored` crate if you want to get fancy and display actual colors
// * The docs.rs site can be used to read documentation for third-party crates
// * Disconnection can be accomplished by dropping the sender, or
//   by telling the thread to self-terminate

use crossbeam_channel::{unbounded, Receiver};
use std::thread::{self, JoinHandle};

enum LightMsg {
    // Add additional variants needed to complete the exercise
    ChangeColor(u8, u8, u8),
    Disconnect,
    On,
    Off,
}

#[derive(Debug)]
enum LightStatus {
    Off,
    On,
}

fn spawn_light_thread(receiver: Receiver<LightMsg>) -> JoinHandle<LightStatus> {
    // Add code here to spawn a thread to control the light bulb
    thread::spawn(move || {
        let mut light_status = LightStatus::Off;

        loop {
            match receiver.recv() {
                Ok(light_msg) => match light_msg {
                    LightMsg::ChangeColor(r, g, b) => {
                        println!("r: {}, g: {}, b: {}", r, g, b);

                        match light_status {
                            LightStatus::On => println!("Light is ON"),
                            LightStatus::Off => println!("Light is OFF"),
                        };
                    }
                    LightMsg::On => {
                        println!("Turning light ON");
                        light_status = LightStatus::On;
                    }
                    LightMsg::Off => {
                        println!("Turning light OFF");
                        light_status = LightStatus::Off;
                    }
                    LightMsg::Disconnect => {
                        println!("Disconnecting");
                        light_status = LightStatus::Off;
                        break;
                    }
                },
                Err(e) => {
                    println!("channel disconnected: {}", e);
                    light_status = LightStatus::Off;
                    break;
                }
            }
        }

        light_status
    })
}

fn main() {
    let (sender, receiver) = unbounded();

    let light_thread = spawn_light_thread(receiver);

    sender.send(LightMsg::On);
    sender.send(LightMsg::ChangeColor(255, 0, 0));
    sender.send(LightMsg::ChangeColor(0, 255, 0));
    sender.send(LightMsg::ChangeColor(0, 0, 255));
    sender.send(LightMsg::Off);
    sender.send(LightMsg::Disconnect);

    let light_status_result = light_thread.join();

    if let Ok(light_status) = light_status_result {
        println!("{:?}", light_status);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crossbeam_channel::unbounded;

    #[test]
    fn light_off_when_disconnect() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        s.send(LightMsg::Disconnect).expect("channel disconnected");

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after disconnection");
        }
    }

    #[test]
    fn light_off_when_dropped() {
        let (s, r) = unbounded();

        let light = spawn_light_thread(r);
        drop(s);

        let light_status = light.join().expect("failed to join light thread");

        if let LightStatus::On = light_status {
            panic!("light should be off after dropping sender");
        }
    }
}
