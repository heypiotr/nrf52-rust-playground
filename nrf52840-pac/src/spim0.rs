#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Start SPI transaction"]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction"]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 0x04],
    #[doc = "0x1c - Suspend SPI transaction"]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction"]
    pub tasks_resume: TASKS_RESUME,
    _reserved4: [u8; 0xe0],
    #[doc = "0x104 - SPI transaction has stopped"]
    pub events_stopped: EVENTS_STOPPED,
    _reserved5: [u8; 0x08],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved6: [u8; 0x04],
    #[doc = "0x118 - End of RXD buffer and TXD buffer reached"]
    pub events_end: EVENTS_END,
    _reserved7: [u8; 0x04],
    #[doc = "0x120 - End of TXD buffer reached"]
    pub events_endtx: EVENTS_ENDTX,
    _reserved8: [u8; 0x28],
    #[doc = "0x14c - Transaction started"]
    pub events_started: EVENTS_STARTED,
    _reserved9: [u8; 0xb0],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved10: [u8; 0x0100],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved12: [u8; 0xf4],
    #[doc = "0x400 - Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurs and can be cleared (set to NOSTALL) by the CPU."]
    pub stallstat: STALLSTAT,
    _reserved13: [u8; 0xfc],
    #[doc = "0x500 - Enable SPIM"]
    pub enable: ENABLE,
    _reserved14: [u8; 0x04],
    #[doc = "0x508..0x518 - Unspecified"]
    pub psel: PSEL,
    _reserved15: [u8; 0x0c],
    #[doc = "0x524 - SPI frequency. Accuracy depends on the HFCLK source selected."]
    pub frequency: FREQUENCY,
    _reserved16: [u8; 0x0c],
    #[doc = "0x534..0x544 - RXD EasyDMA channel"]
    pub rxd: RXD,
    #[doc = "0x544..0x554 - TXD EasyDMA channel"]
    pub txd: TXD,
    #[doc = "0x554 - Configuration register"]
    pub config: CONFIG,
    _reserved19: [u8; 0x08],
    #[doc = "0x560..0x568 - Unspecified"]
    pub iftiming: IFTIMING,
    #[doc = "0x568 - Polarity of CSN output"]
    pub csnpol: CSNPOL,
    #[doc = "0x56c - Pin select for DCX signal"]
    pub pseldcx: PSELDCX,
    #[doc = "0x570 - DCX configuration"]
    pub dcxcnt: DCXCNT,
    _reserved23: [u8; 0x4c],
    #[doc = "0x5c0 - Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
    pub orc: ORC,
}
#[doc = "TASKS_START (w) register accessor: an alias for `Reg<TASKS_START_SPEC>`"]
pub type TASKS_START = crate::Reg<tasks_start::TASKS_START_SPEC>;
#[doc = "Start SPI transaction"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop SPI transaction"]
pub mod tasks_stop;
#[doc = "TASKS_SUSPEND (w) register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend SPI transaction"]
pub mod tasks_suspend;
#[doc = "TASKS_RESUME (w) register accessor: an alias for `Reg<TASKS_RESUME_SPEC>`"]
pub type TASKS_RESUME = crate::Reg<tasks_resume::TASKS_RESUME_SPEC>;
#[doc = "Resume SPI transaction"]
pub mod tasks_resume;
#[doc = "EVENTS_STOPPED (rw) register accessor: an alias for `Reg<EVENTS_STOPPED_SPEC>`"]
pub type EVENTS_STOPPED = crate::Reg<events_stopped::EVENTS_STOPPED_SPEC>;
#[doc = "SPI transaction has stopped"]
pub mod events_stopped;
#[doc = "EVENTS_ENDRX (rw) register accessor: an alias for `Reg<EVENTS_ENDRX_SPEC>`"]
pub type EVENTS_ENDRX = crate::Reg<events_endrx::EVENTS_ENDRX_SPEC>;
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "EVENTS_END (rw) register accessor: an alias for `Reg<EVENTS_END_SPEC>`"]
pub type EVENTS_END = crate::Reg<events_end::EVENTS_END_SPEC>;
#[doc = "End of RXD buffer and TXD buffer reached"]
pub mod events_end;
#[doc = "EVENTS_ENDTX (rw) register accessor: an alias for `Reg<EVENTS_ENDTX_SPEC>`"]
pub type EVENTS_ENDTX = crate::Reg<events_endtx::EVENTS_ENDTX_SPEC>;
#[doc = "End of TXD buffer reached"]
pub mod events_endtx;
#[doc = "EVENTS_STARTED (rw) register accessor: an alias for `Reg<EVENTS_STARTED_SPEC>`"]
pub type EVENTS_STARTED = crate::Reg<events_started::EVENTS_STARTED_SPEC>;
#[doc = "Transaction started"]
pub mod events_started;
#[doc = "SHORTS (rw) register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "STALLSTAT (rw) register accessor: an alias for `Reg<STALLSTAT_SPEC>`"]
pub type STALLSTAT = crate::Reg<stallstat::STALLSTAT_SPEC>;
#[doc = "Stall status for EasyDMA RAM accesses. The fields in this register are set to STALL by hardware whenever a stall occurs and can be cleared (set to NOSTALL) by the CPU."]
pub mod stallstat;
#[doc = "ENABLE (rw) register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable SPIM"]
pub mod enable;
#[doc = "Unspecified"]
pub use self::psel::PSEL;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "FREQUENCY (rw) register accessor: an alias for `Reg<FREQUENCY_SPEC>`"]
pub type FREQUENCY = crate::Reg<frequency::FREQUENCY_SPEC>;
#[doc = "SPI frequency. Accuracy depends on the HFCLK source selected."]
pub mod frequency;
#[doc = "RXD EasyDMA channel"]
pub use self::rxd::RXD;
#[doc = r"Cluster"]
#[doc = "RXD EasyDMA channel"]
pub mod rxd;
#[doc = "TXD EasyDMA channel"]
pub use self::txd::TXD;
#[doc = r"Cluster"]
#[doc = "TXD EasyDMA channel"]
pub mod txd;
#[doc = "CONFIG (rw) register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration register"]
pub mod config;
#[doc = "Unspecified"]
pub use self::iftiming::IFTIMING;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod iftiming;
#[doc = "CSNPOL (rw) register accessor: an alias for `Reg<CSNPOL_SPEC>`"]
pub type CSNPOL = crate::Reg<csnpol::CSNPOL_SPEC>;
#[doc = "Polarity of CSN output"]
pub mod csnpol;
#[doc = "PSELDCX (rw) register accessor: an alias for `Reg<PSELDCX_SPEC>`"]
pub type PSELDCX = crate::Reg<pseldcx::PSELDCX_SPEC>;
#[doc = "Pin select for DCX signal"]
pub mod pseldcx;
#[doc = "DCXCNT (rw) register accessor: an alias for `Reg<DCXCNT_SPEC>`"]
pub type DCXCNT = crate::Reg<dcxcnt::DCXCNT_SPEC>;
#[doc = "DCX configuration"]
pub mod dcxcnt;
#[doc = "ORC (rw) register accessor: an alias for `Reg<ORC_SPEC>`"]
pub type ORC = crate::Reg<orc::ORC_SPEC>;
#[doc = "Byte transmitted after TXD.MAXCNT bytes have been transmitted in the case when RXD.MAXCNT is greater than TXD.MAXCNT"]
pub mod orc;
