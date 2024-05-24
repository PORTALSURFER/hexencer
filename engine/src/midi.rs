use std::time::Duration;

use hexencer_core::{Instrument, MidiMessage, Note};
use midir::MidiOutput;

pub type MidiEngineSender = tokio::sync::mpsc::UnboundedSender<MidiMessage>;

#[derive(Default)]
pub struct MidiEngine {
    conn_out: Option<midir::MidiOutputConnection>,
    conn_out2: Option<midir::MidiOutputConnection>,
    running: bool,
}

impl MidiEngine {
    pub fn new() -> Self {
        let midi_out = MidiOutput::new("Test Output").unwrap();
        let midi_out2 = MidiOutput::new("Test Output2").unwrap();
        // let midi_out2 = MidiOutput::new("Test Output").unwrap();

        // Get an output port (read from console if multiple are available)
        let out_ports = midi_out.ports();
        let out_ports2 = midi_out2.ports();

        let port = out_ports.get(4).ok_or("no output port found").unwrap();
        let port2 = out_ports2.get(3).ok_or("no output port found").unwrap();

        println!("\nOpening midi connections");
        let conn_out = Some(midi_out.connect(port, "midir-test").unwrap());
        let conn_out2 = Some(midi_out2.connect(port2, "midir-test2").unwrap());

        Self {
            conn_out,
            conn_out2,
            running: false,
        }
    }

    async fn play(&mut self, note: &Note, instrument: &Instrument) {
        const NOTE_ON_MSG: u8 = 0x90;
        const NOTE_OFF_MSG: u8 = 0x80;
        const VELOCITY: u8 = 0x64;
        // We're ignoring errors in here

        match note.channel {
            0 => {
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration)).await;
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&[NOTE_OFF_MSG, note.index, VELOCITY]));
            }
            1 => {
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&[NOTE_ON_MSG, note.index, VELOCITY]));
                tokio::time::sleep(Duration::from_millis(note.duration)).await;
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

    pub async fn process(mut self, mut rx: tokio::sync::mpsc::UnboundedReceiver<MidiMessage>) {
        println!("running midiio");
        while let Some(v) = rx.recv().await {
            match v {
                MidiMessage::NoteOn(note, instrument) => {
                    self.play(&note, &instrument).await;
                }
                MidiMessage::NoteOff(note) => {
                    println!("stopping");
                    self.stop();
                }
            }
        }
        println!("done waiting for midi events");
    }
}
