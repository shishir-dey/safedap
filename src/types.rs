pub enum DapId {
    Info = 0x00,
    HostStatus = 0x01,
    Connect = 0x02,
    Disconnect = 0x03,
    TransferConfigure = 0x04,
    Transfer = 0x05,
    TransferBlock = 0x06,
    TransferAbort = 0x07,
    WriteAbort = 0x08,
    Delay = 0x09,
    ResetTarget = 0x0a,

    SWJPins = 0x10,
    SWJClock = 0x11,
    SWJSequence = 0x12,

    SWDConfigure = 0x13,
    SWDSequence = 0x1d,

    JTAGSequence = 0x14,
    JTAGConfigure = 0x15,
    JTAGIDCode = 0x16,

    SWOTransport = 0x17,
    SWOMode = 0x18,
    SWOBaudrate = 0x19,
    SWOControl = 0x1a,
    SWOStatus = 0x1b,
    SWOExtStatus = 0x1e,
    SWOData = 0x1c,

    QueueCommands = 0x7e,
    ExecuteCommands = 0x7f,

    Vendor0 = 0x80,
    Vendor31 = 0x9f,
    VendorExtFirst = 0xa0,
    VendorExtLast = 0xfe,

    Invalid = 0xff,
}

pub enum DapInfo {
    Vendor = 0x01,
    Product = 0x02,
    SerialNumber = 0x03,
    CmsisDapVersion = 0x04,
    DeviceVendor = 0x05,
    DeviceName = 0x06,
    BoardVendor = 0x07,
    BoardName = 0x08,
    FirmwareVersion = 0x09,
    Capabilities = 0xf0,
    Tdt = 0xf1,
    UartRxSize = 0xfb,
    UartTxSize = 0xfc,
    SwoBufferSize = 0xfd,
    PacketCount = 0xfe,
    PacketSize = 0xff,
}

pub enum DapCapability {
    Swd = 1 << 0,
    Jtag = 1 << 1,
    SwoUart = 1 << 2,
    SwoManchester = 1 << 3,
    AtomicCmd = 1 << 4,
    Tdt = 1 << 5,
    SwoStreaming = 1 << 6,
    UartComPort = 1 << 7,
}
