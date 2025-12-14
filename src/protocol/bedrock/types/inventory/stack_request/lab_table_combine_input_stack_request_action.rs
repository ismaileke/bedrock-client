use binary_utils::binary::Stream;

#[derive(serde::Serialize, Debug)]
pub struct LabTableCombineInputStackRequestAction {}

impl LabTableCombineInputStackRequestAction {
    pub fn new() -> LabTableCombineInputStackRequestAction {
        LabTableCombineInputStackRequestAction {}
    }

    pub fn read(_stream: &mut Stream) -> LabTableCombineInputStackRequestAction {
        LabTableCombineInputStackRequestAction {}
    }

    pub fn write(&mut self, _stream: &mut Stream) {}
}
