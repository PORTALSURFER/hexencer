use eframe::egui;
use midir::MidiOutput;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{self, Instant};

#[derive(Debug)]
struct Note {
    index: u8,
    channel: u8,
    velocity: u8,
    duration: u64,
}

#[derive(Debug)]
struct Instrument {
    name: String,
    midi_port: u8,
}

#[derive(Debug)]
enum MidiMessage {
    NoteOn(Note, Instrument),
    NoteOff(Note),
}

#[derive(Debug)]
struct MidiEvent {
    tick: u64,
    message: MidiMessage,
}

struct Track {
    events: Vec<MidiEvent>,
}

struct Sequencer {
    bpm: f64,
    ppqn: u32,
    tracks: HashMap<u8, Track>,
    midiio: tokio::sync::mpsc::UnboundedSender<MidiMessage>,
    sequence_t1: Vec<u8>,
    sequence_t2: Vec<u8>,
}

impl Sequencer {
    fn new(tx: tokio::sync::mpsc::UnboundedSender<MidiMessage>) -> Self {
        Self {
            bpm: 120.0,
            ppqn: 480,
            tracks: HashMap::new(),
            midiio: tx,
            sequence_t1: vec![69, 69, 66, 66, 67, 67],
            sequence_t2: vec![0, 0, 0, 0],
        }
    }

    fn add_event(&mut self, track_id: u8, event: MidiEvent) {
        self.tracks
            .entry(track_id)
            .or_insert(Track { events: Vec::new() })
            .events
            .push(event);
    }

    fn tick_duration(&self) -> u64 {
        let beat_duration = 60.0 / self.bpm;
        let tick_duration = (beat_duration / self.ppqn as f64) * 1000.0;
        tick_duration as u64
    }

    async fn run(self) {
        println!("running sequencer");
        let tick_duration = self.tick_duration();
        let mut interval = time::interval(Duration::from_millis(tick_duration));
        let mut current_tick = 0;

        loop {
            interval.tick().await;
            if current_tick % 480 == 0 {
                println!("{} ||||", current_tick);
            } else {
                // println!("{} ..", current_tick);
            }
            self.play_events(current_tick).await;
            current_tick = current_tick + 1;

            // println!("tick {}", current_tick);
            // self.sequence_t1.pop().map(|note| {
            //     let _ = self.midiio.send(MidiMessage::NoteOn(
            //         Note {
            //             index: note,
            //             channel: 1,
            //             velocity: 64,
            //             duration: 4,
            //         },
            //         Instrument {
            //             name: "".to_string(),
            //             midi_port: 1,
            //         },
            //     ));
            // });
            // self.sequence_t2.pop().map(|note| {
            //     let _ = self.midiio.send(MidiMessage::NoteOn(
            //         Note {
            //             index: note,
            //             channel: 2,
            //             velocity: 64,
            //             duration: 4,
            //         },
            //         Instrument {
            //             name: "".to_string(),
            //             midi_port: 2,
            //         },
            //     ));
            // });
            // tokio::time::sleep(tick).await;
            // current_tick = current_tick + 1;
            // if current_tick == ticks {
            //     current_tick = 0;
            // }
            // if self.sequence_t1.len() < 1 {
            //     break;
            // }
        }
    }

    async fn play_events(&self, current_tick: u64) {
        for track in self.tracks.values() {
            for event in &track.events {
                if event.tick == current_tick {
                    println!("playing event {:?} {:?}", current_tick, event);
                } else {
                    // println!("no event");
                }
            }
        }
    }
}

struct MidiIO {
    conn_out: Option<midir::MidiOutputConnection>,
    conn_out2: Option<midir::MidiOutputConnection>,
}

impl MidiIO {
    fn new() -> Self {
        let midi_out = MidiOutput::new("Test Output").unwrap();
        let midi_out2 = MidiOutput::new("Test Output").unwrap();
        // let midi_out2 = MidiOutput::new("Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_ports2 = midi_out2.ports();

        let port = out_ports.get(4).ok_or("no output port found").unwrap();
        let port2 = out_ports2.get(3).ok_or("no output port found").unwrap();

        println!("\nOpening connection");
        let conn_out = Some(midi_out.connect(port, "midir-test").unwrap());
        let conn_out2 = Some(midi_out2.connect(port2, "midir-test2").unwrap());

        Self {
            conn_out,
            conn_out2,
        }
    }

    async fn play(&mut self, note: &Note, instrument: &Instrument) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here

        match instrument.midi_port {
            1 => {
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration * 1000)).await;
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_OFF_MSG, note.index, VELOCITY]));
            }
            2 => {
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration * 1000)).await;
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&[NOTE_OFF_MSG, note.index, VELOCITY]));
            }
            _ => {}
        }
    }

    fn stop(&mut self) {
        println!("\nClosing connection");
        self.conn_out.take().map(|c| c.close());
        println!("Connection closed");
    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::UnboundedReceiver<MidiMessage>) {
        println!("running midiio");
        while let Some(v) = rx.recv().await {
            match v {
                MidiMessage::NoteOn(note, instrument) => {
                    println!("playing");
                    self.play(&note, &instrument).await;
                }
                MidiMessage::NoteOff(note) => {
                    println!("stopping");
                    self.stop();
                }
            }
        }
        println!("done");
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let midi_player = MidiIO::new();
    let mut sequencer = Sequencer::new(tx);

    sequencer.add_event(
        0,
        MidiEvent {
            tick: 480,
            message: MidiMessage::NoteOn(
                Note {
                    index: 69,
                    channel: 1,
                    velocity: 64,
                    duration: 4,
                },
                Instrument {
                    name: "".to_string(),
                    midi_port: 1,
                },
            ),
        },
    );

    sequencer.add_event(
        0,
        MidiEvent {
            tick: 481,
            message: MidiMessage::NoteOn(
                Note {
                    index: 69,
                    channel: 1,
                    velocity: 64,
                    duration: 4,
                },
                Instrument {
                    name: "".to_string(),
                    midi_port: 1,
                },
            ),
        },
    );
    sequencer.add_event(
        0,
        MidiEvent {
            tick: 960,
            message: MidiMessage::NoteOn(
                Note {
                    index: 69,
                    channel: 1,
                    velocity: 64,
                    duration: 4,
                },
                Instrument {
                    name: "".to_string(),
                    midi_port: 1,
                },
            ),
        },
    );

    let mp = tokio::spawn(async move {
        midi_player.run(rx).await;
    });

    let sq = tokio::spawn(async move {
        sequencer.run().await;
    });

    tokio::select! {
        _ = mp => {
            println!("midi player done");
        },
        _ = sq => {
            println!("sequencer done");
        }
    }

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
    //     ..Default::default()
    // };
}

fn gui() {
    // eframe::run_simple_native("Gui App", options, move |ctx, _frame| {
    //     egui::CentralPanel::default().show(ctx, |ui| {
    //         ui.heading("midi app");
    //         ui.horizontal(|ui| {
    //             if ui.button("Play").clicked() {
    //             }
    //             if ui.button("Stop").clicked() {
    //             }
    //         });
    //     });
    // });
}
