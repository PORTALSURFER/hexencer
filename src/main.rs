use eframe::egui;
use midir::MidiOutput;
use std::error::Error;
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;

enum MidiEvent {
    NoteOn(u8, u64),
    NoteOff(u8),
}

struct Sequencer {
    bpm: u64,
    playing: bool,
    midiio: tokio::sync::mpsc::UnboundedSender<MidiEvent>,
    sequence: Vec<u8>,
}

impl Sequencer {
    fn new(bpm: u64, tx: tokio::sync::mpsc::UnboundedSender<MidiEvent>) -> Self {
        Self {
            bpm,
            playing: false,
            midiio: tx,
            sequence: vec![69, 69, 66, 66, 67, 67],
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
            self.sequence.pop().map(|note| {
                let _ = self.midiio.send(MidiEvent::NoteOn(note, 4));
            });
            tokio::time::sleep(tick).await;
            current_tick = current_tick + 1;
            if current_tick == ticks {
                current_tick = 0;
            }
            if self.sequence.len() < 1 {
                break;
            }
        }
    }
}

struct MidiIO {
    conn_out: Option<midir::MidiOutputConnection>,
}

impl MidiIO {
    fn new() -> Self {
        let midi_out = MidiOutput::new("Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let port = out_ports.get(4).ok_or("no output port found").unwrap();

        println!("\nOpening connection");
        let conn_out = Some(midi_out.connect(port, "midir-test").unwrap());

        Self { conn_out }
    }

    async fn play(&mut self, note: u8, duration: u64) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here
        let _ = self
            .conn_out
            .as_mut()
            .map(|s| s.send(&[NOTE_ON_MSG, note, VELOCITY]));
        tokio::time::sleep(Duration::from_millis(duration * 10)).await;
        let _ = self
            .conn_out
            .as_mut()
            .map(|s| s.send(&[NOTE_OFF_MSG, note, VELOCITY]));
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
                MidiEvent::NoteOn(note, duration) => {
                    println!("playing");
                    self.play(note, duration).await;
                }
                MidiEvent::NoteOff(note) => {
                    println!("stopping");
                    self.stop();
                }
                _ => {
                    println!("unknown");
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
    let sequencer = Sequencer::new(120, tx);

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
