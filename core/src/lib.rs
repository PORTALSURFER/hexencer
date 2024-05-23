pub struct TrackManager {
    tracks: Vec<Track>,
}

pub struct InstrumentManager {
    struments: Vec<Instrument>,
}

pub struct ProjectManager {
    track_manager: TrackManager,
    instrument_manager: InstrumentManager,
}
