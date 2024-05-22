use eframe::egui;
use midir::MidiOutput;
use std::collections::HashMap;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

struct Notes(HashMap<u8, String>);

impl Notes {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn insert(&mut self, key: u8, value: String) {
        self.0.insert(key, value);
    }
}

fn init_notes() -> Notes {
    let note_names = vec![
        "C",
        "C#",
        "D",
        "D#",
        "E",
        "F",
        "F#",
        "G",
        "G#",
        "A",
        "A#",
        "B",
    ];
    let mut note_aliases = Notes::new();

    for mini_number in 0..127 {
        let note_index = (mini_number % 12) as usize;
        let octave = (mini_number / 12) as i8 - 1;
        let note_name = format!("{}{}", note_names[note_index], octave);
        note_aliases.insert(mini_number, note_name);
    }

    note_aliases
}

enum MidiEvent {
    NoteOn(u8, u64, u8),
    NoteOff(u8),
}

struct Sequencer {
    bpm: u64,
    playing: bool,
    midiio: tokio::sync::mpsc::UnboundedSender<MidiEvent>,
    sequence_t1: Vec<u8>,
    sequence_t2: Vec<u8>,
}

impl Sequencer {
    fn new(bpm: u64, tx: tokio::sync::mpsc::UnboundedSender<MidiEvent>) -> Self {
        Self {
            bpm,
            playing: false,
            midiio: tx,
            sequence_t1: vec![69, 69, 66, 66, 67, 67],
            sequence_t2: vec![0, 0, 0, 0],
        }
    }

    async fn run(mut self) {
        println!("running sequencer");
        self.playing = true;
        let ticks = 4;
        let tick_length = 60000 / self.bpm;
        let tick = Duration::from_millis(tick_length);
        let mut current_tick = 0;
        loop {
            if !self.playing {
                break;
            }
            println!("tick {}", current_tick);
            self.sequence_t1.pop().map(|note| {
                let _ = self.midiio.send(MidiEvent::NoteOn(note, 4, 1));
            });
            self.sequence_t2.pop().map(|note| {
                let _ = self.midiio.send(MidiEvent::NoteOn(note, 4, 2));
            });
            tokio::time::sleep(tick).await;
            current_tick = current_tick + 1;
            if current_tick == ticks {
                current_tick = 0;
            }
            if self.sequence_t1.len() < 1 {
                break;
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

        Self { conn_out, conn_out2 }
    }

    async fn play(&mut self, note: u8, duration: u64, midi_channel: u8) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here

        match midi_channel {
            1 => {
                let _ = self
                .conn_out
                .as_mut()
                .map(|s| s.send(&[NOTE_ON_MSG, note, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(duration * 10)).await;
                let _ = self
                .conn_out
                .as_mut()
                .map(|s| s.send(&[NOTE_OFF_MSG, note, VELOCITY]));
            },
            2 => {
                let _ = self
                .conn_out2
                .as_mut()
                .map(|s| s.send(&[NOTE_ON_MSG, note, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(duration * 10)).await;
                let _ = self
                .conn_out2
                .as_mut()
                .map(|s| s.send(&[NOTE_OFF_MSG, note, VELOCITY]));
            },
            _ => {}
        }
    }

    fn stop(&mut self) {
        println!("\nClosing connection");
        self.conn_out.take().map(|c| c.close());
        println!("Connection closed");
    }

    async fn run(mut self, mut rx: tokio::sync::mpsc::UnboundedReceiver<MidiEvent>) {
        println!("running midiio");
        while let Some(v) = rx.recv().await {
            match v {
                MidiEvent::NoteOn(note, duration, midi_channel) => {
                    println!("playing");
                    self.play(note, duration, midi_channel).await;
                }
                MidiEvent::NoteOff(_note) => {
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
    let sequencer = Sequencer::new(160, tx);

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
