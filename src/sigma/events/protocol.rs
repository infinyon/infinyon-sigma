use crate::prelude::types::LogString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "np_type")]
#[allow(non_camel_case_types)]
pub enum NetworkProtocol {
    HOPOPT,
    ICMP,
    IGMP,
    GGP,
    IPV4,
    ST,
    TCP,
    CBT,
    EGP,
    IGP,
    BBN_RCC_MON,
    NVP_II,
    PUP,
    ARGUS,
    EMCON,
    XNET,
    CHAOS,
    UDP,
    MUX,
    DCN_MEAS,
    HMP,
    PRM,
    XNS_IDP,
    TRUNK_1,
    TRUNK_2,
    LEAF_1,
    LEAF_2,
    RDP,
    IRTP,
    ISO_TP4,
    NETBLT,
    MFE_NSP,
    MERIT_INP,
    DCCP,
    ThirdPC,
    IDPR,
    XTP,
    DDP,
    IDPR_CMTP,
    TPpp,
    IL,
    IPV6,
    SDRP,
    IPV6_ROUTE,
    IPV6_FRAG,
    IDRP,
    RSVP,
    GRE,
    DSR,
    BNA,
    ESP,
    AH,
    I_NLSP,
    SWIPE,
    NARP,
    MOBILE,
    TLSP,
    SKIP,
    IPV6_ICMP,
    IPV6_NONXT,
    IPV6_OPTS,
    CFTP,
    SAT_EXPAK,
    KRYPTOLAN,
    RVD,
    IPPC,
    SAT_MON,
    VISA,
    IPCV,
    CPNX,
    CPHB,
    WSN,
    PVP,
    BR_SAT_MON,
    SUN_ND,
    WB_MON,
    WB_EXPAK,
    ISO_IP,
    VMTP,
    SECURE_VMTP,
    VINES,
    TTP,
    IPTM,
    NSFNET_IGP,
    DGP,
    TCF,
    EIGRP,
    OSPFIGP,
    SPRITE_RPC,
    LARP,
    MTP,
    AX_25,
    IPIP,
    MICP,
    SCC_SP,
    ETHERIP,
    ENCAP,
    GMTP,
    IFMP,
    PNNI,
    PIM,
    ARIS,
    SCPS,
    QNX,
    A_N,
    IPCOMP,
    SNP,
    COMPAQ_PEER,
    IPX_IN_IP,
    VRRP,
    PGM,
    L2TP,
    DDX,
    IATP,
    STP,
    SRP,
    UTI,
    SMP,
    SM,
    PTP,
    ISIS,
    FIRE,
    CRTP,
    CRUDP,
    SSCOPMCE,
    IPLT,
    SPS,
    PIPE,
    SCTP,
    FC,
    RSVP_E2E_IGNORE,
    MOBILITY,
    UDPLITE,
    MPLS_IN_IP,
    MANET,
    HIP,
    SHIM6,
    WESP,
    ROHC,
    ETHERNET,
    USE,
    RESERVED,
    OTHER(LogString),
    UNKNOWN,
}
impl std::fmt::Display for NetworkProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub fn parse_protocol_id(id: u16) -> NetworkProtocol {
    match id {
        0 => NetworkProtocol::HOPOPT,
        1 => NetworkProtocol::ICMP,
        2 => NetworkProtocol::IGMP,
        3 => NetworkProtocol::GGP,
        4 => NetworkProtocol::IPV4,
        5 => NetworkProtocol::ST,
        6 => NetworkProtocol::TCP,
        7 => NetworkProtocol::CBT,
        8 => NetworkProtocol::EGP,
        9 => NetworkProtocol::IGP,
        10 => NetworkProtocol::BBN_RCC_MON,
        11 => NetworkProtocol::NVP_II,
        12 => NetworkProtocol::PUP,
        13 => NetworkProtocol::ARGUS,
        14 => NetworkProtocol::EMCON,
        15 => NetworkProtocol::XNET,
        16 => NetworkProtocol::CHAOS,
        17 => NetworkProtocol::UDP,
        18 => NetworkProtocol::MUX,
        19 => NetworkProtocol::DCN_MEAS,
        20 => NetworkProtocol::HMP,
        21 => NetworkProtocol::PRM,
        22 => NetworkProtocol::XNS_IDP,
        23 => NetworkProtocol::TRUNK_1,
        24 => NetworkProtocol::TRUNK_2,
        25 => NetworkProtocol::LEAF_1,
        26 => NetworkProtocol::LEAF_2,
        27 => NetworkProtocol::RDP,
        28 => NetworkProtocol::IRTP,
        29 => NetworkProtocol::ISO_TP4,
        30 => NetworkProtocol::NETBLT,
        31 => NetworkProtocol::MFE_NSP,
        32 => NetworkProtocol::MERIT_INP,
        33 => NetworkProtocol::DCCP,
        34 => NetworkProtocol::ThirdPC,
        35 => NetworkProtocol::IDPR,
        36 => NetworkProtocol::XTP,
        37 => NetworkProtocol::DDP,
        38 => NetworkProtocol::IDPR_CMTP,
        39 => NetworkProtocol::TPpp,
        40 => NetworkProtocol::IL,
        41 => NetworkProtocol::IPV6,
        42 => NetworkProtocol::SDRP,
        43 => NetworkProtocol::IPV6_ROUTE,
        44 => NetworkProtocol::IPV6_FRAG,
        45 => NetworkProtocol::IDRP,
        46 => NetworkProtocol::RSVP,
        47 => NetworkProtocol::GRE,
        48 => NetworkProtocol::DSR,
        49 => NetworkProtocol::BNA,
        50 => NetworkProtocol::ESP,
        51 => NetworkProtocol::AH,
        52 => NetworkProtocol::I_NLSP,
        53 => NetworkProtocol::SWIPE,
        54 => NetworkProtocol::NARP,
        55 => NetworkProtocol::MOBILE,
        56 => NetworkProtocol::TLSP,
        57 => NetworkProtocol::SKIP,
        58 => NetworkProtocol::IPV6_ICMP,
        59 => NetworkProtocol::IPV6_NONXT,
        60 => NetworkProtocol::IPV6_OPTS,
        62 => NetworkProtocol::CFTP,
        64 => NetworkProtocol::SAT_EXPAK,
        65 => NetworkProtocol::KRYPTOLAN,
        66 => NetworkProtocol::RVD,
        67 => NetworkProtocol::IPPC,
        69 => NetworkProtocol::SAT_MON,
        70 => NetworkProtocol::VISA,
        71 => NetworkProtocol::IPCV,
        72 => NetworkProtocol::CPNX,
        73 => NetworkProtocol::CPHB,
        74 => NetworkProtocol::WSN,
        75 => NetworkProtocol::PVP,
        76 => NetworkProtocol::BR_SAT_MON,
        77 => NetworkProtocol::SUN_ND,
        78 => NetworkProtocol::WB_MON,
        79 => NetworkProtocol::WB_EXPAK,
        80 => NetworkProtocol::ISO_IP,
        81 => NetworkProtocol::VMTP,
        82 => NetworkProtocol::SECURE_VMTP,
        83 => NetworkProtocol::VINES,
        //84 => NetworkProtocol::TTP,
        84 => NetworkProtocol::IPTM,
        85 => NetworkProtocol::NSFNET_IGP,
        86 => NetworkProtocol::DGP,
        87 => NetworkProtocol::TCF,
        88 => NetworkProtocol::EIGRP,
        89 => NetworkProtocol::OSPFIGP,
        90 => NetworkProtocol::SPRITE_RPC,
        91 => NetworkProtocol::LARP,
        92 => NetworkProtocol::MTP,
        93 => NetworkProtocol::AX_25,
        94 => NetworkProtocol::IPIP,
        95 => NetworkProtocol::MICP,
        96 => NetworkProtocol::SCC_SP,
        97 => NetworkProtocol::ETHERIP,
        98 => NetworkProtocol::ENCAP,
        100 => NetworkProtocol::GMTP,
        101 => NetworkProtocol::IFMP,
        102 => NetworkProtocol::PNNI,
        103 => NetworkProtocol::PIM,
        104 => NetworkProtocol::ARIS,
        105 => NetworkProtocol::SCPS,
        106 => NetworkProtocol::QNX,
        107 => NetworkProtocol::A_N,
        108 => NetworkProtocol::IPCOMP,
        109 => NetworkProtocol::SNP,
        110 => NetworkProtocol::COMPAQ_PEER,
        111 => NetworkProtocol::IPX_IN_IP,
        112 => NetworkProtocol::VRRP,
        113 => NetworkProtocol::PGM,
        115 => NetworkProtocol::L2TP,
        116 => NetworkProtocol::DDX,
        117 => NetworkProtocol::IATP,
        118 => NetworkProtocol::STP,
        119 => NetworkProtocol::SRP,
        120 => NetworkProtocol::UTI,
        121 => NetworkProtocol::SMP,
        122 => NetworkProtocol::SM,
        123 => NetworkProtocol::PTP,
        124 => NetworkProtocol::ISIS,
        125 => NetworkProtocol::FIRE,
        126 => NetworkProtocol::CRTP,
        127 => NetworkProtocol::CRUDP,
        128 => NetworkProtocol::SSCOPMCE,
        129 => NetworkProtocol::IPLT,
        130 => NetworkProtocol::SPS,
        131 => NetworkProtocol::PIPE,
        132 => NetworkProtocol::SCTP,
        133 => NetworkProtocol::FC,
        134 => NetworkProtocol::RSVP_E2E_IGNORE,
        135 => NetworkProtocol::MOBILITY,
        136 => NetworkProtocol::UDPLITE,
        137 => NetworkProtocol::MPLS_IN_IP,
        138 => NetworkProtocol::MANET,
        139 => NetworkProtocol::HIP,
        140 => NetworkProtocol::SHIM6,
        141 => NetworkProtocol::WESP,
        142 => NetworkProtocol::ROHC,
        143 => NetworkProtocol::ETHERNET,
        253 => NetworkProtocol::USE,
        254 => NetworkProtocol::USE,
        255 => NetworkProtocol::RESERVED,
        _ => NetworkProtocol::UNKNOWN,
    }
}