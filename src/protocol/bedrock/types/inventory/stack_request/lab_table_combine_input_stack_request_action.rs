use binary_utils::binary::{Reader, Writer};

#[derive(serde::Serialize, Debug)]
pub struct LabTableCombineInputStackRequestAction {}

impl LabTableCombineInputStackRequestAction {
    pub fn new() -> LabTableCombineInputStackRequestAction {
        LabTableCombineInputStackRequestAction {}
    }

    pub fn read(_stream: &mut Reader) -> LabTableCombineInputStackRequestAction {
        LabTableCombineInputStackRequestAction {}
    }

    pub fn write(&self, _stream: &mut Writer) {}
}
