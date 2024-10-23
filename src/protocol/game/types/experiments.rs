use std::collections::HashMap;
use binary_utils::binary::Stream;

#[derive(Debug)]
pub struct Experiments {
    experiments: HashMap<String, bool>,
    has_previously_used_experiments: bool
}

impl Experiments {
    pub fn read(stream: &mut Stream) -> Experiments {
        let mut experiments = HashMap::new();

        let length = stream.get_l_int();

        for _ in 0..length {
            let len = stream.get_unsigned_var_int();
            let experiment_name = String::from_utf8(stream.get(len).unwrap()).unwrap();
            let enabled = stream.get_bool();
            experiments.insert(experiment_name, enabled);
        }

        let has_previously_used_experiments = stream.get_bool();

        Experiments{ experiments, has_previously_used_experiments }
    }
}