use crate::protocol::bedrock::serializer::packet_serializer::PacketSerializer;
use binary_utils::binary::{Reader, Writer};
use std::collections::HashMap;

#[derive(serde::Serialize, Debug)]
pub struct Experiments {
    pub experiments: HashMap<String, bool>,
    pub has_previously_used_experiments: bool,
}

impl Experiments {
    pub fn read(stream: &mut Reader) -> Experiments {
        let mut experiments = HashMap::new();
        let length = stream.get_u32_le();
        for _ in 0..length {
            let experiment_name = PacketSerializer::get_string(stream);
            let enabled = stream.get_bool();
            experiments.insert(experiment_name, enabled);
        }
        let has_previously_used_experiments = stream.get_bool();

        Experiments { experiments, has_previously_used_experiments }
    }

    pub fn write(&self, stream: &mut Writer) {
        stream.put_u32_le(self.experiments.len() as u32);
        for experiment in self.experiments.iter() {
            PacketSerializer::put_string(stream, experiment.0);
            stream.put_bool(*experiment.1);
        }
        stream.put_bool(self.has_previously_used_experiments);
    }
}
