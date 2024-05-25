use std::fmt::Display;

use hexencer_core::{
    data::{midi_message::MidiMessage, ALL_NOTE_ON_MSG, NOTE_OFF_MSG, NOTE_ON_MSG},
    note::NoteEvent,
};
use midir::MidiOutput;

pub type MidiEngineSender = tokio::sync::mpsc::UnboundedSender<(MidiMessage, u8)>;
pub type MidiEngineReceiver = tokio::sync::mpsc::UnboundedReceiver<(MidiMessage, u8)>;

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

        let port = out_ports.get(2).ok_or("no output port found").unwrap();
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

    async fn play(&mut self, message: &MidiMessage, port: u8) {
        match port {
            0 => {
                let _ = self.conn_out.as_mut().map(|s| s.send(&message.to_midi()));
            }
            1 => {
                let _ = self.conn_out2.as_mut().map(|s| s.send(&message.to_midi()));
            }
            _ => {}
        }
    }

    fn stop(&mut self) {
        println!("\nClosing connection");
        self.conn_out.take().map(|c| c.close());
        println!("Connection closed");
    }

    pub async fn process(mut self, mut midi_command_receiver: MidiEngineReceiver) {
        println!("running midiio");
        while let Some((midi_message, port)) = midi_command_receiver.recv().await {
            self.play(&midi_message, port).await;
        }
    }
}
