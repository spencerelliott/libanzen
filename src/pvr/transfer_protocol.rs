pub(crate) trait DataTransferProtocol {
    fn queue(&self);
    fn send(&self);
}

pub(crate) struct SQDataTransferProtocol { }

impl DataTransferProtocol for SQDataTransferProtocol {
    fn queue(&self) {

    }

    fn send(&self) {

    }
}