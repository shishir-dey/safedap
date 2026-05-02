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

pub enum DapTransfer {
    APnDP = 1 << 0,
    RnW = 1 << 1,
    A2 = 1 << 2,
    A3 = 1 << 3,
    MatchValue = 1 << 4,
    MatchMask = 1 << 5,
    JtagAbort = 1 << 6,
}

pub enum DapTransferStatus {
    Invalid = 0,
    Ok = 1,
    Wait = 2,
    Fault = 3,
    Error = 4,
    Mismatch = 5,
}

pub enum DapPort {
    Disabled = 0,
    AutoDetect = 1,
    Swd = 2,
    Jtag = 3,
}

pub enum DapSwjMode {
    SwclkTck = 0,
    SwdioTms = 1,
    Tdi = 2,
    Tdo = 3,
    NTrst = 4,
    NReset = 5,
}

pub enum JtagInstruction {
    Abort = 0x08,
    Dpacc = 0x09,
    Apacc = 0x0a,
    Idcode = 0x0e,
    Bypass = 0x0f,
    Invalid = 0xff,
}

pub enum JtagSequence {
    Count = 0x3f,
    Tms = 0x40,
    Tdo = 0x80,
}

pub enum SwdSequence {
    Count = 0x3f,
    Din = 0x80,
}
