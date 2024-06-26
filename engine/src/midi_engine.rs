use hexencer_core::data::MidiMessage;
use midir::MidiOutput;
use tokio::task;

/// sender type used to send commands to the midi engine
pub type MidiEngineSender = tokio::sync::mpsc::UnboundedSender<(MidiMessage, u8, u8)>;
/// receiver type used to receive commands on the midi engine
pub type MidiEngineReceiver = tokio::sync::mpsc::UnboundedReceiver<(MidiMessage, u8, u8)>;

/// reponsible for setting up midi connections, and sending, receiving, midi requests from them
#[derive(Default)]
pub struct MidiEngine {
    /// midi output connection 1
    conn_out: Option<midir::MidiOutputConnection>,
    /// midi output connection 2
    conn_out2: Option<midir::MidiOutputConnection>,
}

impl MidiEngine {
    /// create a new 'MidiEngine'
    pub fn new() -> Self {
        let midi_out = MidiOutput::new("Test Output").unwrap();
        let midi_out2 = MidiOutput::new("Test Output2").unwrap();

        let mut con1 = None;
        let mut con2 = None;

        let out_ports = midi_out.ports();
        let out_ports2 = midi_out2.ports();

        let port = out_ports.get(2);
        let port2 = out_ports2.get(3);

        if port.is_some() && port2.is_some() {
            tracing::info!("opening midi connections");
            let conn_out = midi_out.connect(port.unwrap(), "midir-test");
            con1 = match conn_out {
                Ok(conn) => Some(conn),

                Err(_) => None,
            };
            let conn_out2 = midi_out2.connect(port2.unwrap(), "midir-test2");

            con2 = match conn_out2 {
                Ok(conn) => Some(conn),

                Err(_) => None,
            };
        }

        Self {
            conn_out: con1,
            conn_out2: con2,
        }
    }

    /// sends a midi message to the midi port
    async fn play(&mut self, message: &MidiMessage, port: u8, channel: u8) {
        match port {
            0 => {
                let _ = self
                    .conn_out
                    .as_mut()
                    .map(|s| s.send(&message.to_midi(channel)));
            }
            1 => {
                let _ = self
                    .conn_out2
                    .as_mut()
                    .map(|s| s.send(&message.to_midi(channel)));
            }
            _ => {}
        }
    }

    /// close the midi connections
    pub fn close(&mut self) {
        tracing::info!("closing midi connections");
        self.conn_out.take().map(|c| c.close());
        tracing::info!("connection closed");
    }

    /// start listening and processing midi engine commands
    pub async fn listen(mut self, mut midi_command_receiver: MidiEngineReceiver) {
        tracing::info!("running midiio");
        while let Some((midi_message, port, channel)) = midi_command_receiver.recv().await {
            self.play(&midi_message, port, channel).await;
        }
    }
}

/// starts up the midi engine and listens for commands, return the sender to send commands to the midi engine
pub fn start_midi_engine() -> MidiEngineSender {
    let (midi_sender, midi_receiver) = tokio::sync::mpsc::unbounded_channel();
    let midi_engine = MidiEngine::new();
    task::spawn(midi_engine.listen(midi_receiver));
    midi_sender
}
