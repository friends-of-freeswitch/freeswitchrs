#![allow(//dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

use std;
use std::os::raw::*;

/// This function copies a &str to a C-style nul-terminated char*.
/// It uses malloc, so that other code (FreeSWITCH) can call free() on it.
/// For example, event_header.name is a *mut c_char that FS will free when finished.
pub fn str_to_ptr(s: &str) -> *mut c_char {
    unsafe {
        let res = ::libc::malloc(s.len() + 1) as *mut c_char;
        std::ptr::copy_nonoverlapping(s.as_ptr(), res as *mut u8, s.len());
        *res.offset(s.len() as isize + 1) = 0;
        res
    }
}

pub type time_t = c_long;

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timeval {
    pub tv_sec: c_long,
    pub tv_usec: c_long,
}
impl std::default::Default for timeval {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum FILE {}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct in6_addr {
    pub __in6_u: [u32; 4usize],
}
impl std::default::Default for in6_addr {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cJSON {
    pub next: *mut cJSON,
    pub prev: *mut cJSON,
    pub child: *mut cJSON,
    pub type_: c_int,
    pub valuestring: *mut c_char,
    pub valueint: c_int,
    pub valuedouble: f64,
    pub string: *mut c_char,
}
impl std::default::Default for cJSON {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cJSON_Hooks {
    pub malloc_fn: Option<extern "C" fn(sz: usize) -> *mut c_void>,
    pub free_fn: Option<unsafe extern "C" fn(ptr: *mut c_void)>,
}
impl std::default::Default for cJSON_Hooks {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum vpx_img_fmt {
    NONE = 0,
    RGB24 = 1,
    RGB32 = 2,
    RGB565 = 3,
    RGB555 = 4,
    UYVY = 5,
    YUY2 = 6,
    YVYU = 7,
    BGR24 = 8,
    RGB32_LE = 9,
    ARGB = 10,
    ARGB_LE = 11,
    RGB565_LE = 12,
    RGB555_LE = 13,
    YV12 = 769,
    I420 = 258,
    VPXYV12 = 771,
    VPXI420 = 260,
    I422 = 261,
    I444 = 262,
    I440 = 263,
    _444A = 1286,
    I42016 = 2306,
    I42216 = 2309,
    I44416 = 2310,
    I44016 = 2311,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum vpx_color_space {
    UNKNOWN = 0,
    BT_601 = 1,
    BT_709 = 2,
    SMPTE_170 = 3,
    SMPTE_240 = 4,
    BT_2020 = 5,
    RESERVED = 6,
    SRGB = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum vpx_color_range {
    STUDIO_RANGE = 0,
    FULL_RANGE = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct vpx_image {
    pub fmt: vpx_img_fmt,
    pub cs: vpx_color_space,
    pub range: vpx_color_range,
    pub w: c_uint,
    pub h: c_uint,
    pub bit_depth: c_uint,
    pub d_w: c_uint,
    pub d_h: c_uint,
    pub r_w: c_uint,
    pub r_h: c_uint,
    pub x_chroma_shift: c_uint,
    pub y_chroma_shift: c_uint,
    pub planes: [*mut c_uchar; 4usize],
    pub stride: [c_int; 4usize],
    pub bps: c_int,
    pub user_priv: *mut c_void,
    pub img_data: *mut c_uchar,
    pub img_data_owner: c_int,
    pub self_allocd: c_int,
    pub fb_priv: *mut c_void,
}
impl std::default::Default for vpx_image {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct vpx_image_rect {
    pub x: c_uint,
    pub y: c_uint,
    pub w: c_uint,
    pub h: c_uint,
}
impl std::default::Default for vpx_image_rect {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum pvt_class {
    PRIMARY = 0,
    SECONDARY = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dtmf_source {
    UNKNOWN = 0,
    INBAND_AUDIO = 1,
    RTP = 2,
    ENDPOINT = 3,
    APP = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum digit_action_target {
    SELF = 0,
    PEER = 1,
    BOTH = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dtmf_flag {
    SKIP_PROCESS = 1,
    SENSITIVE = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct dtmf {
    pub digit: c_char,
    pub duration: u32,
    pub flags: i32,
    pub source: dtmf_source,
}
impl std::default::Default for dtmf {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum call_direction {
    INBOUND = 0,
    OUTBOUND = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum bind_flag_enum {
    DIAL_ALEG = 1,
    EXEC_ALEG = 2,
    DIAL_BLEG = 4,
    EXEC_BLEG = 8,
    EXEC_OPPOSITE = 16,
    EXEC_SAME = 32,
    ONCE = 64,
    EXEC_INLINE = 128,
}
pub type bind_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dtmf_direction {
    RECV = 0,
    SEND = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum originate_flag_enum {
    NONE = 0,
    NOBLOCK = 1,
    FORKED_DIAL = 2,
    NO_EFFECTIVE_ANI = 4,
    NO_EFFECTIVE_ANIII = 8,
    NO_EFFECTIVE_CID_NUM = 16,
    NO_EFFECTIVE_CID_NAME = 32,
    NO_LIMITS = 64,
}
pub type originate_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum port_flag_enum {
    NONE = 0,
    ODD = 1,
    EVEN = 2,
    ROBUST_TCP = 4,
    ROBUST_UDP = 8,
}
pub type port_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum eavesdrop_flag_enum {
    NONE = 0,
    MUX_READ = 1,
    MUX_WRITE = 2,
    DTMF = 4,
    COPY_DISPLAY = 8,
    BRIDGE_READ = 16,
    BRIDGE_WRITE = 32,
}
pub type eavesdrop_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_flag_enum {
    NONE = 0,
    USE_SQL = 1,
    NO_NEW_OUTBOUND_SESSIONS = 2,
    NO_NEW_INBOUND_SESSIONS = 4,
    NO_NEW_SESSIONS = 6,
    SHUTTING_DOWN = 8,
    VG = 16,
    RESTART = 32,
    SHUTDOWN_REQUESTED = 64,
    USE_AUTO_NAT = 128,
    EARLY_HANGUP = 256,
    CALIBRATE_CLOCK = 512,
    USE_HEAVY_TIMING = 1024,
    USE_CLOCK_RT = 2048,
    VERBOSE_EVENTS = 4096,
    USE_WIN32_MONOTONIC = 8192,
    AUTO_SCHEMAS = 16384,
    MINIMAL = 32768,
    USE_NAT_MAPPING = 65536,
    CLEAR_SQL = 131072,
    THREADED_SYSTEM_EXEC = 262144,
    SYNC_CLOCK_REQUESTED = 524288,
    CORE_NON_SQLITE_DB_REQ = 1048576,
    DEBUG_SQL = 2097152,
    API_EXPANSION = 4194304,
    SESSION_THREAD_POOL = 8388608,
    DIALPLAN_TIMESTAMPS = 16777216,
}
pub type core_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum module_interface_name {
    ENDPOINT_INTERFACE = 0,
    TIMER_INTERFACE = 1,
    DIALPLAN_INTERFACE = 2,
    CODEC_INTERFACE = 3,
    APPLICATION_INTERFACE = 4,
    API_INTERFACE = 5,
    FILE_INTERFACE = 6,
    SPEECH_INTERFACE = 7,
    DIRECTORY_INTERFACE = 8,
    CHAT_INTERFACE = 9,
    SAY_INTERFACE = 10,
    ASR_INTERFACE = 11,
    MANAGEMENT_INTERFACE = 12,
    LIMIT_INTERFACE = 13,
    CHAT_APPLICATION_INTERFACE = 14,
    JSON_API_INTERFACE = 15,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum unicast_flag_enum {
    NONE = 0,
    THREAD_RUNNING = 1,
    READY = 2,
    NATIVE = 4,
}
pub type unicast_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum switch_bool {
    FALSE = 0,
    TRUE = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum say_method {
    NA = 0,
    PRONOUNCED = 1,
    ITERATED = 2,
    COUNTED = 3,
    PRONOUNCED_YEAR = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum say_type {
    NUMBER = 0,
    ITEMS = 1,
    PERSONS = 2,
    MESSAGES = 3,
    CURRENCY = 4,
    TIME_MEASUREMENT = 5,
    CURRENT_DATE = 6,
    CURRENT_TIME = 7,
    CURRENT_DATE_TIME = 8,
    TELEPHONE_NUMBER = 9,
    TELEPHONE_EXTENSION = 10,
    URL = 11,
    IP_ADDRESS = 12,
    EMAIL_ADDRESS = 13,
    POSTAL_ADDRESS = 14,
    ACCOUNT_NUMBER = 15,
    NAME_SPELLED = 16,
    NAME_PHONETIC = 17,
    SHORT_DATE_TIME = 18,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum say_gender {
    MASCULINE = 0,
    FEMININE = 1,
    NEUTER = 2,
    UTRUM = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum management_action {
    NONE = 0,
    GET = 1,
    SET = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum scheduler_flag_enum {
    NONE = 0,
    OWN_THREAD = 1,
    FREE_ARG = 2,
    NO_DEL = 4,
}
pub type scheduler_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum media_flag_enum {
    NONE = 0,
    REBRIDGE = 1,
    ECHO_ALEG = 2,
    ECHO_BLEG = 4,
    FORCE = 8,
    LOOP = 16,
    HOLD_BLEG = 32,
    IMMEDIATE = 64,
    EXEC_INLINE = 128,
    PRIORITY = 256,
    REPLYONLY_A = 512,
    REPLYONLY_B = 1024,
}
pub type media_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum bitpack_mode {
    RFC3551 = 0,
    AAL2 = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum abc_type {
    INIT = 0,
    READ = 1,
    WRITE = 2,
    WRITE_REPLACE = 3,
    READ_REPLACE = 4,
    READ_PING = 5,
    TAP_NATIVE_READ = 6,
    TAP_NATIVE_WRITE = 7,
    CLOSE = 8,
    READ_VIDEO_PING = 9,
    WRITE_VIDEO_PING = 10,
    STREAM_VIDEO_PING = 11,
    VIDEO_PATCH = 12,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct directories {
    pub base_dir: *mut c_char,
    pub mod_dir: *mut c_char,
    pub conf_dir: *mut c_char,
    pub log_dir: *mut c_char,
    pub run_dir: *mut c_char,
    pub db_dir: *mut c_char,
    pub script_dir: *mut c_char,
    pub temp_dir: *mut c_char,
    pub htdocs_dir: *mut c_char,
    pub grammar_dir: *mut c_char,
    pub storage_dir: *mut c_char,
    pub cache_dir: *mut c_char,
    pub recordings_dir: *mut c_char,
    pub sounds_dir: *mut c_char,
    pub lib_dir: *mut c_char,
    pub certs_dir: *mut c_char,
    pub fonts_dir: *mut c_char,
    pub images_dir: *mut c_char,
    pub data_dir: *mut c_char,
    pub localstate_dir: *mut c_char,
}
impl std::default::Default for directories {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct filenames {
    pub conf_name: *mut c_char,
}
impl std::default::Default for filenames {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rw {
    READ = 0,
    WRITE = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum caller_profile_flag_enum {
    NONE = 0,
    SCREEN = 1,
    HIDE_NAME = 2,
    HIDE_NUMBER = 4,
}
pub type caller_profile_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum audio_col {
    STR_TITLE = 1,
    STR_COPYRIGHT = 2,
    STR_SOFTWARE = 3,
    STR_ARTIST = 4,
    STR_COMMENT = 5,
    STR_DATE = 6,
}
pub const XML_SECTION_MAX: xml_section_enum = xml_section_enum::CHANNELS;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xml_section_enum {
    RESULT = 0,
    CONFIG = 1,
    DIRECTORY = 2,
    DIALPLAN = 4,
    LANGUAGES = 8,
    CHATPLAN = 16,
    CHANNELS = 32,
}
pub type xml_section = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum vad_flag_enum {
    TALKING = 1,
    EVENTS_TALK = 2,
    EVENTS_NOTALK = 4,
    CNG = 8,
}
pub type vad_flag = u32;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct error_period {
    pub start: i64,
    pub stop: i64,
    pub next: *mut error_period,
}
impl std::default::Default for error_period {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct rtp_numbers {
    pub raw_bytes: usize,
    pub media_bytes: usize,
    pub packet_count: usize,
    pub period_packet_count: usize,
    pub media_packet_count: usize,
    pub skip_packet_count: usize,
    pub jb_packet_count: usize,
    pub dtmf_packet_count: usize,
    pub cng_packet_count: usize,
    pub flush_packet_count: usize,
    pub largest_jb_size: usize,
    pub last_proc_time: i64,
    pub jitter_n: i64,
    pub jitter_add: i64,
    pub jitter_addsq: i64,
    pub variance: f64,
    pub min_variance: f64,
    pub max_variance: f64,
    pub std_deviation: f64,
    pub lossrate: f64,
    pub burstrate: f64,
    pub mean_interval: f64,
    pub loss: [c_int; 1024usize],
    pub last_loss: c_int,
    pub recved: c_int,
    pub last_processed_seq: c_int,
    pub flaws: usize,
    pub last_flaw: usize,
    pub R: f64,
    pub mos: f64,
    pub error_log: *mut error_period,
}
impl std::clone::Clone for rtp_numbers {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for rtp_numbers {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtcp_numbers {
    pub packet_count: u32,
    pub octet_count: u32,
    pub peer_ssrc: u32,
    pub last_rpt_ts: u32,
    pub ssrc: u32,
    pub csrc: u32,
    pub last_pkt_tsdiff: u32,
    pub inter_jitter: f64,
    pub last_rpt_ext_seq: u32,
    pub last_rpt_cycle: u16,
    pub period_pkt_count: u16,
    pub pkt_count: u16,
    pub sent_pkt_count: u16,
    pub rtcp_rtp_count: u32,
    pub high_ext_seq_recv: u32,
    pub cycle: u16,
    pub bad_seq: u32,
    pub base_seq: u16,
    pub cum_lost: u32,
    pub last_recv_lsr_local: u32,
    pub last_recv_lsr_peer: u32,
    pub init: u32,
}
impl std::default::Default for rtcp_numbers {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtp_stats {
    pub inbound: rtp_numbers,
    pub outbound: rtp_numbers,
    pub rtcp: rtcp_numbers,
    pub read_count: u32,
}
impl std::default::Default for rtp_stats {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtp_flush {
    ONCE = 0,
    STICK = 1,
    UNSTICK = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtp_flag {
    NOBLOCK = 0,
    DTMF_ON = 1,
    IO = 2,
    USE_TIMER = 3,
    RTCP_PASSTHRU = 4,
    SECURE_SEND = 5,
    SECURE_RECV = 6,
    AUTOADJ = 7,
    RAW_WRITE = 8,
    GOOGLEHACK = 9,
    VAD = 10,
    BREAK = 11,
    UDPTL = 12,
    DATAWAIT = 13,
    BYTESWAP = 14,
    PASS_RFC2833 = 15,
    AUTO_CNG = 16,
    SECURE_SEND_RESET = 17,
    SECURE_RECV_RESET = 18,
    PROXY_MEDIA = 19,
    SHUTDOWN = 20,
    FLUSH = 21,
    AUTOFLUSH = 22,
    STICKY_FLUSH = 23,
    ZRTP_FLAG_SECURE_SEND = 24,
    ZRTP_FLAG_SECURE_RECV = 25,
    ZRTP_FLAG_SECURE_MITM_SEND = 26,
    ZRTP_FLAG_SECURE_MITM_RECV = 27,
    DEBUG_RTP_READ = 28,
    DEBUG_RTP_WRITE = 29,
    VIDEO = 30,
    ENABLE_RTCP = 31,
    RTCP_MUX = 32,
    KILL_JB = 33,
    VIDEO_BREAK = 34,
    PAUSE = 35,
    FIR = 36,
    PLI = 37,
    RESET = 38,
    MUTE = 39,
    NACK = 40,
    TMMBR = 41,
    GEN_TS_DELTA = 42,
    INVALID = 43,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtp_bug_flag {
    NONE = 0,
    CISCO_SKIP_MARK_BIT_2833 = 1,
    SONUS_SEND_INVALID_TIMESTAMP_2833 = 2,
    IGNORE_MARK_BIT = 4,
    SEND_LINEAR_TIMESTAMPS = 8,
    START_SEQ_AT_ZERO = 16,
    NEVER_SEND_MARKER = 32,
    IGNORE_DTMF_DURATION = 64,
    ACCEPT_ANY_PACKETS = 128,
    GEN_ONE_GEN_ALL = 256,
    CHANGE_SSRC_ON_MARKER = 512,
    FLUSH_JB_ON_DTMF = 1024,
    ACCEPT_ANY_PAYLOAD = 2048,
    ALWAYS_AUTO_ADJUST = 4096,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtp_hdr {
    pub _bindgen_bitfield_1_: c_uint,
    pub _bindgen_bitfield_2_: c_uint,
    pub _bindgen_bitfield_3_: c_uint,
}
impl std::default::Default for rtp_hdr {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtp_hdr_ext {
    pub _bindgen_bitfield_1_: c_uint,
}
impl std::default::Default for rtp_hdr_ext {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtcp_hdr {
    pub _bindgen_bitfield_1_: c_uint,
}
impl std::default::Default for rtcp_hdr {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct audio_buffer_header {
    pub ts: u32,
    pub len: u32,
}
impl std::default::Default for audio_buffer_header {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum priority {
    NORMAL = 0,
    LOW = 1,
    HIGH = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ivr_option_enum {
    NONE = 0,
    ASYNC = 1,
    FILE = 2,
}
pub type ivr_option = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_session_message_types {
    REDIRECT_AUDIO = 0,
    TRANSMIT_TEXT = 1,
    INDICATE_ANSWER = 2,
    INDICATE_PROGRESS = 3,
    INDICATE_BRIDGE = 4,
    INDICATE_UNBRIDGE = 5,
    INDICATE_TRANSFER = 6,
    INDICATE_RINGING = 7,
    INDICATE_ALERTING = 8,
    INDICATE_MEDIA = 9,
    INDICATE_3P_MEDIA = 10,
    INDICATE_NOMEDIA = 11,
    INDICATE_3P_NOMEDIA = 12,
    INDICATE_HOLD = 13,
    INDICATE_UNHOLD = 14,
    INDICATE_REDIRECT = 15,
    INDICATE_RESPOND = 16,
    INDICATE_BROADCAST = 17,
    INDICATE_MEDIA_REDIRECT = 18,
    INDICATE_DEFLECT = 19,
    INDICATE_VIDEO_REFRESH_REQ = 20,
    INDICATE_DISPLAY = 21,
    INDICATE_TRANSCODING_NECESSARY = 22,
    INDICATE_AUDIO_SYNC = 23,
    INDICATE_VIDEO_SYNC = 24,
    INDICATE_REQUEST_IMAGE_MEDIA = 25,
    INDICATE_UUID_CHANGE = 26,
    INDICATE_SIMPLIFY = 27,
    INDICATE_DEBUG_MEDIA = 28,
    INDICATE_PROXY_MEDIA = 29,
    INDICATE_APPLICATION_EXEC = 30,
    INDICATE_APPLICATION_EXEC_COMPLETE = 31,
    INDICATE_PHONE_EVENT = 32,
    INDICATE_T38_DESCRIPTION = 33,
    INDICATE_UDPTL_MODE = 34,
    INDICATE_CLEAR_PROGRESS = 35,
    INDICATE_JITTER_BUFFER = 36,
    INDICATE_RECOVERY_REFRESH = 37,
    INDICATE_SIGNAL_DATA = 38,
    INDICATE_MESSAGE = 39,
    INDICATE_INFO = 40,
    INDICATE_AUDIO_DATA = 41,
    INDICATE_BLIND_TRANSFER_RESPONSE = 42,
    INDICATE_STUN_ERROR = 43,
    INDICATE_MEDIA_RENEG = 44,
    INDICATE_KEEPALIVE = 45,
    INDICATE_HARD_MUTE = 46,
    INDICATE_BITRATE_REQ = 47,
    INDICATE_BITRATE_ACK = 48,
    INDICATE_CODEC_DEBUG_REQ = 49,
    INDICATE_CODEC_SPECIFIC_REQ = 50,
    REFER_EVENT = 51,
    ANSWER_EVENT = 52,
    PROGRESS_EVENT = 53,
    RING_EVENT = 54,
    RESAMPLE_EVENT = 55,
    HEARTBEAT_EVENT = 56,
    INVALID = 57,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct t38_options {
    pub T38FaxVersion: u16,
    pub T38MaxBitRate: u32,
    pub T38FaxFillBitRemoval: switch_bool,
    pub T38FaxTranscodingMMR: switch_bool,
    pub T38FaxTranscodingJBIG: switch_bool,
    pub T38FaxRateManagement: *const c_char,
    pub T38FaxMaxBuffer: u32,
    pub T38FaxMaxDatagram: u32,
    pub T38FaxUdpEC: *const c_char,
    pub T38VendorInfo: *const c_char,
    pub remote_ip: *const c_char,
    pub remote_port: u16,
    pub local_ip: *const c_char,
    pub local_port: u16,
    pub sdp_o_line: *const c_char,
}
impl std::default::Default for t38_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum stack {
    BOTTOM = 1,
    TOP = 2,
    NODUP = 4,
    UNSHIFT = 8,
    PUSH = 16,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum status {
    SUCCESS = 0,
    FALSE = 1,
    TIMEOUT = 2,
    RESTART = 3,
    INTR = 4,
    NOTIMPL = 5,
    MEMERR = 6,
    NOOP = 7,
    RESAMPLE = 8,
    GENERR = 9,
    INUSE = 10,
    BREAK = 11,
    SOCKERR = 12,
    MORE_DATA = 13,
    NOTFOUND = 14,
    UNLOAD = 15,
    NOUNLOAD = 16,
    IGNORE = 17,
    TOO_SMALL = 18,
    FOUND = 19,
    CONTINUE = 20,
    TERM = 21,
    NOT_INITALIZED = 22,
    TOO_LATE = 23,
    XBREAK = 35,
    WINBREAK = 730035,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum log_level {
    DEBUG10 = 110,
    DEBUG9 = 109,
    DEBUG8 = 108,
    DEBUG7 = 107,
    DEBUG6 = 106,
    DEBUG5 = 105,
    DEBUG4 = 104,
    DEBUG3 = 103,
    DEBUG2 = 102,
    DEBUG1 = 101,
    DEBUG = 7,
    INFO = 6,
    NOTICE = 5,
    WARNING = 4,
    ERROR = 3,
    CRIT = 2,
    ALERT = 1,
    CONSOLE = 0,
    INVALID = 64,
    UNINIT = 1000,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum text_channel {
    ID_LOG = 0,
    ID_LOG_CLEAN = 1,
    ID_EVENT = 2,
    ID_SESSION = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_session_message_flag_enum {
    DYNAMIC = 1,
    FREE_STRING_REPLY = 2,
    FREE_POINTER_REPLY = 4,
}
pub type core_session_message_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum channel_callstate {
    DOWN = 0,
    DIALING = 1,
    RINGING = 2,
    EARLY = 3,
    ACTIVE = 4,
    HELD = 5,
    RING_WAIT = 6,
    HANGUP = 7,
    UNHELD = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum device_state {
    DOWN = 0,
    RINGING = 1,
    ACTIVE = 2,
    ACTIVE_MULTI = 3,
    HELD = 4,
    UNHELD = 5,
    HANGUP = 6,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum channel_state {
    NEW = 0,
    INIT = 1,
    ROUTING = 2,
    SOFT_EXECUTE = 3,
    EXECUTE = 4,
    EXCHANGE_MEDIA = 5,
    PARK = 6,
    CONSUME_MEDIA = 7,
    HIBERNATE = 8,
    RESET = 9,
    HANGUP = 10,
    REPORTING = 11,
    DESTROY = 12,
    NONE = 13,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ring_ready {
    NONE = 0,
    RINGING = 1,
    QUEUED = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum channel_cap {
    MEDIA_ACK = 1,
    BYPASS_MEDIA = 2,
    PROXY_MEDIA = 3,
    JITTERBUFFER = 4,
    FS_RTP = 5,
    QUEUEABLE_DTMF_DELAY = 6,
    FLAG_MAX = 7,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum channel_flag {
    ANSWERED = 1,
    OUTBOUND = 2,
    EARLY_MEDIA = 3,
    BRIDGE_ORIGINATOR = 4,
    UUID_BRIDGE_ORIGINATOR = 5,
    TRANSFER = 6,
    ACCEPT_CNG = 7,
    REDIRECT = 8,
    BRIDGED = 9,
    HOLD = 10,
    SERVICE = 11,
    TAGGED = 12,
    WINNER = 13,
    CONTROLLED = 14,
    PROXY_MODE = 15,
    PROXY_OFF = 16,
    SUSPEND = 17,
    EVENT_PARSE = 18,
    GEN_RINGBACK = 19,
    RING_READY = 20,
    BREAK = 21,
    BROADCAST = 22,
    UNICAST = 23,
    VIDEO = 24,
    EVENT_LOCK = 25,
    EVENT_LOCK_PRI = 26,
    RESET = 27,
    ORIGINATING = 28,
    STOP_BROADCAST = 29,
    PROXY_MEDIA = 30,
    INNER_BRIDGE = 31,
    REQ_MEDIA = 32,
    VERBOSE_EVENTS = 33,
    PAUSE_BUGS = 34,
    DIVERT_EVENTS = 35,
    BLOCK_STATE = 36,
    FS_RTP = 37,
    REPORTING = 38,
    PARK = 39,
    TIMESTAMP_SET = 40,
    ORIGINATOR = 41,
    XFER_ZOMBIE = 42,
    MEDIA_ACK = 43,
    THREAD_SLEEPING = 44,
    DISABLE_RINGBACK = 45,
    NOT_READY = 46,
    SIGNAL_BRIDGE_TTL = 47,
    MEDIA_BRIDGE_TTL = 48,
    BYPASS_MEDIA_AFTER_BRIDGE = 49,
    LEG_HOLDING = 50,
    BROADCAST_DROP_MEDIA = 51,
    EARLY_HANGUP = 52,
    MEDIA_SET = 53,
    CONSUME_ON_ORIGINATE = 54,
    PASSTHRU_PTIME_MISMATCH = 55,
    BRIDGE_NOWRITE = 56,
    RECOVERED = 57,
    JITTERBUFFER = 58,
    JITTERBUFFER_PLC = 59,
    DIALPLAN = 60,
    BLEG = 61,
    BLOCK_BROADCAST_UNTIL_MEDIA = 62,
    CNG_PLC = 63,
    ATTENDED_TRANSFER = 64,
    LAZY_ATTENDED_TRANSFER = 65,
    SIGNAL_DATA = 66,
    SIMPLIFY = 67,
    ZOMBIE_EXEC = 68,
    INTERCEPT = 69,
    INTERCEPTED = 70,
    VIDEO_REFRESH_REQ = 71,
    SERVICE_AUDIO = 72,
    SERVICE_VIDEO = 73,
    ZRTP_PASSTHRU_REQ = 74,
    ZRTP_PASSTHRU = 75,
    ZRTP_HASH = 76,
    CHANNEL_SWAP = 77,
    DEVICE_LEG = 78,
    FINAL_DEVICE_LEG = 79,
    PICKUP = 80,
    CONFIRM_BLIND_TRANSFER = 81,
    NO_PRESENCE = 82,
    CONFERENCE = 83,
    CONFERENCE_ADV = 84,
    RECOVERING = 85,
    RECOVERING_BRIDGE = 86,
    TRACKED = 87,
    TRACKABLE = 88,
    NO_CDR = 89,
    EARLY_OK = 90,
    MEDIA_TRANS = 91,
    HOLD_ON_BRIDGE = 92,
    SECURE = 93,
    LIBERAL_DTMF = 94,
    SLA_BARGE = 95,
    SLA_BARGING = 96,
    PROTO_HOLD = 97,
    HOLD_LOCK = 98,
    VIDEO_POSSIBLE = 99,
    NOTIMER_DURING_BRIDGE = 100,
    PASS_RFC2833 = 101,
    T38_PASSTHRU = 102,
    DROP_DTMF = 103,
    REINVITE = 104,
    AUTOFLUSH_DURING_BRIDGE = 105,
    RTP_NOTIMER_DURING_BRIDGE = 106,
    AVPF = 107,
    AVPF_MOZ = 108,
    ICE = 109,
    DTLS = 110,
    VERBOSE_SDP = 111,
    DTLS_OK = 112,
    _3PCC = 113,
    VIDEO_PASSIVE = 114,
    NOVIDEO = 115,
    VIDEO_BITRATE_UNMANAGABLE = 116,
    VIDEO_ECHO = 117,
    VIDEO_BLANK = 118,
    VIDEO_WRITING = 119,
    SLA_INTERCEPT = 120,
    VIDEO_BREAK = 121,
    AUDIO_PAUSE = 122,
    VIDEO_PAUSE_READ = 123,
    VIDEO_PAUSE_WRITE = 124,
    BYPASS_MEDIA_AFTER_HOLD = 125,
    HANGUP_HELD = 126,
    CONFERENCE_RESET_MEDIA = 127,
    VIDEO_DECODED_READ = 128,
    VIDEO_DEBUG_READ = 129,
    VIDEO_DEBUG_WRITE = 130,
    VIDEO_ONLY = 131,
    VIDEO_READY = 132,
    VIDEO_MIRROR_INPUT = 133,
    VIDEO_READ_FILE_ATTACHED = 134,
    VIDEO_WRITE_FILE_ATTACHED = 135,
    _3P_MEDIA_REQUESTED = 136,
    _3P_NOMEDIA_REQUESTED = 137,
    _3P_NOMEDIA_REQUESTED_BLEG = 138,
    VIDEO_SDP_RECVD = 139,
    FLAG_MAX = 140,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct vid_params {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}
impl std::default::Default for vid_params {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum channel_app_flag {
    TAGGED = 1,
    T38 = 2,
    T38_REQ = 4,
    T38_FAIL = 8,
    T38_NEGOTIATED = 16,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum frame_flag_enum {
    NONE = 0,
    CNG = 1,
    RAW_RTP = 2,
    RTP_HEADER = 4,
    PLC = 8,
    RFC2833 = 16,
    PROXY_PACKET = 32,
    DYNAMIC = 64,
    ZRTP = 128,
    UDPTL_PACKET = 256,
    NOT_AUDIO = 512,
    RTCP = 1024,
    MARKER = 2048,
    WAIT_KEY_FRAME = 4096,
    RAW_RTP_PARSE_FRAME = 8192,
    PICTURE_RESET = 16384,
    SAME_IMAGE = 32768,
    USE_VIDEO_TIMESTAMP = 65536,
    ENCODED = 131072,
}
pub type frame_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum application_flag_enum {
    NONE = 0,
    SUPPORT_NOMEDIA = 1,
    ROUTING_EXEC = 2,
    MEDIA_TAP = 4,
    ZOMBIE_EXEC = 8,
    NO_LOOPBACK = 16,
}
pub type application_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum chat_application_flag_enum {
    NONE = 0,
}
pub type chat_application_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum signal {
    NONE = 0,
    KILL = 1,
    XFER = 2,
    BREAK = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum codec_flag_enum {
    ENCODE = 1,
    DECODE = 2,
    SILENCE_START = 4,
    SILENCE_STOP = 8,
    SILENCE = 16,
    FREE_POOL = 32,
    AAL2 = 64,
    PASSTHROUGH = 128,
    READY = 256,
    HAS_PLC = 32768,
    VIDEO_PATCHING = 65536,
}
pub type codec_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum speech_flag_enum {
    NONE = 0,
    HASTEXT = 1,
    PEEK = 2,
    FREE_POOL = 4,
    BLOCKING = 8,
    PAUSE = 16,
    OPEN = 32,
    DONE = 64,
}
pub type speech_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum asr_flag_enum {
    NONE = 0,
    DATA = 1,
    FREE_POOL = 2,
    CLOSED = 4,
    FIRE_EVENTS = 8,
    AUTO_RESUME = 16,
}
pub type asr_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum directory_flag_enum {
    FREE_POOL = 1,
}
pub type directory_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum codec_type {
    AUDIO = 0,
    VIDEO = 1,
    T38 = 2,
    APP = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum media_type {
    AUDIO = 0,
    VIDEO = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum switch_timer_flag_enum {
    FREE_POOL = 1,
}
pub type switch_timer_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum media_bug_flag_enum {
    BOTH = 0,
    READ_STREAM = 1,
    WRITE_STREAM = 2,
    WRITE_REPLACE = 4,
    READ_REPLACE = 8,
    READ_PING = 16,
    STEREO = 32,
    ANSWER_REQ = 64,
    BRIDGE_REQ = 128,
    THREAD_LOCK = 256,
    PRUNE = 512,
    NO_PAUSE = 1024,
    STEREO_SWAP = 2048,
    LOCK = 4096,
    TAP_NATIVE_READ = 8192,
    TAP_NATIVE_WRITE = 16384,
    ONE_ONLY = 32768,
    MASK = 65536,
    READ_VIDEO_PING = 131072,
    WRITE_VIDEO_PING = 262144,
    READ_VIDEO_STREAM = 524288,
    WRITE_VIDEO_STREAM = 1048576,
    VIDEO_PATCH = 2097152,
    SPY_VIDEO_STREAM = 4194304,
    SPY_VIDEO_STREAM_BLEG = 8388608,
    READ_VIDEO_PATCH = 16777216,
}
pub type media_bug_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum file_flag_enum {
    FLAG_READ = 1,
    FLAG_WRITE = 2,
    FLAG_FREE_POOL = 4,
    DATA_SHORT = 8,
    DATA_INT = 16,
    DATA_FLOAT = 32,
    DATA_DOUBLE = 64,
    DATA_RAW = 128,
    PAUSE = 256,
    NATIVE = 512,
    SEEK = 1024,
    OPEN = 2048,
    CALLBACK = 4096,
    DONE = 8192,
    BUFFER_DONE = 16384,
    WRITE_APPEND = 32768,
    WRITE_OVER = 65536,
    NOMUX = 131072,
    BREAK_ON_CHANGE = 262144,
    FLAG_VIDEO = 524288,
    FLAG_VIDEO_EOF = 1048576,
}
pub type file_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum io_flag_enum {
    NONE = 0,
    NOBLOCK = 1,
    SINGLE_READ = 2,
    FORCE = 4,
}
pub type io_flag = u32;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum event_types {
    CUSTOM = 0,
    CLONE = 1,
    CHANNEL_CREATE = 2,
    CHANNEL_DESTROY = 3,
    CHANNEL_STATE = 4,
    CHANNEL_CALLSTATE = 5,
    CHANNEL_ANSWER = 6,
    CHANNEL_HANGUP = 7,
    CHANNEL_HANGUP_COMPLETE = 8,
    CHANNEL_EXECUTE = 9,
    CHANNEL_EXECUTE_COMPLETE = 10,
    CHANNEL_HOLD = 11,
    CHANNEL_UNHOLD = 12,
    CHANNEL_BRIDGE = 13,
    CHANNEL_UNBRIDGE = 14,
    CHANNEL_PROGRESS = 15,
    CHANNEL_PROGRESS_MEDIA = 16,
    CHANNEL_OUTGOING = 17,
    CHANNEL_PARK = 18,
    CHANNEL_UNPARK = 19,
    CHANNEL_APPLICATION = 20,
    CHANNEL_ORIGINATE = 21,
    CHANNEL_UUID = 22,
    API = 23,
    LOG = 24,
    INBOUND_CHAN = 25,
    OUTBOUND_CHAN = 26,
    STARTUP = 27,
    SHUTDOWN = 28,
    PUBLISH = 29,
    UNPUBLISH = 30,
    TALK = 31,
    NOTALK = 32,
    SESSION_CRASH = 33,
    MODULE_LOAD = 34,
    MODULE_UNLOAD = 35,
    DTMF = 36,
    MESSAGE = 37,
    PRESENCE_IN = 38,
    NOTIFY_IN = 39,
    PRESENCE_OUT = 40,
    PRESENCE_PROBE = 41,
    MESSAGE_WAITING = 42,
    MESSAGE_QUERY = 43,
    ROSTER = 44,
    CODEC = 45,
    BACKGROUND_JOB = 46,
    DETECTED_SPEECH = 47,
    DETECTED_TONE = 48,
    PRIVATE_COMMAND = 49,
    HEARTBEAT = 50,
    TRAP = 51,
    ADD_SCHEDULE = 52,
    DEL_SCHEDULE = 53,
    EXE_SCHEDULE = 54,
    RE_SCHEDULE = 55,
    RELOADXML = 56,
    NOTIFY = 57,
    PHONE_FEATURE = 58,
    PHONE_FEATURE_SUBSCRIBE = 59,
    SEND_MESSAGE = 60,
    RECV_MESSAGE = 61,
    REQUEST_PARAMS = 62,
    CHANNEL_DATA = 63,
    GENERAL = 64,
    COMMAND = 65,
    SESSION_HEARTBEAT = 66,
    CLIENT_DISCONNECTED = 67,
    SERVER_DISCONNECTED = 68,
    SEND_INFO = 69,
    RECV_INFO = 70,
    RECV_RTCP_MESSAGE = 71,
    CALL_SECURE = 72,
    NAT = 73,
    RECORD_START = 74,
    RECORD_STOP = 75,
    PLAYBACK_START = 76,
    PLAYBACK_STOP = 77,
    CALL_UPDATE = 78,
    FAILURE = 79,
    SOCKET_DATA = 80,
    MEDIA_BUG_START = 81,
    MEDIA_BUG_STOP = 82,
    CONFERENCE_DATA_QUERY = 83,
    CONFERENCE_DATA = 84,
    CALL_SETUP_REQ = 85,
    CALL_SETUP_RESULT = 86,
    CALL_DETAIL = 87,
    DEVICE_STATE = 88,
    ALL = 89,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum input_type {
    INPUT_TYPE_DTMF = 0,
    INPUT_TYPE_EVENT = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum call_cause {
    NONE = 0,
    UNALLOCATED_NUMBER = 1,
    NO_ROUTE_TRANSIT_NET = 2,
    NO_ROUTE_DESTINATION = 3,
    CHANNEL_UNACCEPTABLE = 6,
    CALL_AWARDED_DELIVERED = 7,
    NORMAL_CLEARING = 16,
    USER_BUSY = 17,
    NO_USER_RESPONSE = 18,
    NO_ANSWER = 19,
    SUBSCRIBER_ABSENT = 20,
    CALL_REJECTED = 21,
    NUMBER_CHANGED = 22,
    REDIRECTION_TO_NEW_DESTINATION = 23,
    EXCHANGE_ROUTING_ERROR = 25,
    DESTINATION_OUT_OF_ORDER = 27,
    INVALID_NUMBER_FORMAT = 28,
    FACILITY_REJECTED = 29,
    RESPONSE_TO_STATUS_ENQUIRY = 30,
    NORMAL_UNSPECIFIED = 31,
    NORMAL_CIRCUIT_CONGESTION = 34,
    NETWORK_OUT_OF_ORDER = 38,
    NORMAL_TEMPORARY_FAILURE = 41,
    CONGESTION = 42,
    ACCESS_INFO_DISCARDED = 43,
    REQUESTED_CHAN_UNAVAIL = 44,
    PRE_EMPTED = 45,
    FACILITY_NOT_SUBSCRIBED = 50,
    OUTGOING_CALL_BARRED = 52,
    INCOMING_CALL_BARRED = 54,
    BEARERCAPABILITY_NOTAUTH = 57,
    BEARERCAPABILITY_NOTAVAIL = 58,
    SERVICE_UNAVAILABLE = 63,
    BEARERCAPABILITY_NOTIMPL = 65,
    CHAN_NOT_IMPLEMENTED = 66,
    FACILITY_NOT_IMPLEMENTED = 69,
    SERVICE_NOT_IMPLEMENTED = 79,
    INVALID_CALL_REFERENCE = 81,
    INCOMPATIBLE_DESTINATION = 88,
    INVALID_MSG_UNSPECIFIED = 95,
    MANDATORY_IE_MISSING = 96,
    MESSAGE_TYPE_NONEXIST = 97,
    WRONG_MESSAGE = 98,
    IE_NONEXIST = 99,
    INVALID_IE_CONTENTS = 100,
    WRONG_CALL_STATE = 101,
    RECOVERY_ON_TIMER_EXPIRE = 102,
    MANDATORY_IE_LENGTH_ERROR = 103,
    PROTOCOL_ERROR = 111,
    INTERWORKING = 127,
    SUCCESS = 142,
    ORIGINATOR_CANCEL = 487,
    CRASH = 500,
    SYSTEM_SHUTDOWN = 501,
    LOSE_RACE = 502,
    MANAGER_REQUEST = 503,
    BLIND_TRANSFER = 600,
    ATTENDED_TRANSFER = 601,
    ALLOTTED_TIMEOUT = 602,
    USER_CHALLENGE = 603,
    MEDIA_TIMEOUT = 604,
    PICKED_OFF = 605,
    USER_NOT_REGISTERED = 606,
    PROGRESS_TIMEOUT = 607,
    INVALID_GATEWAY = 608,
    GATEWAY_DOWN = 609,
    INVALID_URL = 610,
    INVALID_PROFILE = 611,
    NO_PICKUP = 612,
    SRTP_READ_ERROR = 613,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum session_ctl {
    PAUSE_INBOUND = 0,
    PAUSE_OUTBOUND = 1,
    PAUSE_ALL = 2,
    HUPALL = 3,
    SHUTDOWN = 4,
    CHECK_RUNNING = 5,
    LOGLEVEL = 6,
    SPS = 7,
    LAST_SPS = 8,
    RECLAIM = 9,
    MAX_SESSIONS = 10,
    SYNC_CLOCK = 11,
    MAX_DTMF_DURATION = 12,
    MIN_DTMF_DURATION = 13,
    DEFAULT_DTMF_DURATION = 14,
    SHUTDOWN_ELEGANT = 15,
    SHUTDOWN_ASAP = 16,
    CANCEL_SHUTDOWN = 17,
    SEND_SIGHUP = 18,
    DEBUG_LEVEL = 19,
    FLUSH_DB_HANDLES = 20,
    SHUTDOWN_NOW = 21,
    REINCARNATE_NOW = 22,
    CALIBRATE_CLOCK = 23,
    SAVE_HISTORY = 24,
    CRASH = 25,
    MIN_IDLE_CPU = 26,
    VERBOSE_EVENTS = 27,
    SHUTDOWN_CHECK = 28,
    PAUSE_INBOUND_CHECK = 29,
    PAUSE_OUTBOUND_CHECK = 30,
    PAUSE_CHECK = 31,
    READY_CHECK = 32,
    THREADED_SYSTEM_EXEC = 33,
    SYNC_CLOCK_WHEN_IDLE = 34,
    DEBUG_SQL = 35,
    SQL = 36,
    API_EXPANSION = 37,
    RECOVER = 38,
    SPS_PEAK = 39,
    SPS_PEAK_FIVEMIN = 40,
    SESSIONS_PEAK = 41,
    SESSIONS_PEAK_FIVEMIN = 42,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum state_handler_flag {
    STICKY = 1,
}
pub type os_socket = c_int;
pub enum apr_pool { }
pub type memory_pool = apr_pool;
pub type port = u16;
pub type payload = u8;
pub enum rtp { }
pub enum rtcp { }
pub enum event_subclass { }
pub enum event_node { }
pub enum loadable_module { }
pub enum channel { }
pub enum sql_queue_manager { }
pub enum core_session { }
pub enum buffer { }
pub enum odbc_handle { }
pub enum pgsql_handle { }
pub enum pgsql_result { }
pub enum core_port_allocator { }
pub enum media_bug { }
pub type hashtable_destructor = Option<unsafe extern "C" fn(ptr: *mut c_void)>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct console_callback_match_node {
    pub val: *mut c_char,
    pub next: *mut console_callback_match_node,
}
impl std::default::Default for console_callback_match_node {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct console_callback_match {
    pub head: *mut console_callback_match_node,
    pub end: *mut console_callback_match_node,
    pub count: c_int,
    pub dynamic: c_int,
}
impl std::default::Default for console_callback_match {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type media_bug_exec_cb = Option<unsafe extern "C" fn(bug: *mut media_bug,
                                                         user_data: *mut c_void)>;
pub type core_video_thread_callback_func = Option<unsafe extern "C" fn(session: *mut core_session,
                                                                       frame: *mut frame,
                                                                       user_data: *mut c_void)
                                                                       -> status>;
pub type cap_callback = Option<unsafe extern "C" fn(var: *const c_char,
                                                    val: *const c_char,
                                                    user_data: *mut c_void)>;
pub type console_complete_callback =
    Option<unsafe extern "C" fn(arg1: *const c_char,
                                arg2: *const c_char,
                                matches: *mut *mut console_callback_match)
                                -> status>;
pub type media_bug_callback = Option<unsafe extern "C" fn(arg1: *mut media_bug,
                                                          arg2: *mut c_void,
                                                          arg3: abc_type)
                                                          -> switch_bool>;
pub type tone_detect_callback = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                            arg2: *const c_char,
                                                            arg3: *const c_char)
                                                            -> switch_bool>;
pub enum xml_binding { }
pub type video_function = Option<unsafe extern "C" fn(session: *mut core_session,
                                                      user_data: *mut c_void)>;
pub type core_codec_encode_func = Option<unsafe extern "C" fn(codec: *mut codec,
                                                              other_codec: *mut codec,
                                                              decoded_data: *mut c_void,
                                                              decoded_data_len: u32,
                                                              decoded_rate: u32,
                                                              encoded_data: *mut c_void,
                                                              encoded_data_len: *mut u32,
                                                              encoded_rate: *mut u32,
                                                              flag: *mut c_uint)
                                                              -> status>;
pub type core_codec_decode_func = Option<unsafe extern "C" fn(codec: *mut codec,
                                                              other_codec: *mut codec,
                                                              encoded_data: *mut c_void,
                                                              encoded_data_len: u32,
                                                              encoded_rate: u32,
                                                              decoded_data: *mut c_void,
                                                              decoded_data_len: *mut u32,
                                                              decoded_rate: *mut u32,
                                                              flag: *mut c_uint)
                                                              -> status>;
pub type core_codec_video_encode_func = Option<unsafe extern "C" fn(codec: *mut codec,
                                                                    frame: *mut frame)
                                                                    -> status>;
pub type core_codec_video_decode_func = Option<unsafe extern "C" fn(codec: *mut codec,
                                                                    frame: *mut frame)
                                                                    -> status>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum codec_control_command {
    SCC_VIDEO_GEN_KEYFRAME = 0,
    SCC_VIDEO_BANDWIDTH = 1,
    SCC_VIDEO_RESET = 2,
    SCC_AUDIO_PACKET_LOSS = 3,
    SCC_DEBUG = 4,
    SCC_CODEC_SPECIFIC = 5,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum codec_control_type {
    SCCT_NONE = 0,
    SCCT_STRING = 1,
    SCCT_INT = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum io_type {
    IO_READ = 0,
    IO_WRITE = 1,
}
pub type core_codec_control_func = Option<unsafe extern "C" fn(codec: *mut codec,
                                                               cmd: codec_control_command,
                                                               ctype: codec_control_type,
                                                               cmd_data: *mut c_void,
                                                               atype: codec_control_type,
                                                               cmd_arg: *mut c_void,
                                                               rtype: *mut codec_control_type,
                                                               ret_data: *mut *mut c_void)
                                                               -> status>;
pub type core_codec_init_func = Option<unsafe extern "C" fn(arg1: *mut codec,
                                                            arg2: codec_flag,
                                                            codec_settings: *const codec_settings)
                                                            -> status>;
pub type core_codec_fmtp_parse_func = Option<unsafe extern "C" fn(fmtp: *const c_char,
                                                                  codec_fmtp: *mut codec_fmtp)
                                                                  -> status>;
pub type core_codec_destroy_func = Option<unsafe extern "C" fn(arg1: *mut codec) -> status>;
pub type chat_application_function = Option<unsafe extern "C" fn(arg1: *mut event,
                                                                 arg2: *const c_char)
                                                                 -> status>;
pub type application_function = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                            arg2: *const c_char)>;
pub type core_recover_callback = Option<unsafe extern "C" fn(session: *mut core_session) -> c_int>;
pub type event_callback = Option<unsafe extern "C" fn(arg1: *mut event)>;
pub type dialplan_hunt_function = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                              arg2: *mut c_void,
                                                              arg3: *mut caller_profile)
                                                              -> *mut caller_extension>;
pub type hash_delete_callback = Option<unsafe extern "C" fn(key: *const c_void,
                                                            val: *const c_void,
                                                            pData: *mut c_void)
                                                            -> switch_bool>;
pub type scheduler_func = Option<unsafe extern "C" fn(task: *mut scheduler_task)>;
pub type state_handler = Option<unsafe extern "C" fn(arg1: *mut core_session) -> status>;
pub type stream_handle_read_function = Option<unsafe extern "C" fn(handle: *mut stream_handle,
                                                                   len: *mut c_int)
                                                                   -> *mut u8>;
pub type stream_handle_write_function = Option<unsafe extern "C" fn(handle: *mut stream_handle,
                                                                    fmt: *const c_char,
                                                                    ...)
                                                                    -> status>;
pub type stream_handle_raw_write_function =
    Option<unsafe extern "C" fn(handle: *mut stream_handle, data: *mut u8, datalen: usize) -> status>;
pub type api_function = Option<unsafe extern "C" fn(cmd: *const c_char,
                                                    session: *mut core_session,
                                                    stream: *mut stream_handle)
                                                    -> status>;
pub type json_api_function = Option<unsafe extern "C" fn(json: *const cJSON,
                                                         session: *mut core_session,
                                                         json_reply: *mut *mut cJSON)
                                                         -> status>;
pub type input_callback_function = Option<unsafe extern "C" fn(session: *mut core_session,
                                                               input: *mut c_void,
                                                               input_type: input_type,
                                                               buf: *mut c_void,
                                                               buflen: c_uint)
                                                               -> status>;
pub type read_frame_callback_function = Option<unsafe extern "C" fn(session: *mut core_session,
                                                                    frame: *mut frame,
                                                                    user_data: *mut c_void)
                                                                    -> status>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dm_match_type {
    POSITIVE = 0,
    NEGATIVE = 1,
}
pub enum ivr_dmachine { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ivr_dmachine_match {
    pub dmachine: *mut ivr_dmachine,
    pub match_digits: *const c_char,
    pub match_key: i32,
    pub type_: dm_match_type,
    pub user_data: *mut c_void,
}
impl std::default::Default for ivr_dmachine_match {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type ivr_dmachine_callback = Option<unsafe extern "C" fn(match_: *mut ivr_dmachine_match)
                                                             -> status>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct input_args {
    pub input_callback: input_callback_function,
    pub buf: *mut c_void,
    pub buflen: u32,
    pub read_frame_callback: read_frame_callback_function,
    pub user_data: *mut c_void,
    pub dmachine: *mut ivr_dmachine,
    pub loops: c_int,
}
impl std::default::Default for input_args {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct say_args {
    pub type_: say_type,
    pub method: say_method,
    pub gender: say_gender,
    pub ext: *const c_char,
}
impl std::default::Default for say_args {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type say_callback = Option<unsafe extern "C" fn(session: *mut core_session,
                                                    tosay: *mut c_char,
                                                    say_args: *mut say_args,
                                                    args: *mut input_args)
                                                    -> status>;
pub type say_string_callback = Option<unsafe extern "C" fn(session: *mut core_session,
                                                           tosay: *mut c_char,
                                                           say_args: *mut say_args,
                                                           rstr: *mut *mut c_char)
                                                           -> status>;
pub enum say_file_handle { }
pub type new_say_callback = Option<unsafe extern "C" fn(sh: *mut say_file_handle,
                                                        tosay: *mut c_char,
                                                        say_args: *mut say_args)
                                                        -> status>;
pub type xml_t = *mut xml;
pub type xml_open_root_function = Option<unsafe extern "C" fn(reload: u8,
                                                              err: *mut *const c_char,
                                                              user_data: *mut c_void)
                                                              -> xml_t>;
pub type xml_search_function = Option<unsafe extern "C" fn(section: *const c_char,
                                                           tag_name: *const c_char,
                                                           key_name: *const c_char,
                                                           key_value: *const c_char,
                                                           params: *mut event,
                                                           user_data: *mut c_void)
                                                           -> xml_t>;
pub enum hashtable { }
pub enum hashtable_iterator { }
pub type hash = hashtable;
pub type inthash = hashtable;
pub type hash_index = hashtable_iterator;
pub enum network_list { }
pub type module_load =
    Option<unsafe extern "C" fn(module_interface: *mut *mut loadable_module_interface,
                                pool: *mut memory_pool)
                                -> status>;
pub type module_runtime = Option<extern "C" fn() -> status>;
pub type module_shutdown = Option<extern "C" fn() -> status>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum thread_priority {
    LOW = 1,
    NORMAL = 10,
    IMPORTANT = 50,
    REALTIME = 99,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum module_flag_enum {
    NONE = 0,
    GLOBAL_SYMBOLS = 1,
}
pub type module_flag = u32;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct loadable_module_function_table {
    pub api_version: c_int,
    pub load: module_load,
    pub shutdown: module_shutdown,
    pub runtime: module_runtime,
    pub flags: module_flag,
}
impl std::default::Default for loadable_module_function_table {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type modulename_callback_func = Option<unsafe extern "C" fn(user_data: *mut c_void,
                                                                module_name: *const c_char)
                                                                -> c_int>;
pub enum ivr_digit_stream_parser { }
pub enum media_handle { }
pub type event_channel_id = u32;
pub type event_channel_func = Option<unsafe extern "C" fn(event_channel: *const c_char,
                                                          json: *mut cJSON,
                                                          key: *const c_char,
                                                          id: event_channel_id)>;
pub enum live_array { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum sdp_type {
    REQUEST = 0,
    RESPONSE = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtp_crypto_key_type {
    AEAD_AES_256_GCM_8 = 0,
    AEAD_AES_128_GCM_8 = 1,
    AES_CM_256_HMAC_SHA1_80 = 2,
    AES_CM_192_HMAC_SHA1_80 = 3,
    AES_CM_128_HMAC_SHA1_80 = 4,
    AES_CM_256_HMAC_SHA1_32 = 5,
    AES_CM_192_HMAC_SHA1_32 = 6,
    AES_CM_128_HMAC_SHA1_32 = 7,
    AES_CM_128_NULL_AUTH = 8,
    CRYPTO_INVALID = 9,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct payload_map {
    pub type_: media_type,
    pub sdp_type: sdp_type,
    pub ptime: u32,
    pub rate: u32,
    pub allocated: u8,
    pub negotiated: u8,
    pub current: u8,
    pub hash: c_ulong,
    pub rm_encoding: *mut c_char,
    pub iananame: *mut c_char,
    pub modname: *mut c_char,
    pub pt: payload,
    pub rm_rate: c_ulong,
    pub adv_rm_rate: c_ulong,
    pub codec_ms: u32,
    pub bitrate: u32,
    pub rm_fmtp: *mut c_char,
    pub agreed_pt: payload,
    pub recv_pt: payload,
    pub fmtp_out: *mut c_char,
    pub remote_sdp_ip: *mut c_char,
    pub remote_sdp_port: port,
    pub channels: c_int,
    pub adv_channels: c_int,
    pub next: *mut payload_map,
}
impl std::default::Default for payload_map {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum media_flow {
    SENDRECV = 0,
    SENDONLY = 1,
    RECVONLY = 2,
    INACTIVE = 3,
    DISABLED = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_media_ice_type {
    GOOGLE_JINGLE = 1,
    VANILLA = 2,
    CONTROLLED = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum poll {
    READ = 1,
    WRITE = 2,
    ERROR = 4,
    HUP = 8,
    RDNORM = 16,
    RDBAND = 32,
    PRI = 64,
    INVALID = 128,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct waitlist {
    pub sock: os_socket,
    pub events: u32,
    pub revents: u32,
}
impl std::default::Default for waitlist {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum jb { }
pub enum img_txt_handle { }
pub enum frame_buffer { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum video_read_flag {
    BLOCK = 1,
    FLUSH = 2,
    CHECK = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum vid_spy_fmt {
    LOWER_RIGHT_SMALL = 0,
    LOWER_RIGHT_LARGE = 1,
    DUAL_CROP = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum file_command {
    FLUSH_AUDIO = 0,
    PAUSE_READ = 1,
}
pub type thread_id = c_ulong;
pub type interval_time_t = i64;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct time_exp {
    pub tm_usec: i32,
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i32,
}
impl std::default::Default for time_exp {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum apr_thread_mutex { }
pub type mutex = apr_thread_mutex;
pub enum apr_thread_rwlock { }
pub type thread_rwlock = apr_thread_rwlock;
pub enum apr_thread_cond { }
pub type thread_cond = apr_thread_cond;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct uuid {
    pub data: [c_uchar; 16usize],
}
impl std::default::Default for uuid {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum apr_queue { }
pub type queue = apr_queue;
pub enum apr_file { }
pub type file = apr_file;
pub type fileperms = i32;
pub type seek_where = c_int;
pub enum dir { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct array_header {
    pub pool: *mut memory_pool,
    pub elt_size: c_int,
    pub nelts: c_int,
    pub nalloc: c_int,
    pub elts: *mut c_char,
}
impl std::default::Default for array_header {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum apr_thread { }
pub type thread = apr_thread;
pub enum apr_threadattr { }
pub type threadattr = apr_threadattr;
pub type thread_start = Option<unsafe extern "C" fn(arg1: *mut thread, arg2: *mut c_void)
                                                    -> *mut c_void>;
pub enum apr_socket { }
pub type socket = apr_socket;
pub enum sockaddr {}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum shutdown_how_e {
    READ = 0,
    WRITE = 1,
    READWRITE = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum pollset_type {
    NO_DESC = 0,
    SOCKET = 1,
    FILE = 2,
    LASTDESC = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct descriptor {
    pub _bindgen_data_: [u64; 1usize],
}
impl descriptor {
    pub unsafe fn f(&mut self) -> *mut *mut file {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s(&mut self) -> *mut *mut socket {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
}
impl std::default::Default for descriptor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct pollfd {
    pub p: *mut memory_pool,
    pub desc_type: pollset_type,
    pub reqevents: i16,
    pub rtnevents: i16,
    pub desc: descriptor,
    pub client_data: *mut c_void,
}
impl std::default::Default for pollfd {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum apr_pollset { }
pub type pollset = apr_pollset;
pub enum sqlite3 { }
pub type core_db = sqlite3;
pub enum sqlite3_stmt { }
pub type core_db_stmt = sqlite3_stmt;
pub type core_db_callback_func = Option<unsafe extern "C" fn(pArg: *mut c_void,
                                                             argc: c_int,
                                                             argv: *mut *mut c_char,
                                                             columnNames: *mut *mut c_char)
                                                             -> c_int>;
pub type core_db_err_callback_func = Option<unsafe extern "C" fn(pArg: *mut c_void,
                                                                 errmsg: *const c_char)
                                                                 -> c_int>;
pub type core_db_destructor_type = Option<unsafe extern "C" fn(arg1: *mut c_void)>;
pub type dso_func = Option<extern "C" fn() -> c_int>;
pub type dso_lib = *mut c_void;
pub type dso_data = *mut c_void;
pub enum real_pcre { }
pub type regex = real_pcre;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct core_time_duration {
    pub mms: u32,
    pub ms: u32,
    pub sec: u32,
    pub min: u32,
    pub hr: u32,
    pub day: u32,
    pub yr: u32,
}
impl std::default::Default for core_time_duration {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct app_log {
    pub app: *mut c_char,
    pub arg: *mut c_char,
    pub stamp: time_t,
    pub next: *mut app_log,
}
impl std::default::Default for app_log {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct thread_data {
    pub func: thread_start,
    pub obj: *mut c_void,
    pub alloc: c_int,
    pub pool: *mut memory_pool,
}
impl std::default::Default for thread_data {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct hold_record {
    pub on: time_t,
    pub off: time_t,
    pub uuid: *mut c_char,
    pub next: *mut hold_record,
}
impl std::default::Default for hold_record {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct device_uuid_node {
    pub uuid: *mut c_char,
    pub xml_cdr: xml_t,
    pub event: *mut event,
    pub callstate: channel_callstate,
    pub hold_record: *mut hold_record,
    pub hup_profile: *mut caller_profile,
    pub direction: call_direction,
    pub parent: *mut device_record,
    pub next: *mut device_uuid_node,
}
impl std::default::Default for device_uuid_node {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct device_stats {
    pub total: u32,
    pub total_in: u32,
    pub total_out: u32,
    pub offhook: u32,
    pub offhook_in: u32,
    pub offhook_out: u32,
    pub active: u32,
    pub active_in: u32,
    pub active_out: u32,
    pub held: u32,
    pub held_in: u32,
    pub held_out: u32,
    pub unheld: u32,
    pub unheld_in: u32,
    pub unheld_out: u32,
    pub hup: u32,
    pub hup_in: u32,
    pub hup_out: u32,
    pub ringing: u32,
    pub ringing_in: u32,
    pub ringing_out: u32,
    pub early: u32,
    pub early_in: u32,
    pub early_out: u32,
    pub ring_wait: u32,
}
impl std::default::Default for device_stats {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct device_record {
    pub device_id: *mut c_char,
    pub uuid: *mut c_char,
    pub refs: c_int,
    pub stats: device_stats,
    pub last_stats: device_stats,
    pub state: device_state,
    pub last_state: device_state,
    pub active_start: time_t,
    pub active_stop: time_t,
    pub last_call_time: time_t,
    pub ring_start: time_t,
    pub ring_stop: time_t,
    pub hold_start: time_t,
    pub hold_stop: time_t,
    pub call_start: time_t,
    pub uuid_list: *mut device_uuid_node,
    pub uuid_tail: *mut device_uuid_node,
    pub mutex: *mut mutex,
    pub pool: *mut memory_pool,
    pub user_data: *mut c_void,
}
impl std::default::Default for device_record {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type device_state_function = Option<unsafe extern "C" fn(session: *mut core_session,
                                                             callstate: channel_callstate,
                                                             drec: *mut device_record)>;
#[repr(C)]
#[derive(Copy)]
pub struct dtls_fp_s {
    pub len: u32,
    pub data: [u8; 65usize],
    pub type_: *mut c_char,
    pub str: [c_char; 192usize],
}
impl std::clone::Clone for dtls_fp_s {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for dtls_fp_s {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type dtls_fingerprint = dtls_fp_s;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dtls_type {
    DTLS_TYPE_CLIENT = 1,
    DTLS_TYPE_SERVER = 2,
    DTLS_TYPE_RTP = 4,
    DTLS_TYPE_RTCP = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum dtls_state {
    DS_OFF = 0,
    DS_HANDSHAKE = 1,
    DS_SETUP = 2,
    DS_READY = 3,
    DS_FAIL = 4,
    DS_INVALID = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct core_session_message {
    pub from: *mut c_char,
    pub message_id: core_session_message_types,
    pub numeric_arg: c_int,
    pub string_arg: *const c_char,
    pub string_arg_size: usize,
    pub pointer_arg: *mut c_void,
    pub pointer_arg_size: usize,
    pub numeric_reply: c_int,
    pub string_reply: *mut c_char,
    pub string_reply_size: usize,
    pub pointer_reply: *mut c_void,
    pub pointer_reply_size: usize,
    pub flags: core_session_message_flag,
    pub _file: *const c_char,
    pub _func: *const c_char,
    pub _line: c_int,
    pub string_array_arg: [*const c_char; 10usize],
    pub delivery_time: time_t,
}
impl std::default::Default for core_session_message {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct core_thread_session {
    pub running: c_int,
    pub mutex: *mut mutex,
    pub objs: [*mut c_void; 128usize],
    pub input_callback: input_callback_function,
    pub pool: *mut memory_pool,
}
impl std::clone::Clone for core_thread_session {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for core_thread_session {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum core_runtime { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum hup_type {
    NONE = 0,
    UNANSWERED = 1,
    ANSWERED = 2,
}
pub type core_db_event_callback_func = Option<unsafe extern "C" fn(pArg: *mut c_void,
                                                                   event: *mut event)
                                                                   -> c_int>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cache_db_flag {
    INUSE = 1,
    PRUNE = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum cache_db_handle_type {
    CORE_DB = 0,
    ODBC = 1,
    PGSQL = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cache_db_native_handle {
    pub _bindgen_data_: [u64; 1usize],
}
impl cache_db_native_handle {
    pub unsafe fn core_db_dbh(&mut self) -> *mut *mut core_db {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn odbc_dbh(&mut self) -> *mut *mut odbc_handle {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pgsql_dbh(&mut self) -> *mut *mut pgsql_handle {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
}
impl std::default::Default for cache_db_native_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cache_db_core_db_options {
    pub db_path: *mut c_char,
}
impl std::default::Default for cache_db_core_db_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cache_db_odbc_options {
    pub dsn: *mut c_char,
    pub user: *mut c_char,
    pub pass: *mut c_char,
}
impl std::default::Default for cache_db_odbc_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cache_db_pgsql_options {
    pub dsn: *mut c_char,
}
impl std::default::Default for cache_db_pgsql_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cache_db_connection_options {
    pub _bindgen_data_: [u64; 3usize],
}
impl cache_db_connection_options {
    pub unsafe fn core_db_options(&mut self) -> *mut cache_db_core_db_options {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn odbc_options(&mut self) -> *mut cache_db_odbc_options {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn pgsql_options(&mut self) -> *mut cache_db_pgsql_options {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
}
impl std::default::Default for cache_db_connection_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum cache_db_handle { }
#[repr(C)]
#[derive(Copy)]
pub struct log_node {
    pub data: *mut c_char,
    pub file: [c_char; 80usize],
    pub line: u32,
    pub func: [c_char; 80usize],
    pub level: log_level,
    pub timestamp: time_t,
    pub content: *mut c_char,
    pub userdata: *mut c_char,
    pub channel: text_channel,
    pub slevel: log_level,
}
impl std::clone::Clone for log_node {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for log_node {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type log_function = Option<unsafe extern "C" fn(node: *const log_node, level: log_level)
                                                    -> status>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct audio_resampler {
    pub resampler: *mut c_void,
    pub from_rate: c_int,
    pub to_rate: c_int,
    pub factor: f64,
    pub rfactor: f64,
    pub to: *mut i16,
    pub to_len: u32,
    pub to_size: u32,
    pub channels: c_int,
}
impl std::default::Default for audio_resampler {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum state_handler_name {
    ON_INIT = 0,
    ON_ROUTING = 1,
    ON_EXECUTE = 2,
    ON_HANGUP = 3,
    ON_EXCHANGE_MEDIA = 4,
    ON_SOFT_EXECUTE = 5,
    ON_CONSUME_MEDIA = 6,
    ON_HIBERNATE = 7,
    ON_RESET = 8,
    ON_PARK = 9,
    ON_REPORTING = 10,
    ON_DESTROY = 11,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct state_handler_table {
    pub on_init: state_handler,
    pub on_routing: state_handler,
    pub on_execute: state_handler,
    pub on_hangup: state_handler,
    pub on_exchange_media: state_handler,
    pub on_soft_execute: state_handler,
    pub on_consume_media: state_handler,
    pub on_hibernate: state_handler,
    pub on_reset: state_handler,
    pub on_park: state_handler,
    pub on_reporting: state_handler,
    pub on_destroy: state_handler,
    pub flags: c_int,
    pub padding: [*mut c_void; 10usize],
}
impl std::default::Default for state_handler_table {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
pub struct stream_handle {
    pub read_function: stream_handle_read_function,
    pub write_function: stream_handle_write_function,
    pub raw_write_function: stream_handle_raw_write_function,
    pub data: *mut c_void,
    pub end: *mut c_void,
    pub data_size: usize,
    pub data_len: usize,
    pub alloc_len: usize,
    pub alloc_chunk: usize,
    pub param_event: *mut event,
}
impl std::default::Default for stream_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type io_outgoing_channel = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                           arg2: *mut event,
                                                           arg3: *mut caller_profile,
                                                           arg4: *mut *mut core_session,
                                                           arg5: *mut *mut memory_pool,
                                                           arg6: originate_flag,
                                                           arg7: *mut call_cause)
                                                           -> call_cause>;
pub type io_read_frame = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                     arg2: *mut *mut frame,
                                                     arg3: io_flag,
                                                     arg4: c_int)
                                                     -> status>;
pub type io_write_frame = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                      arg2: *mut frame,
                                                      arg3: io_flag,
                                                      arg4: c_int)
                                                      -> status>;
pub type io_kill_channel = Option<unsafe extern "C" fn(arg1: *mut core_session, arg2: c_int)
                                                       -> status>;
pub type io_send_dtmf = Option<unsafe extern "C" fn(arg1: *mut core_session, arg2: *const dtmf)
                                                    -> status>;
pub type io_receive_message = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                          arg2: *mut core_session_message)
                                                          -> status>;
pub type io_receive_event = Option<unsafe extern "C" fn(arg1: *mut core_session, arg2: *mut event)
                                                        -> status>;
pub type io_state_change = Option<unsafe extern "C" fn(arg1: *mut core_session) -> status>;
pub type io_state_run = Option<unsafe extern "C" fn(arg1: *mut core_session) -> status>;
pub type io_read_video_frame = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                           arg2: *mut *mut frame,
                                                           arg3: io_flag,
                                                           arg4: c_int)
                                                           -> status>;
pub type io_write_video_frame = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                            arg2: *mut frame,
                                                            arg3: io_flag,
                                                            arg4: c_int)
                                                            -> status>;
pub type io_get_jb = Option<unsafe extern "C" fn(arg1: *mut core_session, arg2: media_type)
                                                 -> *mut jb>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum io_routine_name {
    OUTGOING_CHANNEL = 0,
    READ_FRAME = 1,
    WRITE_FRAME = 2,
    KILL_CHANNEL = 3,
    SEND_DTMF = 4,
    RECEIVE_MESSAGE = 5,
    RECEIVE_EVENT = 6,
    STATE_CHANGE = 7,
    READ_VIDEO_FRAME = 8,
    WRITE_VIDEO_FRAME = 9,
    GET_JB = 10,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_routines {
    pub outgoing_channel: io_outgoing_channel,
    pub read_frame: io_read_frame,
    pub write_frame: io_write_frame,
    pub kill_channel: io_kill_channel,
    pub send_dtmf: io_send_dtmf,
    pub receive_message: io_receive_message,
    pub receive_event: io_receive_event,
    pub state_change: io_state_change,
    pub read_video_frame: io_read_video_frame,
    pub write_video_frame: io_write_video_frame,
    pub state_run: io_state_run,
    pub get_jb: io_get_jb,
    pub padding: [*mut c_void; 10usize],
}
impl std::default::Default for io_routines {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct endpoint_interface {
    pub interface_name: *const c_char,
    pub io_routines: *mut io_routines,
    pub state_handler: *mut state_handler_table,
    pub private_info: *mut c_void,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut endpoint_interface,
    pub recover_callback: core_recover_callback,
}
impl std::default::Default for endpoint_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timer {
    pub interval: c_int,
    pub flags: u32,
    pub samples: c_uint,
    pub samplecount: u32,
    pub timer_interface: *mut timer_interface,
    pub memory_pool: *mut memory_pool,
    pub private_info: *mut c_void,
    pub diff: usize,
    pub start: time_t,
    pub tick: u64,
}
impl std::default::Default for timer {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum timer_func_name {
    TIMER_INIT = 0,
    TIMER_NEXT = 1,
    TIMER_STEP = 2,
    TIMER_SYNC = 3,
    TIMER_CHECK = 4,
    TIMER_DESTROY = 5,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct timer_interface {
    pub interface_name: *const c_char,
    pub timer_init: Option<unsafe extern "C" fn(arg1: *mut timer) -> status>,
    pub timer_next: Option<unsafe extern "C" fn(arg1: *mut timer) -> status>,
    pub timer_step: Option<unsafe extern "C" fn(arg1: *mut timer) -> status>,
    pub timer_sync: Option<unsafe extern "C" fn(arg1: *mut timer) -> status>,
    pub timer_check: Option<unsafe extern "C" fn(arg1: *mut timer, arg2: switch_bool) -> status>,
    pub timer_destroy: Option<unsafe extern "C" fn(arg1: *mut timer) -> status>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut timer_interface,
}
impl std::default::Default for timer_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct dialplan_interface {
    pub interface_name: *const c_char,
    pub hunt_function: dialplan_hunt_function,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut dialplan_interface,
}
impl std::default::Default for dialplan_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct file_interface {
    pub interface_name: *const c_char,
    pub file_open: Option<unsafe extern "C" fn(arg1: *mut file_handle, file_path: *const c_char)
                                               -> status>,
    pub file_close: Option<unsafe extern "C" fn(arg1: *mut file_handle) -> status>,
    pub file_truncate: Option<unsafe extern "C" fn(arg1: *mut file_handle, offset: i64) -> status>,
    pub file_read: Option<unsafe extern "C" fn(arg1: *mut file_handle,
                                               data: *mut c_void,
                                               len: *mut usize)
                                               -> status>,
    pub file_write: Option<unsafe extern "C" fn(arg1: *mut file_handle,
                                                data: *mut c_void,
                                                len: *mut usize)
                                                -> status>,
    pub file_read_video: Option<unsafe extern "C" fn(arg1: *mut file_handle,
                                                     frame: *mut frame,
                                                     flags: video_read_flag)
                                                     -> status>,
    pub file_write_video: Option<unsafe extern "C" fn(arg1: *mut file_handle, frame: *mut frame)
                                                      -> status>,
    pub file_seek: Option<unsafe extern "C" fn(arg1: *mut file_handle,
                                               cur_pos: *mut c_uint,
                                               samples: i64,
                                               whence: c_int)
                                               -> status>,
    pub file_set_string: Option<unsafe extern "C" fn(fh: *mut file_handle,
                                                     col: audio_col,
                                                     string: *const c_char)
                                                     -> status>,
    pub file_get_string: Option<unsafe extern "C" fn(fh: *mut file_handle,
                                                     col: audio_col,
                                                     string: *mut *const c_char)
                                                     -> status>,
    pub file_command: Option<unsafe extern "C" fn(fh: *mut file_handle, command: file_command)
                                                  -> status>,
    pub extens: *mut *mut c_char,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut file_interface,
}
impl std::default::Default for file_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub const VIDEO_ENCODE_SPEED_FAST: video_encode_speed = video_encode_speed::DEFAULT;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum video_encode_speed {
    DEFAULT = 0,
    MEDIUM = 1,
    SLOW = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum video_profile {
    BASELINE = 0,
    MAIN = 1,
    HIGH = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct mm {
    pub samplerate: c_int,
    pub channels: c_int,
    pub keyint: c_int,
    pub ab: c_int,
    pub vb: c_int,
    pub vw: c_int,
    pub vh: c_int,
    pub fps: f32,
    pub source_fps: f32,
    pub vbuf: c_int,
    pub vprofile: video_profile,
    pub vencspd: video_encode_speed,
    pub try_hardware_encoder: u8,
}
impl std::default::Default for mm {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct file_handle {
    pub file_interface: *mut file_interface,
    pub flags: u32,
    pub fd: *mut file,
    pub samples: c_uint,
    pub samplerate: u32,
    pub native_rate: u32,
    pub channels: u32,
    pub real_channels: u32,
    pub format: c_uint,
    pub sections: c_uint,
    pub seekable: c_int,
    pub sample_count: usize,
    pub speed: c_int,
    pub memory_pool: *mut memory_pool,
    pub prebuf: u32,
    pub interval: u32,
    pub private_info: *mut c_void,
    pub handler: *mut c_char,
    pub pos: i64,
    pub audio_buffer: *mut buffer,
    pub sp_audio_buffer: *mut buffer,
    pub thresh: u32,
    pub silence_hits: u32,
    pub offset_pos: u32,
    pub samples_in: usize,
    pub samples_out: usize,
    pub vol: i32,
    pub resampler: *mut audio_resampler,
    pub buffer: *mut buffer,
    pub dbuf: *mut u8,
    pub dbuflen: usize,
    pub pre_buffer: *mut buffer,
    pub pre_buffer_data: *mut c_uchar,
    pub pre_buffer_datalen: usize,
    pub file: *const c_char,
    pub func: *const c_char,
    pub line: c_int,
    pub file_path: *mut c_char,
    pub spool_path: *mut c_char,
    pub prefix: *const c_char,
    pub max_samples: c_int,
    pub params: *mut event,
    pub cur_channels: u32,
    pub cur_samplerate: u32,
    pub stream_name: *mut c_char,
    pub modname: *mut c_char,
    pub mm: mm,
    pub flag_mutex: *mut mutex,
    pub duration: i64,
    pub vpos: i64,
}
impl std::default::Default for file_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct asr_interface {
    pub interface_name: *const c_char,
    pub asr_open: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                              codec: *const c_char,
                                              rate: c_int,
                                              dest: *const c_char,
                                              flags: *mut asr_flag)
                                              -> status>,
    pub asr_load_grammar: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                      grammar: *const c_char,
                                                      name: *const c_char)
                                                      -> status>,
    pub asr_unload_grammar: Option<unsafe extern "C" fn(ah: *mut asr_handle, name: *const c_char)
                                                        -> status>,
    pub asr_close: Option<unsafe extern "C" fn(ah: *mut asr_handle, flags: *mut asr_flag) -> status>,
    pub asr_feed: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                              data: *mut c_void,
                                              len: c_uint,
                                              flags: *mut asr_flag)
                                              -> status>,
    pub asr_resume: Option<unsafe extern "C" fn(ah: *mut asr_handle) -> status>,
    pub asr_pause: Option<unsafe extern "C" fn(ah: *mut asr_handle) -> status>,
    pub asr_check_results: Option<unsafe extern "C" fn(ah: *mut asr_handle, flags: *mut asr_flag)
                                                       -> status>,
    pub asr_get_results: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                     xmlstr: *mut *mut c_char,
                                                     flags: *mut asr_flag)
                                                     -> status>,
    pub asr_get_result_headers: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                            headers: *mut *mut event,
                                                            flags: *mut asr_flag)
                                                            -> status>,
    pub asr_start_input_timers: Option<unsafe extern "C" fn(ah: *mut asr_handle) -> status>,
    pub asr_text_param: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                    param: *mut c_char,
                                                    val: *const c_char)>,
    pub asr_numeric_param: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                       param: *mut c_char,
                                                       val: c_int)>,
    pub asr_float_param: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                     param: *mut c_char,
                                                     val: f64)>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut asr_interface,
    pub asr_enable_grammar: Option<unsafe extern "C" fn(ah: *mut asr_handle, name: *const c_char)
                                                        -> status>,
    pub asr_disable_grammar: Option<unsafe extern "C" fn(ah: *mut asr_handle, name: *const c_char)
                                                         -> status>,
    pub asr_disable_all_grammars: Option<unsafe extern "C" fn(ah: *mut asr_handle) -> status>,
    pub asr_feed_dtmf: Option<unsafe extern "C" fn(ah: *mut asr_handle,
                                                   dtmf: *const dtmf,
                                                   flags: *mut asr_flag)
                                                   -> status>,
}
impl std::default::Default for asr_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct asr_handle {
    pub asr_interface: *mut asr_interface,
    pub flags: u32,
    pub name: *mut c_char,
    pub codec: *mut c_char,
    pub rate: u32,
    pub grammar: *mut c_char,
    pub param: *mut c_char,
    pub memory_pool: *mut memory_pool,
    pub buffer: *mut buffer,
    pub dbuf: *mut u8,
    pub dbuflen: usize,
    pub resampler: *mut audio_resampler,
    pub samplerate: u32,
    pub native_rate: u32,
    pub private_info: *mut c_void,
}
impl std::default::Default for asr_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct speech_interface {
    pub interface_name: *const c_char,
    pub speech_open: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                 voice_name: *const c_char,
                                                 rate: c_int,
                                                 channels: c_int,
                                                 flags: *mut speech_flag)
                                                 -> status>,
    pub speech_close: Option<unsafe extern "C" fn(arg1: *mut speech_handle,
                                                  flags: *mut speech_flag)
                                                  -> status>,
    pub speech_feed_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                     text: *mut c_char,
                                                     flags: *mut speech_flag)
                                                     -> status>,
    pub speech_read_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                     data: *mut c_void,
                                                     datalen: *mut usize,
                                                     flags: *mut speech_flag)
                                                     -> status>,
    pub speech_flush_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle)>,
    pub speech_text_param_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                           param: *mut c_char,
                                                           val: *const c_char)>,
    pub speech_numeric_param_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                              param: *mut c_char,
                                                              val: c_int)>,
    pub speech_float_param_tts: Option<unsafe extern "C" fn(sh: *mut speech_handle,
                                                            param: *mut c_char,
                                                            val: f64)>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut speech_interface,
}
impl std::default::Default for speech_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct speech_handle {
    pub speech_interface: *mut speech_interface,
    pub flags: u32,
    pub name: *mut c_char,
    pub rate: u32,
    pub speed: u32,
    pub samples: u32,
    pub channels: u32,
    pub real_channels: u32,
    pub voice: [c_char; 80usize],
    pub engine: *mut c_char,
    pub param: *mut c_char,
    pub memory_pool: *mut memory_pool,
    pub resampler: *mut audio_resampler,
    pub buffer: *mut buffer,
    pub dbuf: *mut u8,
    pub dbuflen: usize,
    pub samplerate: u32,
    pub native_rate: u32,
    pub private_info: *mut c_void,
}
impl std::clone::Clone for speech_handle {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for speech_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct say_interface {
    pub interface_name: *const c_char,
    pub say_function: say_callback,
    pub say_string_function: say_string_callback,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut say_interface,
}
impl std::default::Default for say_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct chat_interface {
    pub interface_name: *const c_char,
    pub chat_send: Option<unsafe extern "C" fn(message_event: *mut event) -> status>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut chat_interface,
}
impl std::default::Default for chat_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct management_interface {
    pub relative_oid: *const c_char,
    pub management_function: Option<unsafe extern "C" fn(relative_oid: *mut c_char,
                                                         action: management_action,
                                                         data: *mut c_char,
                                                         datalen: usize)
                                                         -> status>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut management_interface,
}
impl std::default::Default for management_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct limit_interface {
    pub interface_name: *const c_char,
    pub incr: Option<unsafe extern "C" fn(session: *mut core_session,
                                          realm: *const c_char,
                                          resource: *const c_char,
                                          max: c_int,
                                          interval: c_int)
                                          -> status>,
    pub release: Option<unsafe extern "C" fn(session: *mut core_session,
                                             realm: *const c_char,
                                             resource: *const c_char)
                                             -> status>,
    pub usage: Option<unsafe extern "C" fn(realm: *const c_char,
                                           resource: *const c_char,
                                           rcount: *mut u32)
                                           -> c_int>,
    pub reset: Option<extern "C" fn() -> status>,
    pub status: Option<extern "C" fn() -> *mut c_char>,
    pub interval_reset: Option<unsafe extern "C" fn(realm: *const c_char, resource: *const c_char)
                                                    -> status>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut limit_interface,
}
impl std::default::Default for limit_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct directory_interface {
    pub interface_name: *const c_char,
    pub directory_open: Option<unsafe extern "C" fn(dh: *mut directory_handle,
                                                    source: *mut c_char,
                                                    dsn: *mut c_char,
                                                    passwd: *mut c_char)
                                                    -> status>,
    pub directory_close: Option<unsafe extern "C" fn(dh: *mut directory_handle) -> status>,
    pub directory_query: Option<unsafe extern "C" fn(dh: *mut directory_handle,
                                                     base: *mut c_char,
                                                     query: *mut c_char)
                                                     -> status>,
    pub directory_next: Option<unsafe extern "C" fn(dh: *mut directory_handle) -> status>,
    pub directory_next_pair: Option<unsafe extern "C" fn(dh: *mut directory_handle,
                                                         var: *mut *mut c_char,
                                                         val: *mut *mut c_char)
                                                         -> status>,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut directory_interface,
}
impl std::default::Default for directory_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct directory_handle {
    pub directory_interface: *mut directory_interface,
    pub flags: u32,
    pub memory_pool: *mut memory_pool,
    pub private_info: *mut c_void,
}
impl std::default::Default for directory_handle {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct audio_codec_settings {
    pub unused: c_int,
}
impl std::default::Default for audio_codec_settings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct video_codec_settings {
    pub bandwidth: u32,
    pub width: i32,
    pub height: i32,
    pub try_hardware_encoder: u8,
}
impl std::default::Default for video_codec_settings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct codec_settings {
    pub _bindgen_data_: [u32; 4usize],
}
impl codec_settings {
    pub unsafe fn audio(&mut self) -> *mut audio_codec_settings {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn video(&mut self) -> *mut video_codec_settings {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
}
impl std::default::Default for codec_settings {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct codec_fmtp {
    pub actual_samples_per_second: u32,
    pub bits_per_second: c_int,
    pub microseconds_per_packet: c_int,
    pub stereo: c_int,
    pub private_info: *mut c_void,
}
impl std::default::Default for codec_fmtp {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct picture {
    pub width: u32,
    pub height: u32,
    pub planes: [*mut u8; 4usize],
    pub stride: [u32; 4usize],
}
impl std::default::Default for picture {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct codec {
    pub codec_interface: *mut codec_interface,
    pub implementation: *const codec_implementation,
    pub fmtp_in: *mut c_char,
    pub fmtp_out: *mut c_char,
    pub flags: u32,
    pub memory_pool: *mut memory_pool,
    pub private_info: *mut c_void,
    pub agreed_pt: payload,
    pub mutex: *mut mutex,
    pub next: *mut codec,
    pub session: *mut core_session,
    pub cur_frame: *mut frame,
}
impl std::default::Default for codec {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct codec_implementation {
    pub codec_type: codec_type,
    pub ianacode: payload,
    pub iananame: *mut c_char,
    pub fmtp: *mut c_char,
    pub samples_per_second: u32,
    pub actual_samples_per_second: u32,
    pub bits_per_second: c_int,
    pub microseconds_per_packet: c_int,
    pub samples_per_packet: u32,
    pub decoded_bytes_per_packet: u32,
    pub encoded_bytes_per_packet: u32,
    pub number_of_channels: u8,
    pub codec_frames_per_packet: c_int,
    pub init: core_codec_init_func,
    pub encode: core_codec_encode_func,
    pub decode: core_codec_decode_func,
    pub encode_video: core_codec_video_encode_func,
    pub decode_video: core_codec_video_decode_func,
    pub codec_control: core_codec_control_func,
    pub destroy: core_codec_destroy_func,
    pub codec_id: u32,
    pub impl_id: u32,
    pub modname: *mut c_char,
    pub next: *mut codec_implementation,
}
impl std::default::Default for codec_implementation {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct codec_interface {
    pub interface_name: *const c_char,
    pub implementations: *mut codec_implementation,
    pub parse_fmtp: core_codec_fmtp_parse_func,
    pub codec_id: u32,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub modname: *mut c_char,
    pub parent: *mut loadable_module_interface,
    pub next: *mut codec_interface,
}
impl std::default::Default for codec_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct application_interface {
    pub interface_name: *const c_char,
    pub application_function: application_function,
    pub long_desc: *const c_char,
    pub short_desc: *const c_char,
    pub syntax: *const c_char,
    pub flags: u32,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut application_interface,
}
impl std::default::Default for application_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct chat_application_interface {
    pub interface_name: *const c_char,
    pub chat_application_function: chat_application_function,
    pub long_desc: *const c_char,
    pub short_desc: *const c_char,
    pub syntax: *const c_char,
    pub flags: u32,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut chat_application_interface,
}
impl std::default::Default for chat_application_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct api_interface {
    pub interface_name: *const c_char,
    pub desc: *const c_char,
    pub function: api_function,
    pub syntax: *const c_char,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut api_interface,
}
impl std::default::Default for api_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct json_api_interface {
    pub interface_name: *const c_char,
    pub desc: *const c_char,
    pub function: json_api_function,
    pub syntax: *const c_char,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub reflock: *mut mutex,
    pub parent: *mut loadable_module_interface,
    pub next: *mut json_api_interface,
}
impl std::default::Default for json_api_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct frame {
    pub codec: *mut codec,
    pub source: *const c_char,
    pub packet: *mut c_void,
    pub packetlen: u32,
    pub extra_data: *mut c_void,
    pub data: *mut c_void,
    pub datalen: u32,
    pub buflen: u32,
    pub samples: u32,
    pub rate: u32,
    pub channels: u32,
    pub payload: payload,
    pub timestamp: u32,
    pub seq: u16,
    pub ssrc: u32,
    pub m: switch_bool,
    pub flags: frame_flag,
    pub user_data: *mut c_void,
    pub pmap: *mut payload_map,
    pub img: *mut vpx_image,
}
impl std::default::Default for frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct slin_data {
    pub session: *mut core_session,
    pub write_frame: frame,
    pub codec: codec,
    pub frame_data: [c_char; 8192usize],
}
impl std::clone::Clone for slin_data {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for slin_data {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct loadable_module_interface {
    pub module_name: *const c_char,
    pub endpoint_interface: *mut endpoint_interface,
    pub timer_interface: *mut timer_interface,
    pub dialplan_interface: *mut dialplan_interface,
    pub codec_interface: *mut codec_interface,
    pub application_interface: *mut application_interface,
    pub chat_application_interface: *mut chat_application_interface,
    pub api_interface: *mut api_interface,
    pub json_api_interface: *mut json_api_interface,
    pub file_interface: *mut file_interface,
    pub speech_interface: *mut speech_interface,
    pub directory_interface: *mut directory_interface,
    pub chat_interface: *mut chat_interface,
    pub say_interface: *mut say_interface,
    pub asr_interface: *mut asr_interface,
    pub management_interface: *mut management_interface,
    pub limit_interface: *mut limit_interface,
    pub rwlock: *mut thread_rwlock,
    pub refs: c_int,
    pub pool: *mut memory_pool,
}
impl std::default::Default for loadable_module_interface {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum _LIB_VERSION_TYPE {
    IEEE_ = -1,
    SVID_ = 0,
    XOPEN_ = 1,
    POSIX_ = 2,
    ISOC_ = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ip {
    pub _bindgen_data_: [u32; 4usize],
}
impl ip {
    pub unsafe fn v4(&mut self) -> *mut u32 {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn v6(&mut self) -> *mut in6_addr {
        let raw: *mut u8 = std::mem::transmute(&self._bindgen_data_);
        std::mem::transmute(raw.offset(0))
    }
}
impl std::default::Default for ip {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum uri_flags {
    NUMERIC_HOST = 1,
    NUMERIC_PORT = 2,
    NO_SCOPE = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct http_request {
    pub method: *const c_char,
    pub uri: *const c_char,
    pub qs: *const c_char,
    pub host: *const c_char,
    pub port: port,
    pub from: *const c_char,
    pub user_agent: *const c_char,
    pub referer: *const c_char,
    pub user: *const c_char,
    pub keepalive: switch_bool,
    pub content_type: *const c_char,
    pub content_length: usize,
    pub bytes_header: usize,
    pub bytes_read: usize,
    pub bytes_buffered: usize,
    pub headers: *mut event,
    pub user_data: *mut c_void,
    pub _buffer: *mut c_char,
    pub _destroy_headers: switch_bool,
}
impl std::default::Default for http_request {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct cputime {
    pub userms: i64,
    pub kernelms: i64,
}
impl std::default::Default for cputime {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct profile_node {
    pub var: *mut c_char,
    pub val: *mut c_char,
    pub next: *mut profile_node,
}
impl std::default::Default for profile_node {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct caller_profile {
    pub username: *const c_char,
    pub dialplan: *const c_char,
    pub caller_id_name: *const c_char,
    pub caller_id_number: *const c_char,
    pub orig_caller_id_name: *const c_char,
    pub orig_caller_id_number: *const c_char,
    pub callee_id_name: *const c_char,
    pub callee_id_number: *const c_char,
    pub caller_ton: u8,
    pub caller_numplan: u8,
    pub network_addr: *const c_char,
    pub ani: *const c_char,
    pub ani_ton: u8,
    pub ani_numplan: u8,
    pub aniii: *const c_char,
    pub rdnis: *const c_char,
    pub rdnis_ton: u8,
    pub rdnis_numplan: u8,
    pub destination_number: *mut c_char,
    pub destination_number_ton: u8,
    pub destination_number_numplan: u8,
    pub source: *const c_char,
    pub chan_name: *mut c_char,
    pub uuid: *mut c_char,
    pub context: *const c_char,
    pub profile_index: *const c_char,
    pub flags: caller_profile_flag,
    pub originator_caller_profile: *mut caller_profile,
    pub originatee_caller_profile: *mut caller_profile,
    pub origination_caller_profile: *mut caller_profile,
    pub hunt_caller_profile: *mut caller_profile,
    pub times: *mut channel_timetable,
    pub old_times: *mut channel_timetable,
    pub caller_extension: *mut caller_extension,
    pub pool: *mut memory_pool,
    pub next: *mut caller_profile,
    pub direction: call_direction,
    pub logical_direction: call_direction,
    pub soft: *mut profile_node,
    pub uuid_str: *mut c_char,
    pub clone_of: *mut c_char,
    pub transfer_source: *mut c_char,
}
impl std::default::Default for caller_profile {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct caller_application {
    pub application_name: *mut c_char,
    pub application_data: *mut c_char,
    pub application_function: application_function,
    pub next: *mut caller_application,
}
impl std::default::Default for caller_application {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct caller_extension {
    pub extension_name: *mut c_char,
    pub extension_number: *mut c_char,
    pub current_application: *mut caller_application,
    pub last_application: *mut caller_application,
    pub applications: *mut caller_application,
    pub children: *mut caller_profile,
    pub next: *mut caller_extension,
}
impl std::default::Default for caller_extension {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtcp_report_block_frame {
    pub ssrc: u32,
    pub fraction: u8,
    pub lost: u32,
    pub highest_sequence_number_received: u32,
    pub jitter: u32,
    pub lsr: u32,
    pub dlsr: u32,
    pub loss_avg: u32,
    pub rtt_avg: f64,
}
impl std::default::Default for rtcp_report_block_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rtcp_frame {
    pub report_count: u16,
    pub packet_type: u16,
    pub ssrc: u32,
    pub ntp_msw: u32,
    pub ntp_lsw: u32,
    pub timestamp: u32,
    pub packet_count: u32,
    pub octect_count: u32,
    pub nb_reports: u32,
    pub reports: [rtcp_report_block_frame; 5usize],
}
impl std::default::Default for rtcp_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct channel_timetable {
    pub profile_created: time_t,
    pub created: time_t,
    pub answered: time_t,
    pub progress: time_t,
    pub progress_media: time_t,
    pub hungup: time_t,
    pub transferred: time_t,
    pub resurrected: time_t,
    pub bridged: time_t,
    pub last_hold: time_t,
    pub hold_accum: time_t,
    pub next: *mut channel_timetable,
}
impl std::default::Default for channel_timetable {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct event_header {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub array: *mut *mut c_char,
    pub idx: c_int,
    pub hash: c_ulong,
    pub next: *mut event_header,
}
impl std::default::Default for event_header {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct event {
    pub event_id: event_types,
    pub priority: priority,
    pub owner: *mut c_char,
    pub subclass_name: *mut c_char,
    pub headers: *mut event_header,
    pub last_header: *mut event_header,
    pub body: *mut c_char,
    pub bind_user_data: *mut c_void,
    pub event_user_data: *mut c_void,
    pub key: c_ulong,
    pub next: *mut event,
    pub flags: c_int,
}
impl std::default::Default for event {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct serial_event {
    pub event_id: c_int,
    pub priority: c_int,
    pub flags: c_int,
    pub owner: *mut c_char,
    pub subclass_name: *mut c_char,
    pub body: *mut c_char,
}
impl std::default::Default for serial_event {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct serial_event_header {
    pub name: *mut c_char,
    pub value: *mut c_char,
}
impl std::default::Default for serial_event_header {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum event_flag {
    EF_UNIQ_HEADERS = 1,
    EF_NO_CHAT_EXEC = 2,
    EF_DEFAULT_ALLOW = 4,
}
pub type live_array_command_handler = Option<unsafe extern "C" fn(la: *mut live_array,
                                                                  cmd: *const c_char,
                                                                  sessid: *const c_char,
                                                                  jla: *mut cJSON,
                                                                  user_data: *mut c_void)>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum img_position {
    POS_LEFT_TOP = 0,
    POS_LEFT_MID = 1,
    POS_LEFT_BOT = 2,
    POS_CENTER_TOP = 3,
    POS_CENTER_MID = 4,
    POS_CENTER_BOT = 5,
    POS_RIGHT_TOP = 6,
    POS_RIGHT_MID = 7,
    POS_RIGHT_BOT = 8,
    POS_NONE = 9,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum img_fit {
    FIT_SIZE = 0,
    FIT_SCALE = 1,
    FIT_SIZE_AND_SCALE = 2,
    FIT_NONE = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct yuv_color {
    pub y: u8,
    pub u: u8,
    pub v: u8,
}
impl std::default::Default for yuv_color {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct rgb_color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl std::default::Default for rgb_color {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct image_rect {
    pub x: c_uint,
    pub y: c_uint,
    pub w: c_uint,
    pub h: c_uint,
}
impl std::default::Default for image_rect {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum convert_fmt {
    YUYV = 0,
}
pub enum png_opaque { }
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct png {
    pub pvt: *mut png_opaque,
    pub w: c_int,
    pub h: c_int,
}
impl std::default::Default for png {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum image_rotation_mode {
    SRM_NONE = 0,
    SRM_90 = 90,
    SRM_180 = 180,
    SRM_270 = 270,
}
#[repr(C)]
#[derive(Copy)]
pub struct unicast_conninfo {
    pub session: *mut core_session,
    pub read_codec: codec,
    pub write_frame: frame,
    pub write_frame_data: [u8; 8192usize],
    pub socket: *mut socket,
    pub local_ip: *mut c_char,
    pub local_port: port,
    pub remote_ip: *mut c_char,
    pub remote_port: port,
    pub local_addr: *mut sockaddr,
    pub remote_addr: *mut sockaddr,
    pub flag_mutex: *mut mutex,
    pub flags: i32,
    pub type_: c_int,
    pub transport: c_int,
    pub stream_id: c_int,
    pub thread: *mut thread,
}
impl std::clone::Clone for unicast_conninfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for unicast_conninfo {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub enum ivr_digit_stream { }
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ivr_menu_flags {
    FALLTOMAIN = 1,
    FREEPOOL = 2,
    STACK = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ivr_action {
    DIE = 0,
    EXECMENU = 1,
    EXECAPP = 2,
    PLAYSOUND = 3,
    BACK = 4,
    TOMAIN = 5,
    NOOP = 6,
}
pub enum ivr_menu { }
pub type ivr_menu_action_function = unsafe extern "C" fn(arg1: *mut ivr_menu,
                                                         arg2: *mut c_char,
                                                         arg3: *mut c_char,
                                                         arg4: usize,
                                                         arg5: *mut c_void)
                                                         -> ivr_action;
pub enum ivr_menu_action { }
pub enum ivr_menu_xml_ctx { }
#[repr(C)]
#[derive(Copy)]
pub struct rtp_packet {
    pub header: rtp_hdr,
    pub body: [c_char; 16396usize],
}
impl std::clone::Clone for rtp_packet {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for rtp_packet {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtp_crypto_direction {
    SEND = 0,
    RECV = 1,
    SEND_RTCP = 2,
    RECV_RTCP = 3,
    MAX = 4,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct srtp_crypto_suite {
    pub name: *mut c_char,
    pub type_: rtp_crypto_key_type,
    pub keylen: c_int,
}
impl std::default::Default for srtp_crypto_suite {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct rtp_crypto_key {
    pub index: u32,
    pub type_: rtp_crypto_key_type,
    pub key: [c_uchar; 64usize],
    pub keylen: usize,
    pub next: *mut rtp_crypto_key,
}
impl std::clone::Clone for rtp_crypto_key {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for rtp_crypto_key {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum ice_proto {
    RTP = 0,
    RTCP = 1,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct icand {
    pub foundation: *mut c_char,
    pub component_id: c_int,
    pub transport: *mut c_char,
    pub priority: u32,
    pub con_addr: *mut c_char,
    pub con_port: port,
    pub cand_type: *mut c_char,
    pub raddr: *mut c_char,
    pub rport: port,
    pub generation: *mut c_char,
    pub ready: u8,
}
impl std::default::Default for icand {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct ice {
    pub cands: [[icand; 2usize]; 50usize],
    pub cand_idx: [c_int; 2usize],
    pub chosen: [c_int; 2usize],
    pub is_chosen: [c_int; 2usize],
    pub ufrag: *mut c_char,
    pub pwd: *mut c_char,
    pub options: *mut c_char,
}
impl std::clone::Clone for ice {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for ice {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtcp_pt {
    IJ = 195,
    SR = 200,
    RR = 201,
    SDES = 202,
    BYE = 203,
    APP = 204,
    RTPFB = 205,
    PSFB = 206,
    XR = 207,
    AVB = 208,
    RSI = 209,
    TOKEN = 210,
    IDMS = 211,
    LAST = 255,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtcp_sdes {
    END = 0,
    CNAME = 1,
    NAME = 2,
    EMAIL = 3,
    PHONE = 4,
    LOC = 5,
    TOOL = 6,
    NOTE = 7,
    PRIV = 8,
    H323 = 9,
    APSI = 10,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtcp_rtpfb {
    NACK = 1,
    TMMBR = 3,
    TMMBN = 4,
    SR_REQ = 5,
    RAMS = 6,
    TLLEI = 7,
    ECN_FB = 8,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rtcp_psfb {
    PLI = 1,
    SLI = 2,
    RPSI = 3,
    FIR = 4,
    TSTR = 5,
    TSTN = 6,
    VBCM = 7,
    PSLEI = 8,
    AFB = 15,
}
pub type rtp_invalid_handler = Option<unsafe extern "C" fn(rtp_session: *mut rtp,
                                                           sock: *mut socket,
                                                           data: *mut c_void,
                                                           datalen: usize,
                                                           from_addr: *mut sockaddr)>;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xml_flag {
    XML_ROOT = 1,
    XML_NAMEM = 2,
    XML_TXTM = 4,
    XML_DUP = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml {
    pub name: *mut c_char,
    pub attr: *mut *mut c_char,
    pub txt: *mut c_char,
    pub free_path: *mut c_char,
    pub off: usize,
    pub next: xml_t,
    pub sibling: xml_t,
    pub ordered: xml_t,
    pub child: xml_t,
    pub parent: xml_t,
    pub flags: u32,
    pub is_xml_root: switch_bool,
    pub refs: u32,
}
impl std::default::Default for xml {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum xml_config_type {
    INT = 0,
    ATOMIC = 1,
    STRING = 2,
    SWITCH_BOOL = 3,
    CUSTOM = 4,
    ENUM = 5,
    FLAG = 6,
    FLAGARRAY = 7,
    LAST = 8,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml_config_enum_item {
    pub key: *mut c_char,
    pub value: c_int,
}
impl std::default::Default for xml_config_enum_item {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml_config_string_options {
    pub pool: *mut memory_pool,
    pub length: usize,
    pub validation_regex: *mut c_char,
}
impl std::default::Default for xml_config_string_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml_config_int_options {
    pub enforce_min: switch_bool,
    pub min: c_int,
    pub enforce_max: switch_bool,
    pub max: c_int,
}
impl std::default::Default for xml_config_int_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml_config_atomic_options {
    pub enforce_min: switch_bool,
    pub min: u32,
    pub enforce_max: switch_bool,
    pub max: u32,
}
impl std::default::Default for xml_config_atomic_options {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum config_callback_type {
    LOAD = 0,
    RELOAD = 1,
    SHUTDOWN = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum config_flags {
    RELOADABLE = 1,
    REQUIRED = 2,
}
pub type xml_config_callback = Option<unsafe extern "C" fn(item: *mut xml_config_item,
                                                           newvalue: *const c_char,
                                                           callback_type: config_callback_type,
                                                           changed: switch_bool)
                                                           -> status>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct xml_config_item {
    pub key: *const c_char,
    pub type_: xml_config_type,
    pub flags: c_int,
    pub ptr: *mut c_void,
    pub defaultvalue: *const c_void,
    pub data: *mut c_void,
    pub function: xml_config_callback,
    pub syntax: *const c_char,
    pub helptext: *const c_char,
}
impl std::default::Default for xml_config_item {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type outgoing_channel_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                             arg2: *mut event,
                                                             arg3: *mut caller_profile,
                                                             arg4: *mut core_session,
                                                             arg5: originate_flag)
                                                             -> status>;
pub type receive_message_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                            arg2: *mut core_session_message)
                                                            -> status>;
pub type receive_event_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                          arg2: *mut event)
                                                          -> status>;
pub type read_frame_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                       arg2: *mut *mut frame,
                                                       arg3: io_flag,
                                                       arg4: c_int)
                                                       -> status>;
pub type video_read_frame_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                             arg2: *mut *mut frame,
                                                             arg3: io_flag,
                                                             arg4: c_int)
                                                             -> status>;
pub type write_frame_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                        arg2: *mut frame,
                                                        arg3: io_flag,
                                                        arg4: c_int)
                                                        -> status>;
pub type video_write_frame_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                              arg2: *mut frame,
                                                              arg3: io_flag,
                                                              arg4: c_int)
                                                              -> status>;
pub type kill_channel_hook = Option<unsafe extern "C" fn(arg1: *mut core_session, arg2: c_int)
                                                         -> status>;
pub type send_dtmf_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                      arg2: *const dtmf,
                                                      direction: dtmf_direction)
                                                      -> status>;
pub type recv_dtmf_hook = Option<unsafe extern "C" fn(arg1: *mut core_session,
                                                      arg2: *const dtmf,
                                                      direction: dtmf_direction)
                                                      -> status>;
pub type state_change_hook = Option<unsafe extern "C" fn(arg1: *mut core_session) -> status>;
pub type state_run_hook = Option<unsafe extern "C" fn(arg1: *mut core_session) -> status>;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_outgoing_channel {
    pub outgoing_channel: outgoing_channel_hook,
    pub next: *mut io_event_hook_outgoing_channel,
}
impl std::default::Default for io_event_hook_outgoing_channel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_receive_message {
    pub receive_message: receive_message_hook,
    pub next: *mut io_event_hook_receive_message,
}
impl std::default::Default for io_event_hook_receive_message {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_receive_event {
    pub receive_event: receive_event_hook,
    pub next: *mut io_event_hook_receive_event,
}
impl std::default::Default for io_event_hook_receive_event {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_read_frame {
    pub read_frame: read_frame_hook,
    pub next: *mut io_event_hook_read_frame,
}
impl std::default::Default for io_event_hook_read_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_video_read_frame {
    pub video_read_frame: read_frame_hook,
    pub next: *mut io_event_hook_video_read_frame,
}
impl std::default::Default for io_event_hook_video_read_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_write_frame {
    pub write_frame: write_frame_hook,
    pub next: *mut io_event_hook_write_frame,
}
impl std::default::Default for io_event_hook_write_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_video_write_frame {
    pub video_write_frame: video_write_frame_hook,
    pub next: *mut io_event_hook_video_write_frame,
}
impl std::default::Default for io_event_hook_video_write_frame {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_kill_channel {
    pub kill_channel: kill_channel_hook,
    pub next: *mut io_event_hook_kill_channel,
}
impl std::default::Default for io_event_hook_kill_channel {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_send_dtmf {
    pub send_dtmf: send_dtmf_hook,
    pub next: *mut io_event_hook_send_dtmf,
}
impl std::default::Default for io_event_hook_send_dtmf {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_recv_dtmf {
    pub recv_dtmf: recv_dtmf_hook,
    pub next: *mut io_event_hook_recv_dtmf,
}
impl std::default::Default for io_event_hook_recv_dtmf {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_state_change {
    pub state_change: state_change_hook,
    pub next: *mut io_event_hook_state_change,
}
impl std::default::Default for io_event_hook_state_change {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hook_state_run {
    pub state_run: state_run_hook,
    pub next: *mut io_event_hook_state_run,
}
impl std::default::Default for io_event_hook_state_run {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct io_event_hooks {
    pub outgoing_channel: *mut io_event_hook_outgoing_channel,
    pub receive_message: *mut io_event_hook_receive_message,
    pub receive_event: *mut io_event_hook_receive_event,
    pub read_frame: *mut io_event_hook_read_frame,
    pub video_read_frame: *mut io_event_hook_video_read_frame,
    pub write_frame: *mut io_event_hook_write_frame,
    pub video_write_frame: *mut io_event_hook_video_write_frame,
    pub kill_channel: *mut io_event_hook_kill_channel,
    pub send_dtmf: *mut io_event_hook_send_dtmf,
    pub recv_dtmf: *mut io_event_hook_recv_dtmf,
    pub state_change: *mut io_event_hook_state_change,
    pub state_run: *mut io_event_hook_state_run,
}
impl std::default::Default for io_event_hooks {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct scheduler_task {
    pub created: i64,
    pub runtime: i64,
    pub cmd_id: u32,
    pub repeat: u32,
    pub group: *mut c_char,
    pub cmd_arg: *mut c_void,
    pub task_id: u32,
    pub hash: c_ulong,
}
impl std::default::Default for scheduler_task {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct config {
    pub file: *mut FILE,
    pub path: [c_char; 512usize],
    pub category: [c_char; 256usize],
    pub section: [c_char; 256usize],
    pub buf: [c_char; 1024usize],
    pub lineno: c_int,
    pub catno: c_int,
    pub sectno: c_int,
    pub lockto: c_int,
}
impl std::clone::Clone for config {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for config {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nat_type {
    NONE = 0,
    PMP = 1,
    UPNP = 2,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum nat_ip_proto {
    UDP = 0,
    TCP = 1,
}
pub type odbc_statement_handle = *mut c_void;
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum odbc_state {
    INIT = 0,
    DOWN = 1,
    CONNECTED = 2,
    ERROR = 3,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum odbc_status {
    SUCCESS = 0,
    FAIL = -1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum pgsql_state {
    INIT = 0,
    DOWN = 1,
    CONNECTED = 2,
    ERROR = 3,
}
#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum pgsql_status {
    SUCCESS = 0,
    FAIL = -1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_media_dtmf {
    AUTO = 0,
    RFC2833 = 1,
    INFO = 2,
    NONE = 3,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_media_NDLB {
    ALLOW_BAD_IANANAME = 1,
    ALLOW_NONDUP_SDP = 2,
    ALLOW_CRYPTO_IN_AVP = 4,
    DISABLE_SRTP_AUTH = 8,
    SENDRECV_IN_SESSION = 16,
    NEVER_PATCH_REINVITE = 32,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_media_flag {
    RUNNING = 0,
    DISABLE_TRANSCODING = 1,
    AUTOFIX_TIMING = 2,
    CODEC_GREEDY = 3,
    CODEC_SCROOGE = 4,
    DISABLE_HOLD = 5,
    RENEG_ON_HOLD = 6,
    RENEG_ON_REINVITE = 7,
    LIBERAL_DTMF = 8,
    SUPPRESS_CNG = 9,
    DISABLE_RTP_AUTOADJ = 10,
    PASS_RFC2833 = 11,
    AUTOFLUSH = 12,
    REWRITE_TIMESTAMPS = 13,
    RTP_AUTOFLUSH_DURING_BRIDGE = 14,
    MULTI_ANSWER_AUDIO = 15,
    MULTI_ANSWER_VIDEO = 16,
    MAX = 17,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum STUNFLAGS {
    SET = 1,
    PING = 2,
    FUNNY = 4,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum core_media_vflag {
    IN = 1,
    OUT = 2,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct core_media_params {
    pub rtp_timeout_sec: u32,
    pub rtp_hold_timeout_sec: u32,
    pub dtmf_delay: u32,
    pub codec_flags: u32,
    pub ndlb: core_media_NDLB,
    pub auto_rtp_bugs: rtp_bug_flag,
    pub inbound_codec_string: *mut c_char,
    pub outbound_codec_string: *mut c_char,
    pub timer_name: *mut c_char,
    pub remote_sdp_str: *mut c_char,
    pub early_sdp: *mut c_char,
    pub local_sdp_str: *mut c_char,
    pub last_sdp_str: *mut c_char,
    pub last_sdp_response: *mut c_char,
    pub prev_sdp_str: *mut c_char,
    pub prev_sdp_response: *mut c_char,
    pub stun_ip: *mut c_char,
    pub stun_port: port,
    pub stun_flags: u32,
    pub jb_msec: *mut c_char,
    pub vflags: core_media_vflag,
    pub manual_rtp_bugs: rtp_bug_flag,
    pub manual_video_rtp_bugs: rtp_bug_flag,
    pub rtcp_audio_interval_msec: *mut c_char,
    pub rtcp_video_interval_msec: *mut c_char,
    pub extrtpip: *mut c_char,
    pub rtpip: *mut c_char,
    pub rtpip4: *mut c_char,
    pub rtpip6: *mut c_char,
    pub remote_ip: *mut c_char,
    pub remote_port: c_int,
    pub extsipip: *mut c_char,
    pub local_network: *mut c_char,
    pub sipip: *mut c_char,
    pub sdp_username: *mut c_char,
    pub te: payload,
    pub recv_te: payload,
    pub te_rate: c_ulong,
    pub cng_rate: c_ulong,
    pub adv_sdp_audio_ip: *mut c_char,
    pub num_codecs: c_int,
    pub hold_laps: c_int,
    pub dtmf_type: core_media_dtmf,
    pub cng_pt: payload,
    pub external_video_source: switch_bool,
    pub video_key_freq: u32,
    pub video_key_first: u32,
    pub video_write_thread: *mut thread,
}
impl std::default::Default for core_media_params {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum jb_flag {
    QUEUE_ONLY = 1,
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum jb_type {
    VIDEO = 0,
    AUDIO = 1,
}
pub type teletone_process = f64;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_tone_map {
    pub freqs: [teletone_process; 18usize],
}
impl std::default::Default for teletone_tone_map {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_dds_state {
    pub phase_rate: [u32; 4usize],
    pub scale_factor: u32,
    pub phase_accumulator: u32,
    pub tx_level: teletone_process,
}
impl std::default::Default for teletone_dds_state {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
pub type teletone_audio = i16;
pub type tone_handler = Option<unsafe extern "C" fn(ts: *mut teletone_generation_session,
                                                    map: *mut teletone_tone_map)
                                                    -> c_int>;
#[repr(C)]
#[derive(Copy)]
pub struct teletone_generation_session {
    pub TONES: [teletone_tone_map; 127usize],
    pub channels: c_int,
    pub rate: c_int,
    pub duration: c_int,
    pub wait: c_int,
    pub tmp_duration: c_int,
    pub tmp_wait: c_int,
    pub loops: c_int,
    pub LOOPS: c_int,
    pub decay_factor: f32,
    pub decay_direction: c_int,
    pub decay_step: c_int,
    pub volume: f32,
    pub debug: c_int,
    pub debug_stream: *mut FILE,
    pub user_data: *mut c_void,
    pub buffer: *mut teletone_audio,
    pub datalen: c_int,
    pub samples: c_int,
    pub dynamic: c_int,
    pub handler: tone_handler,
}
impl std::clone::Clone for teletone_generation_session {
    fn clone(&self) -> Self {
        *self
    }
}
impl std::default::Default for teletone_generation_session {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum teletone_hit_type {
    NONE = 0,
    BEGIN = 1,
    MIDDLE = 2,
    END = 3,
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_goertzel_state {
    pub v2: f32,
    pub v3: f32,
    pub fac: f64,
}
impl std::default::Default for teletone_goertzel_state {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_dtmf_detect_state {
    pub hit1: c_int,
    pub hit2: c_int,
    pub hit3: c_int,
    pub hit4: c_int,
    pub dur: c_int,
    pub zc: c_int,
    pub row_out: [teletone_goertzel_state; 4usize],
    pub col_out: [teletone_goertzel_state; 4usize],
    pub row_out2nd: [teletone_goertzel_state; 4usize],
    pub col_out2nd: [teletone_goertzel_state; 4usize],
    pub energy: f32,
    pub lenergy: f32,
    pub current_sample: c_int,
    pub digit: c_char,
    pub current_digits: c_int,
    pub detected_digits: c_int,
    pub lost_digits: c_int,
    pub digit_hits: [c_int; 16usize],
}
impl std::default::Default for teletone_dtmf_detect_state {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_detection_descriptor {
    pub fac: f32,
}
impl std::default::Default for teletone_detection_descriptor {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct teletone_multi_tone {
    pub sample_rate: c_int,
    pub tdd: [teletone_detection_descriptor; 18usize],
    pub gs: [teletone_goertzel_state; 18usize],
    pub gs2: [teletone_goertzel_state; 18usize],
    pub tone_count: c_int,
    pub energy: f32,
    pub current_sample: c_int,
    pub min_samples: c_int,
    pub total_samples: c_int,
    pub positives: c_int,
    pub negatives: c_int,
    pub hits: c_int,
    pub positive_factor: c_int,
    pub negative_factor: c_int,
    pub hit_factor: c_int,
}
impl std::default::Default for teletone_multi_tone {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
extern "C" {
    pub static mut SWITCH_GLOBAL_dirs: directories;
    pub static mut SWITCH_GLOBAL_filenames: filenames;
    pub static mut _LIB_VERSION: _LIB_VERSION_TYPE;
    pub static mut switch_config_string_strdup: xml_config_string_options;
    pub static mut TELETONE_SINES: [i16; 128usize];
}
extern "C" {
    pub fn cJSON_InitHooks(hooks: *mut cJSON_Hooks);
    pub fn cJSON_Parse(value: *const c_char) -> *mut cJSON;
    pub fn cJSON_Print(item: *mut cJSON) -> *mut c_char;
    pub fn cJSON_PrintUnformatted(item: *mut cJSON) -> *mut c_char;
    pub fn cJSON_Delete(c: *mut cJSON);
    pub fn cJSON_GetArraySize(array: *mut cJSON) -> c_int;
    pub fn cJSON_GetArrayItem(array: *mut cJSON, item: c_int) -> *mut cJSON;
    pub fn cJSON_GetObjectItem(object: *const cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_GetObjectCstr(object: *const cJSON, string: *const c_char) -> *const c_char;
    pub fn cJSON_GetErrorPtr() -> *const c_char;
    pub fn cJSON_CreateNull() -> *mut cJSON;
    pub fn cJSON_CreateTrue() -> *mut cJSON;
    pub fn cJSON_CreateFalse() -> *mut cJSON;
    pub fn cJSON_CreateBool(b: c_int) -> *mut cJSON;
    pub fn cJSON_CreateNumber(num: f64) -> *mut cJSON;
    pub fn cJSON_CreateString(string: *const c_char) -> *mut cJSON;
    pub fn cJSON_CreateArray() -> *mut cJSON;
    pub fn cJSON_CreateObject() -> *mut cJSON;
    pub fn cJSON_CreateIntArray(numbers: *mut c_int, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateFloatArray(numbers: *mut f32, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateDoubleArray(numbers: *mut f64, count: c_int) -> *mut cJSON;
    pub fn cJSON_CreateStringArray(strings: *mut *const c_char, count: c_int) -> *mut cJSON;
    pub fn cJSON_AddItemToArray(array: *mut cJSON, item: *mut cJSON);
    pub fn cJSON_AddItemToObject(object: *mut cJSON, string: *const c_char, item: *mut cJSON);
    pub fn cJSON_AddItemReferenceToArray(array: *mut cJSON, item: *mut cJSON);
    pub fn cJSON_AddItemReferenceToObject(object: *mut cJSON,
                                          string: *const c_char,
                                          item: *mut cJSON);
    pub fn cJSON_DetachItemFromArray(array: *mut cJSON, which: c_int) -> *mut cJSON;
    pub fn cJSON_DeleteItemFromArray(array: *mut cJSON, which: c_int);
    pub fn cJSON_DetachItemFromObject(object: *mut cJSON, string: *const c_char) -> *mut cJSON;
    pub fn cJSON_DeleteItemFromObject(object: *mut cJSON, string: *const c_char);
    pub fn cJSON_ReplaceItemInArray(array: *mut cJSON, which: c_int, newitem: *mut cJSON);
    pub fn cJSON_ReplaceItemInObject(object: *mut cJSON,
                                     string: *const c_char,
                                     newitem: *mut cJSON);
    pub fn cJSON_Duplicate(item: *mut cJSON, recurse: c_int) -> *mut cJSON;
    pub fn cJSON_CreateStringPrintf(fmt: *const c_char, ...) -> *mut cJSON;

    pub fn vpx_img_alloc(img: *mut vpx_image,
                         fmt: vpx_img_fmt,
                         d_w: c_uint,
                         d_h: c_uint,
                         align: c_uint)
                         -> *mut vpx_image;
    pub fn vpx_img_wrap(img: *mut vpx_image,
                        fmt: vpx_img_fmt,
                        d_w: c_uint,
                        d_h: c_uint,
                        align: c_uint,
                        img_data: *mut c_uchar)
                        -> *mut vpx_image;
    pub fn vpx_img_set_rect(img: *mut vpx_image,
                            x: c_uint,
                            y: c_uint,
                            w: c_uint,
                            h: c_uint)
                            -> c_int;
    pub fn vpx_img_flip(img: *mut vpx_image);
    pub fn vpx_img_free(img: *mut vpx_image);

    #[link_name="switch_status_is_timeup"]
    pub fn status_is_timeup(status: c_int) -> c_int;
    #[link_name="switch_thread_self"]
    pub fn thread_self() -> thread_id;
    #[link_name="switch_thread_equal"]
    pub fn thread_equal(tid1: thread_id, tid2: thread_id) -> c_int;
    #[link_name="switch_pool_clear"]
    pub fn pool_clear(pool: *mut memory_pool);
    #[link_name="switch_copy_string"]
    pub fn copy_string(dst: *mut c_char, src: *const c_char, dst_size: usize) -> *mut c_char;
    #[link_name="switch__default"]
    pub fn hashfunc_default(key: *const c_char, klen: *mut isize) -> c_uint;
    #[link_name="switch_ci_hashfunc_default"]
    pub fn ci_hashfunc_default(char_key: *const c_char, klen: *mut isize) -> c_uint;
    #[link_name="switch_time_make"]
    pub fn time_make(sec: time_t, usec: i32) -> time_t;
    #[link_name="switch_time_now"]
    pub fn time_now() -> time_t;
    #[link_name="switch_time_exp_gmt_get"]
    pub fn time_exp_gmt_get(result: *mut time_t, input: *mut time_exp) -> status;
    #[link_name="switch_strftime"]
    pub fn strftime(s: *mut c_char,
                    retsize: *mut usize,
                    max: usize,
                    format: *const c_char,
                    tm: *mut time_exp)
                    -> status;
    #[link_name="switch_strftime_nocheck"]
    pub fn strftime_nocheck(s: *mut c_char,
                            retsize: *mut usize,
                            max: usize,
                            format: *const c_char,
                            tm: *mut time_exp)
                            -> status;
    #[link_name="switch_rfc822_date"]
    pub fn rfc822_date(date_str: *mut c_char, t: time_t) -> status;
    #[link_name="switch_time_exp_gmt"]
    pub fn time_exp_gmt(result: *mut time_exp, input: time_t) -> status;
    #[link_name="switch_time_exp_get"]
    pub fn time_exp_get(result: *mut time_t, input: *mut time_exp) -> status;
    #[link_name="switch_time_exp_lt"]
    pub fn time_exp_lt(result: *mut time_exp, input: time_t) -> status;
    #[link_name="switch_time_exp_tz"]
    pub fn time_exp_tz(result: *mut time_exp, input: time_t, offs: i32) -> status;
    #[link_name="switch_sleep"]
    pub fn sleep(t: interval_time_t);
    #[link_name="switch_micro_sleep"]
    pub fn micro_sleep(t: interval_time_t);
    #[link_name="switch_mutex_init"]
    pub fn mutex_init(lock: *mut *mut mutex, flags: c_uint, pool: *mut memory_pool) -> status;
    #[link_name="switch_mutex_destroy"]
    pub fn mutex_destroy(lock: *mut mutex) -> status;
    #[link_name="switch_mutex_lock"]
    pub fn mutex_lock(lock: *mut mutex) -> status;
    #[link_name="switch_mutex_unlock"]
    pub fn mutex_unlock(lock: *mut mutex) -> status;
    #[link_name="switch_mutex_trylock"]
    pub fn mutex_trylock(lock: *mut mutex) -> status;
    #[link_name="switch_atomic_init"]
    pub fn atomic_init(pool: *mut memory_pool) -> status;
    #[link_name="switch_thread_rwlock_create"]
    pub fn thread_rwlock_create(rwlock: *mut *mut thread_rwlock, pool: *mut memory_pool) -> status;
    #[link_name="switch_thread_rwlock_destroy"]
    pub fn thread_rwlock_destroy(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_rwlock_pool_get"]
    pub fn thread_rwlock_pool_get(rwlock: *mut thread_rwlock) -> *mut memory_pool;
    #[link_name="switch_thread_rwlock_rdlock"]
    pub fn thread_rwlock_rdlock(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_rwlock_tryrdlock"]
    pub fn thread_rwlock_tryrdlock(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_rwlock_wrlock"]
    pub fn thread_rwlock_wrlock(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_rwlock_trywrlock"]
    pub fn thread_rwlock_trywrlock(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_rwlock_trywrlock_timeout"]
    pub fn thread_rwlock_trywrlock_timeout(rwlock: *mut thread_rwlock, timeout: c_int) -> status;
    #[link_name="switch_thread_rwlock_unlock"]
    pub fn thread_rwlock_unlock(rwlock: *mut thread_rwlock) -> status;
    #[link_name="switch_thread_cond_create"]
    pub fn thread_cond_create(cond: *mut *mut thread_cond, pool: *mut memory_pool) -> status;
    #[link_name="switch_thread_cond_wait"]
    pub fn thread_cond_wait(cond: *mut thread_cond, mutex: *mut mutex) -> status;
    #[link_name="switch_thread_cond_timedwait"]
    pub fn thread_cond_timedwait(cond: *mut thread_cond,
                                 mutex: *mut mutex,
                                 timeout: interval_time_t)
                                 -> status;
    #[link_name="switch_thread_cond_signal"]
    pub fn thread_cond_signal(cond: *mut thread_cond) -> status;
    #[link_name="switch_thread_cond_broadcast"]
    pub fn thread_cond_broadcast(cond: *mut thread_cond) -> status;
    #[link_name="switch_thread_cond_destroy"]
    pub fn thread_cond_destroy(cond: *mut thread_cond) -> status;
    #[link_name="switch_uuid_format"]
    pub fn uuid_format(buffer: *mut c_char, uuid: *const uuid);
    #[link_name="switch_uuid_get"]
    pub fn uuid_get(uuid: *mut uuid);
    #[link_name="switch_uuid_parse"]
    pub fn uuid_parse(uuid: *mut uuid, uuid_str: *const c_char) -> status;
    #[link_name="switch_md5"]
    pub fn md5(digest: *mut c_uchar, input: *const c_void, inputLen: usize) -> status;
    #[link_name="switch_md5_string"]
    pub fn md5_string(digest_str: *mut c_char, input: *const c_void, inputLen: usize) -> status;
    #[link_name="switch_queue_create"]
    pub fn queue_create(queue: *mut *mut queue,
                        queue_capacity: c_uint,
                        pool: *mut memory_pool)
                        -> status;
    #[link_name="switch_queue_pop"]
    pub fn queue_pop(queue: *mut queue, data: *mut *mut c_void) -> status;
    #[link_name="switch_queue_pop_timeout"]
    pub fn queue_pop_timeout(queue: *mut queue,
                             data: *mut *mut c_void,
                             timeout: interval_time_t)
                             -> status;
    #[link_name="switch_queue_push"]
    pub fn queue_push(queue: *mut queue, data: *mut c_void) -> status;
    #[link_name="switch_queue_size"]
    pub fn queue_size(queue: *mut queue) -> c_uint;
    #[link_name="switch_queue_trypop"]
    pub fn queue_trypop(queue: *mut queue, data: *mut *mut c_void) -> status;
    #[link_name="switch_queue_interrupt_all"]
    pub fn queue_interrupt_all(queue: *mut queue) -> status;
    #[link_name="switch_queue_term"]
    pub fn queue_term(queue: *mut queue) -> status;
    #[link_name="switch_queue_trypush"]
    pub fn queue_trypush(queue: *mut queue, data: *mut c_void) -> status;
    #[link_name="switch_file_open"]
    pub fn file_open(newf: *mut *mut file,
                     fname: *const c_char,
                     flag: i32,
                     perm: fileperms,
                     pool: *mut memory_pool)
                     -> status;
    #[link_name="switch_file_seek"]
    pub fn file_seek(thefile: *mut file, where_: seek_where, offset: *mut i64) -> status;
    #[link_name="switch_file_copy"]
    pub fn file_copy(from_path: *const c_char,
                     to_path: *const c_char,
                     perms: fileperms,
                     pool: *mut memory_pool)
                     -> status;
    #[link_name="switch_file_close"]
    pub fn file_close(thefile: *mut file) -> status;
    #[link_name="switch_file_trunc"]
    pub fn file_trunc(thefile: *mut file, offset: i64) -> status;
    #[link_name="switch_file_lock"]
    pub fn file_lock(thefile: *mut file, type_: c_int) -> status;
    #[link_name="switch_file_remove"]
    pub fn file_remove(path: *const c_char, pool: *mut memory_pool) -> status;
    #[link_name="switch_file_rename"]
    pub fn file_rename(from_path: *const c_char,
                       to_path: *const c_char,
                       pool: *mut memory_pool)
                       -> status;
    #[link_name="switch_file_read"]
    pub fn file_read(thefile: *mut file, buf: *mut c_void, nbytes: *mut usize) -> status;
    #[link_name="switch_file_write"]
    pub fn file_write(thefile: *mut file, buf: *const c_void, nbytes: *mut usize) -> status;
    #[link_name="switch_file_printf"]
    pub fn file_printf(thefile: *mut file, format: *const c_char, ...) -> c_int;
    #[link_name="switch_file_mktemp"]
    pub fn file_mktemp(thefile: *mut *mut file,
                       templ: *mut c_char,
                       flags: i32,
                       pool: *mut memory_pool)
                       -> status;
    #[link_name="switch_file_get_size"]
    pub fn file_get_size(thefile: *mut file) -> usize;
    #[link_name="switch_file_exists"]
    pub fn file_exists(filename: *const c_char, pool: *mut memory_pool) -> status;
    #[link_name="switch_directory_exists"]
    pub fn directory_exists(dirname: *const c_char, pool: *mut memory_pool) -> status;
    #[link_name="switch_dir_make"]
    pub fn dir_make(path: *const c_char, perm: fileperms, pool: *mut memory_pool) -> status;
    #[link_name="switch_dir_make_recursive"]
    pub fn dir_make_recursive(path: *const c_char,
                              perm: fileperms,
                              pool: *mut memory_pool)
                              -> status;
    #[link_name="switch_dir_open"]
    pub fn dir_open(new_dir: *mut *mut dir,
                    dirname: *const c_char,
                    pool: *mut memory_pool)
                    -> status;
    #[link_name="switch_dir_close"]
    pub fn dir_close(thedir: *mut dir) -> status;
    #[link_name="switch_dir_next_file"]
    pub fn dir_next_file(thedir: *mut dir, buf: *mut c_char, len: usize) -> *const c_char;
    #[link_name="switch_dir_count"]
    pub fn dir_count(thedir: *mut dir) -> u32;
    #[link_name="switch_threadattr_stacksize_set"]
    pub fn threadattr_stacksize_set(attr: *mut threadattr, stacksize: usize) -> status;
    #[link_name="switch_threadattr_priority_set"]
    pub fn threadattr_priority_set(attr: *mut threadattr, priority: thread_priority) -> status;
    #[link_name="switch_threadattr_create"]
    pub fn threadattr_create(new_attr: *mut *mut threadattr, pool: *mut memory_pool) -> status;
    #[link_name="switch_threadattr_detach_set"]
    pub fn threadattr_detach_set(attr: *mut threadattr, on: i32) -> status;
    #[link_name="switch_thread_create"]
    pub fn thread_create(new_thread: *mut *mut thread,
                         attr: *mut threadattr,
                         func: thread_start,
                         data: *mut c_void,
                         cont: *mut memory_pool)
                         -> status;
    #[link_name="switch_socket_create"]
    pub fn socket_create(new_sock: *mut *mut socket,
                         family: c_int,
                         type_: c_int,
                         protocol: c_int,
                         pool: *mut memory_pool)
                         -> status;
    #[link_name="switch_socket_shutdown"]
    pub fn socket_shutdown(sock: *mut socket, how: shutdown_how_e) -> status;
    #[link_name="switch_socket_close"]
    pub fn socket_close(sock: *mut socket) -> status;
    #[link_name="switch_socket_bind"]
    pub fn socket_bind(sock: *mut socket, sa: *mut sockaddr) -> status;
    #[link_name="switch_socket_listen"]
    pub fn socket_listen(sock: *mut socket, backlog: i32) -> status;
    #[link_name="switch_socket_accept"]
    pub fn socket_accept(new_sock: *mut *mut socket,
                         sock: *mut socket,
                         pool: *mut memory_pool)
                         -> status;
    #[link_name="switch_socket_connect"]
    pub fn socket_connect(sock: *mut socket, sa: *mut sockaddr) -> status;
    #[link_name="switch_socket_fd_get"]
    pub fn socket_fd_get(sock: *mut socket) -> c_int;
    #[link_name="switch_sockaddr_get_port"]
    pub fn sockaddr_get_port(sa: *mut sockaddr) -> u16;
    #[link_name="switch_getnameinfo"]
    pub fn getnameinfo(hostname: *mut *mut c_char, sa: *mut sockaddr, flags: i32) -> status;
    #[link_name="switch_sockaddr_get_family"]
    pub fn sockaddr_get_family(sa: *mut sockaddr) -> i32;
    #[link_name="switch_sockaddr_ip_get"]
    pub fn sockaddr_ip_get(addr: *mut *mut c_char, sa: *mut sockaddr) -> status;
    #[link_name="switch_sockaddr_equal"]
    pub fn sockaddr_equal(sa1: *const sockaddr, sa2: *const sockaddr) -> c_int;
    #[link_name="switch_sockaddr_info_get"]
    pub fn sockaddr_info_get(sa: *mut *mut sockaddr,
                             hostname: *const c_char,
                             family: i32,
                             port: port,
                             flags: i32,
                             pool: *mut memory_pool)
                             -> status;
    #[link_name="switch_sockaddr_create"]
    pub fn sockaddr_create(sa: *mut *mut sockaddr, pool: *mut memory_pool) -> status;
    #[link_name="switch_socket_send"]
    pub fn socket_send(sock: *mut socket, buf: *const c_char, len: *mut usize) -> status;
    #[link_name="switch_socket_sendto"]
    pub fn socket_sendto(sock: *mut socket,
                         where_: *mut sockaddr,
                         flags: i32,
                         buf: *const c_char,
                         len: *mut usize)
                         -> status;
    #[link_name="switch_socket_send_nonblock"]
    pub fn socket_send_nonblock(sock: *mut socket, buf: *const c_char, len: *mut usize) -> status;
    #[link_name="switch_socket_recvfrom"]
    pub fn socket_recvfrom(from: *mut sockaddr,
                           sock: *mut socket,
                           flags: i32,
                           buf: *mut c_char,
                           len: *mut usize)
                           -> status;
    #[link_name="switch_socket_atmark"]
    pub fn socket_atmark(sock: *mut socket, atmark: *mut c_int) -> status;
    #[link_name="switch_socket_recv"]
    pub fn socket_recv(sock: *mut socket, buf: *mut c_char, len: *mut usize) -> status;
    #[link_name="switch_socket_opt_set"]
    pub fn socket_opt_set(sock: *mut socket, opt: i32, on: i32) -> status;
    #[link_name="switch_socket_timeout_set"]
    pub fn socket_timeout_set(sock: *mut socket, t: interval_time_t) -> status;
    #[link_name="switch_mcast_join"]
    pub fn mcast_join(sock: *mut socket,
                      join: *mut sockaddr,
                      iface: *mut sockaddr,
                      source: *mut sockaddr)
                      -> status;
    #[link_name="switch_mcast_hops"]
    pub fn mcast_hops(sock: *mut socket, ttl: u8) -> status;
    #[link_name="switch_mcast_loopback"]
    pub fn mcast_loopback(sock: *mut socket, opt: u8) -> status;
    #[link_name="switch_mcast_interface"]
    pub fn mcast_interface(sock: *mut socket, iface: *mut sockaddr) -> status;
    #[link_name="switch_pollset_create"]
    pub fn pollset_create(pollset: *mut *mut pollset,
                          size: u32,
                          pool: *mut memory_pool,
                          flags: u32)
                          -> status;
    #[link_name="switch_pollset_add"]
    pub fn pollset_add(pollset: *mut pollset, descriptor: *const pollfd) -> status;
    #[link_name="switch_pollset_remove"]
    pub fn pollset_remove(pollset: *mut pollset, descriptor: *const pollfd) -> status;
    #[link_name="switch_poll"]
    pub fn poll(aprset: *mut pollfd,
                numsock: i32,
                nsds: *mut i32,
                timeout: interval_time_t)
                -> status;
    #[link_name="switch_pollset_poll"]
    pub fn pollset_poll(pollset: *mut pollset,
                        timeout: interval_time_t,
                        num: *mut i32,
                        descriptors: *mut *const pollfd)
                        -> status;
    #[link_name="switch_socket_create_pollset"]
    pub fn socket_create_pollset(poll: *mut *mut pollfd,
                                 sock: *mut socket,
                                 flags: i16,
                                 pool: *mut memory_pool)
                                 -> status;
    #[link_name="switch_interval_time_from_timeval"]
    pub fn interval_time_from_timeval(tvp: *mut timeval) -> interval_time_t;
    #[link_name="switch_socket_create_pollfd"]
    pub fn socket_create_pollfd(pollfd: *mut *mut pollfd,
                                sock: *mut socket,
                                flags: i16,
                                client_data: *mut c_void,
                                pool: *mut memory_pool)
                                -> status;
    #[link_name="switch_match_glob"]
    pub fn match_glob(pattern: *const c_char,
                      result: *mut *mut array_header,
                      pool: *mut memory_pool)
                      -> status;
    #[link_name="switch_os_sock_get"]
    pub fn os_sock_get(thesock: *mut os_socket, sock: *mut socket) -> status;
    #[link_name="switch_os_sock_put"]
    pub fn os_sock_put(sock: *mut *mut socket,
                       thesock: *mut os_socket,
                       pool: *mut memory_pool)
                       -> status;
    #[link_name="switch_socket_addr_get"]
    pub fn socket_addr_get(sa: *mut *mut sockaddr,
                           remote: switch_bool,
                           sock: *mut socket)
                           -> status;
    #[link_name="switch_file_pipe_create"]
    pub fn file_pipe_create(in_: *mut *mut file,
                            out: *mut *mut file,
                            pool: *mut memory_pool)
                            -> status;
    #[link_name="switch_file_pipe_timeout_get"]
    pub fn file_pipe_timeout_get(thepipe: *mut file, timeout: *mut interval_time_t) -> status;
    #[link_name="switch_file_pipe_timeout_set"]
    pub fn file_pipe_timeout_set(thepipe: *mut file, timeout: interval_time_t) -> status;
    #[link_name="switch_thread_exit"]
    pub fn thread_exit(thd: *mut thread, retval: status) -> status;
    #[link_name="switch_thread_join"]
    pub fn thread_join(retval: *mut status, thd: *mut thread) -> status;
    #[link_name="switch_strerror"]
    pub fn strerror(statcode: status, buf: *mut c_char, bufsize: usize) -> *mut c_char;
    #[link_name="switch_mprintf"]
    pub fn mprintf(zFormat: *const c_char, ...) -> *mut c_char;
    #[link_name="switch_snprintfv"]
    pub fn snprintfv(zBuf: *mut c_char, n: c_int, zFormat: *const c_char, ...) -> *mut c_char;
    #[link_name="switch_core_db_close"]
    pub fn core_db_close(db: *mut core_db) -> c_int;
    #[link_name="switch_core_db_open"]
    pub fn core_db_open(filename: *const c_char, ppDb: *mut *mut core_db) -> c_int;
    #[link_name="switch_core_db_column_text"]
    pub fn core_db_column_text(stmt: *mut core_db_stmt, iCol: c_int) -> *const c_uchar;
    #[link_name="switch_core_db_column_name"]
    pub fn core_db_column_name(stmt: *mut core_db_stmt, N: c_int) -> *const c_char;
    #[link_name="switch_core_db_column_count"]
    pub fn core_db_column_count(pStmt: *mut core_db_stmt) -> c_int;
    #[link_name="switch_core_db_errmsg"]
    pub fn core_db_errmsg(db: *mut core_db) -> *const c_char;
    #[link_name="switch_core_db_exec"]
    pub fn core_db_exec(db: *mut core_db,
                        sql: *const c_char,
                        callback: core_db_callback_func,
                        data: *mut c_void,
                        errmsg: *mut *mut c_char)
                        -> c_int;
    #[link_name="switch_core_db_finalize"]
    pub fn core_db_finalize(pStmt: *mut core_db_stmt) -> c_int;
    #[link_name="switch_core_db_prepare"]
    pub fn core_db_prepare(db: *mut core_db,
                           zSql: *const c_char,
                           nBytes: c_int,
                           ppStmt: *mut *mut core_db_stmt,
                           pzTail: *mut *const c_char)
                           -> c_int;
    #[link_name="switch_core_db_step"]
    pub fn core_db_step(stmt: *mut core_db_stmt) -> c_int;
    #[link_name="switch_core_db_reset"]
    pub fn core_db_reset(pStmt: *mut core_db_stmt) -> c_int;
    #[link_name="switch_core_db_bind_int"]
    pub fn core_db_bind_int(pStmt: *mut core_db_stmt, i: c_int, iValue: c_int) -> c_int;
    #[link_name="switch_core_db_bind_int64"]
    pub fn core_db_bind_int64(pStmt: *mut core_db_stmt, i: c_int, iValue: i64) -> c_int;
    #[link_name="switch_core_db_bind_text"]
    pub fn core_db_bind_text(pStmt: *mut core_db_stmt,
                             i: c_int,
                             zData: *const c_char,
                             nData: c_int,
                             xDel: core_db_destructor_type)
                             -> c_int;
    #[link_name="switch_core_db_bind_double"]
    pub fn core_db_bind_double(pStmt: *mut core_db_stmt, i: c_int, dValue: f64) -> c_int;
    #[link_name="switch_core_db_last_insert_rowid"]
    pub fn core_db_last_insert_rowid(db: *mut core_db) -> i64;
    #[link_name="switch_core_db_get_table"]
    pub fn core_db_get_table(db: *mut core_db,
                             sql: *const c_char,
                             resultp: *mut *mut *mut c_char,
                             nrow: *mut c_int,
                             ncolumn: *mut c_int,
                             errmsg: *mut *mut c_char)
                             -> c_int;
    #[link_name="switch_core_db_free_table"]
    pub fn core_db_free_table(result: *mut *mut c_char);
    #[link_name="switch_core_db_free"]
    pub fn core_db_free(z: *mut c_char);
    #[link_name="switch_core_db_changes"]
    pub fn core_db_changes(db: *mut core_db) -> c_int;
    #[link_name="switch_core_db_load_extension"]
    pub fn core_db_load_extension(db: *mut core_db, extension: *const c_char) -> c_int;
    #[link_name="switch_sql_concat"]
    pub fn sql_concat() -> *mut c_char;
    #[link_name="switch_dso_destroy"]
    pub fn dso_destroy(lib: *mut dso_lib);
    #[link_name="switch_dso_open"]
    pub fn dso_open(path: *const c_char, global: c_int, err: *mut *mut c_char) -> dso_lib;
    #[link_name="switch_dso_func_sym"]
    pub fn dso_func_sym(lib: dso_lib, sym: *const c_char, err: *mut *mut c_char) -> dso_func;
    #[link_name="switch_dso_data_sym"]
    pub fn dso_data_sym(lib: dso_lib, sym: *const c_char, err: *mut *mut c_char) -> *mut c_void;
    #[link_name="switch_regex_compile"]
    pub fn regex_compile(pattern: *const c_char,
                         options: c_int,
                         errorptr: *mut *const c_char,
                         erroroffset: *mut c_int,
                         tables: *const c_uchar)
                         -> *mut regex;
    #[link_name="switch_regex_copy_substring"]
    pub fn regex_copy_substring(subject: *const c_char,
                                ovector: *mut c_int,
                                stringcount: c_int,
                                stringnumber: c_int,
                                buffer: *mut c_char,
                                size: c_int)
                                -> c_int;
    #[link_name="switch_regex_free"]
    pub fn regex_free(data: *mut c_void);
    #[link_name="switch_regex_perform"]
    pub fn regex_perform(field: *const c_char,
                         expression: *const c_char,
                         new_re: *mut *mut regex,
                         ovector: *mut c_int,
                         olen: u32)
                         -> c_int;
    #[link_name="switch_perform_substitution"]
    pub fn perform_substitution(re: *mut regex,
                                match_count: c_int,
                                data: *const c_char,
                                field_data: *const c_char,
                                substituted: *mut c_char,
                                len: usize,
                                ovector: *mut c_int);
    #[link_name="switch_regex_match"]
    pub fn regex_match(target: *const c_char, expression: *const c_char) -> status;
    #[link_name="switch_regex_match_partial"]
    pub fn regex_match_partial(target: *const c_char,
                               expression: *const c_char,
                               partial_match: *mut c_int)
                               -> status;
    #[link_name="switch_capture_regex"]
    pub fn capture_regex(re: *mut regex,
                         match_count: c_int,
                         field_data: *const c_char,
                         ovector: *mut c_int,
                         var: *const c_char,
                         callback: cap_callback,
                         user_data: *mut c_void);
    #[link_name="switch_regex_set_var_callback"]
    pub fn regex_set_var_callback(var: *const c_char, val: *const c_char, user_data: *mut c_void);
    #[link_name="switch_regex_set_event_header_callback"]
    pub fn regex_set_event_header_callback(var: *const c_char,
                                           val: *const c_char,
                                           user_data: *mut c_void);
    #[link_name="switch_core_screen_size"]
    pub fn core_screen_size(x: *mut c_int, y: *mut c_int);
    #[link_name="switch_core_session_sched_heartbeat"]
    pub fn core_session_sched_heartbeat(session: *mut core_session, seconds: u32);
    #[link_name="switch_core_session_unsched_heartbeat"]
    pub fn core_session_unsched_heartbeat(session: *mut core_session);
    #[link_name="switch_core_session_enable_heartbeat"]
    pub fn core_session_enable_heartbeat(session: *mut core_session, seconds: u32);
    #[link_name="switch_core_session_disable_heartbeat"]
    pub fn core_session_disable_heartbeat(session: *mut core_session);
    #[link_name="switch_core_media_bug_pop"]
    pub fn core_media_bug_pop(orig_session: *mut core_session,
                              function: *const c_char,
                              pop: *mut *mut media_bug)
                              -> status;
    #[link_name="switch_core_media_bug_exec_all"]
    pub fn core_media_bug_exec_all(orig_session: *mut core_session,
                                   function: *const c_char,
                                   cb: media_bug_exec_cb,
                                   user_data: *mut c_void)
                                   -> status;
    #[link_name="switch_core_media_bug_patch_video"]
    pub fn core_media_bug_patch_video(orig_session: *mut core_session, frame: *mut frame) -> u32;
    #[link_name="switch_core_media_bug_count"]
    pub fn core_media_bug_count(orig_session: *mut core_session, function: *const c_char) -> u32;
    #[link_name="switch_media_bug_set_spy_fmt"]
    pub fn media_bug_set_spy_fmt(bug: *mut media_bug, spy_fmt: vid_spy_fmt);
    #[link_name="switch_core_media_bug_push_spy_frame"]
    pub fn core_media_bug_push_spy_frame(bug: *mut media_bug, frame: *mut frame, rw: rw) -> status;
    #[link_name="switch_core_media_bug_patch_spy_frame"]
    pub fn core_media_bug_patch_spy_frame(bug: *mut media_bug,
                                          img: *mut vpx_image,
                                          rw: rw)
                                          -> status;
    #[link_name="switch_media_bug_parse_spy_fmt"]
    pub fn media_bug_parse_spy_fmt(name: *const c_char) -> vid_spy_fmt;
    #[link_name="switch_core_media_bug_add"]
    pub fn core_media_bug_add(session: *mut core_session,
                              function: *const c_char,
                              target: *const c_char,
                              callback: media_bug_callback,
                              user_data: *mut c_void,
                              stop_time: time_t,
                              flags: media_bug_flag,
                              new_bug: *mut *mut media_bug)
                              -> status;
    #[link_name="switch_core_media_bug_pause"]
    pub fn core_media_bug_pause(session: *mut core_session);
    #[link_name="switch_core_media_bug_resume"]
    pub fn core_media_bug_resume(session: *mut core_session);
    #[link_name="switch_core_media_bug_inuse"]
    pub fn core_media_bug_inuse(bug: *mut media_bug, readp: *mut usize, writep: *mut usize);
    #[link_name="switch_core_media_bug_get_user_data"]
    pub fn core_media_bug_get_user_data(bug: *mut media_bug) -> *mut c_void;
    #[link_name="switch_core_media_bug_get_write_replace_frame"]
    pub fn core_media_bug_get_write_replace_frame(bug: *mut media_bug) -> *mut frame;
    #[link_name="switch_core_media_bug_get_native_read_frame"]
    pub fn core_media_bug_get_native_read_frame(bug: *mut media_bug) -> *mut frame;
    #[link_name="switch_core_media_bug_get_native_write_frame"]
    pub fn core_media_bug_get_native_write_frame(bug: *mut media_bug) -> *mut frame;
    #[link_name="switch_core_media_bug_get_video_ping_frame"]
    pub fn core_media_bug_get_video_ping_frame(bug: *mut media_bug) -> *mut frame;
    #[link_name="switch_core_media_bug_set_write_replace_frame"]
    pub fn core_media_bug_set_write_replace_frame(bug: *mut media_bug, frame: *mut frame);
    #[link_name="switch_core_media_bug_get_read_replace_frame"]
    pub fn core_media_bug_get_read_replace_frame(bug: *mut media_bug) -> *mut frame;
    #[link_name="switch_core_media_bug_set_read_demux_frame"]
    pub fn core_media_bug_set_read_demux_frame(bug: *mut media_bug, frame: *mut frame);
    #[link_name="switch_core_media_bug_get_session"]
    pub fn core_media_bug_get_session(bug: *mut media_bug) -> *mut core_session;
    #[link_name="switch_core_media_bug_test_flag"]
    pub fn core_media_bug_test_flag(bug: *mut media_bug, flag: u32) -> u32;
    #[link_name="switch_core_media_bug_set_flag"]
    pub fn core_media_bug_set_flag(bug: *mut media_bug, flag: u32) -> u32;
    #[link_name="switch_core_media_bug_clear_flag"]
    pub fn core_media_bug_clear_flag(bug: *mut media_bug, flag: u32) -> u32;
    #[link_name="switch_core_media_bug_set_read_replace_frame"]
    pub fn core_media_bug_set_read_replace_frame(bug: *mut media_bug, frame: *mut frame);
    #[link_name="switch_core_cpu_count"]
    pub fn core_cpu_count() -> u32;
    #[link_name="switch_core_media_bug_remove"]
    pub fn core_media_bug_remove(session: *mut core_session, bug: *mut *mut media_bug) -> status;
    #[link_name="switch_core_media_bug_prune"]
    pub fn core_media_bug_prune(session: *mut core_session) -> u32;
    #[link_name="switch_core_media_bug_remove_callback"]
    pub fn core_media_bug_remove_callback(session: *mut core_session,
                                          callback: media_bug_callback)
                                          -> status;
    #[link_name="switch_core_media_bug_close"]
    pub fn core_media_bug_close(bug: *mut *mut media_bug) -> status;
    #[link_name="switch_core_media_bug_remove_all_function"]
    pub fn core_media_bug_remove_all_function(session: *mut core_session,
                                              function: *const c_char)
                                              -> status;
    #[link_name="switch_core_media_bug_enumerate"]
    pub fn core_media_bug_enumerate(session: *mut core_session,
                                    stream: *mut stream_handle)
                                    -> status;
    #[link_name="switch_core_media_bug_transfer_recordings"]
    pub fn core_media_bug_transfer_recordings(orig_session: *mut core_session,
                                              new_session: *mut core_session)
                                              -> status;
    #[link_name="switch_core_media_bug_transfer_callback"]
    pub fn core_media_bug_transfer_callback(orig_session:
                                                       *mut core_session,
                                                   new_session:
                                                       *mut core_session,
                                                   callback:
                                                       media_bug_callback,
                                                   user_data_dup_func:
                                                       Option<unsafe extern "C" fn(arg1:
                                                                                                      *mut core_session,
                                                                                                  arg2:
                                                                                                      *mut c_void)
                                                                                 ->
                                                                                     *mut c_void>)
     -> status;
    #[link_name="switch_core_media_bug_read"]
    pub fn core_media_bug_read(bug: *mut media_bug,
                               frame: *mut frame,
                               fill: switch_bool)
                               -> status;
    #[link_name="switch_core_media_bug_flush"]
    pub fn core_media_bug_flush(bug: *mut media_bug);
    #[link_name="switch_core_media_bug_flush_all"]
    pub fn core_media_bug_flush_all(session: *mut core_session) -> status;
    #[link_name="switch_core_media_bug_set_pre_buffer_framecount"]
    pub fn core_media_bug_set_pre_buffer_framecount(bug: *mut media_bug,
                                                    framecount: u32)
                                                    -> status;
    #[link_name="switch_core_port_allocator_new"]
    pub fn core_port_allocator_new(ip: *const c_char,
                                   start: port,
                                   end: port,
                                   flags: port_flag,
                                   new_allocator: *mut *mut core_port_allocator)
                                   -> status;
    #[link_name="switch_core_port_allocator_request_port"]
    pub fn core_port_allocator_request_port(alloc: *mut core_port_allocator,
                                            port_ptr: *mut port)
                                            -> status;
    #[link_name="switch_core_port_allocator_free_port"]
    pub fn core_port_allocator_free_port(alloc: *mut core_port_allocator, port: port) -> status;
    #[link_name="switch_core_port_allocator_destroy"]
    pub fn core_port_allocator_destroy(alloc: *mut *mut core_port_allocator);
    #[link_name="switch_core_test_flag"]
    pub fn core_test_flag(flag: c_int) -> c_int;
    #[link_name="switch_core_init"]
    pub fn core_init(flags: core_flag, console: switch_bool, err: *mut *const c_char) -> status;
    #[link_name="switch_core_init_and_modload"]
    pub fn core_init_and_modload(flags: core_flag,
                                 console: switch_bool,
                                 err: *mut *const c_char)
                                 -> status;
    #[link_name="switch_core_session_limit"]
    pub fn core_session_limit(new_limit: u32) -> u32;
    #[link_name="switch_core_sessions_per_second"]
    pub fn core_sessions_per_second(new_limit: u32) -> u32;
    #[link_name="switch_core_destroy"]
    pub fn core_destroy() -> status;
    #[link_name="switch_core_session_io_read_lock"]
    pub fn core_session_io_read_lock(session: *mut core_session) -> status;
    #[link_name="switch_core_session_io_write_lock"]
    pub fn core_session_io_write_lock(session: *mut core_session) -> status;
    #[link_name="switch_core_session_io_rwunlock"]
    pub fn core_session_io_rwunlock(session: *mut core_session) -> status;
    #[link_name="switch_core_session_read_lock"]
    pub fn core_session_read_lock(session: *mut core_session) -> status;
    #[link_name="switch_core_session_read_lock_hangup"]
    pub fn core_session_read_lock_hangup(session: *mut core_session) -> status;
    #[link_name="switch_core_session_write_lock"]
    pub fn core_session_write_lock(session: *mut core_session);
    #[link_name="switch_core_session_rwunlock"]
    pub fn core_session_rwunlock(session: *mut core_session);
    #[link_name="switch_core_add_state_handler"]
    pub fn core_add_state_handler(state_handler: *const state_handler_table) -> c_int;
    #[link_name="switch_core_remove_state_handler"]
    pub fn core_remove_state_handler(state_handler: *const state_handler_table);
    #[link_name="switch_core_get_state_handler"]
    pub fn core_get_state_handler(index: c_int) -> *const state_handler_table;
    #[link_name="switch_core_memory_pool_tag"]
    pub fn core_memory_pool_tag(pool: *mut memory_pool, tag: *const c_char);
    #[link_name="switch_core_perform_new_memory_pool"]
    pub fn core_perform_new_memory_pool(pool: *mut *mut memory_pool,
                                        file: *const c_char,
                                        func: *const c_char,
                                        line: c_int)
                                        -> status;
    #[link_name="switch_core_session_sync_clock"]
    pub fn core_session_sync_clock() -> c_int;
    #[link_name="switch_core_perform_destroy_memory_pool"]
    pub fn core_perform_destroy_memory_pool(pool: *mut *mut memory_pool,
                                            file: *const c_char,
                                            func: *const c_char,
                                            line: c_int)
                                            -> status;
    #[link_name="switch_core_memory_pool_set_data"]
    pub fn core_memory_pool_set_data(pool: *mut memory_pool,
                                     key: *const c_char,
                                     data: *mut c_void);
    #[link_name="switch_core_memory_pool_get_data"]
    pub fn core_memory_pool_get_data(pool: *mut memory_pool, key: *const c_char) -> *mut c_void;
    #[link_name="switch_core_session_run"]
    pub fn core_session_run(session: *mut core_session);
    #[link_name="switch_core_session_running"]
    pub fn core_session_running(session: *mut core_session) -> c_uint;
    #[link_name="switch_core_session_started"]
    pub fn core_session_started(session: *mut core_session) -> c_uint;
    #[link_name="switch_core_perform_permanent_alloc"]
    pub fn core_perform_permanent_alloc(memory: usize,
                                        file: *const c_char,
                                        func: *const c_char,
                                        line: c_int)
                                        -> *mut c_void;
    #[link_name="switch_core_perform_alloc"]
    pub fn core_perform_alloc(pool: *mut memory_pool,
                              memory: usize,
                              file: *const c_char,
                              func: *const c_char,
                              line: c_int)
                              -> *mut c_void;
    #[link_name="switch_core_perform_session_alloc"]
    pub fn core_perform_session_alloc(session: *mut core_session,
                                      memory: usize,
                                      file: *const c_char,
                                      func: *const c_char,
                                      line: c_int)
                                      -> *mut c_void;
    #[link_name="switch_core_perform_permanent_strdup"]
    pub fn core_perform_permanent_strdup(todup: *const c_char,
                                         file: *const c_char,
                                         func: *const c_char,
                                         line: c_int)
                                         -> *mut c_char;
    #[link_name="switch_core_perform_session_strdup"]
    pub fn core_perform_session_strdup(session: *mut core_session,
                                       todup: *const c_char,
                                       file: *const c_char,
                                       func: *const c_char,
                                       line: c_int)
                                       -> *mut c_char;
    #[link_name="switch_core_perform_strdup"]
    pub fn core_perform_strdup(pool: *mut memory_pool,
                               todup: *const c_char,
                               file: *const c_char,
                               func: *const c_char,
                               line: c_int)
                               -> *mut c_char;
    #[link_name="switch_core_session_sprintf"]
    pub fn core_session_sprintf(session: *mut core_session,
                                fmt: *const c_char,
                                ...)
                                -> *mut c_char;
    #[link_name="switch_core_sprintf"]
    pub fn core_sprintf(pool: *mut memory_pool, fmt: *const c_char, ...) -> *mut c_char;
    #[link_name="switch_core_session_get_pool"]
    pub fn core_session_get_pool(session: *mut core_session) -> *mut memory_pool;
    #[link_name="switch_core_session_request_xml"]
    pub fn core_session_request_xml(endpoint_interface: *mut endpoint_interface,
                                    pool: *mut *mut memory_pool,
                                    xml: xml_t)
                                    -> *mut core_session;
    #[link_name="switch_core_session_request_uuid"]
    pub fn core_session_request_uuid(endpoint_interface: *mut endpoint_interface,
                                     direction: call_direction,
                                     originate_flags: originate_flag,
                                     pool: *mut *mut memory_pool,
                                     use_uuid: *const c_char)
                                     -> *mut core_session;
    #[link_name="switch_core_session_set_uuid"]
    pub fn core_session_set_uuid(session: *mut core_session, use_uuid: *const c_char) -> status;
    #[link_name="switch_core_session_perform_destroy"]
    pub fn core_session_perform_destroy(session: *mut *mut core_session,
                                        file: *const c_char,
                                        func: *const c_char,
                                        line: c_int);
    #[link_name="switch_core_session_destroy_state"]
    pub fn core_session_destroy_state(session: *mut core_session);
    #[link_name="switch_core_session_reporting_state"]
    pub fn core_session_reporting_state(session: *mut core_session);
    #[link_name="switch_core_session_hangup_state"]
    pub fn core_session_hangup_state(session: *mut core_session, force: switch_bool);
    #[link_name="switch_core_session_count"]
    pub fn core_session_count() -> u32;
    #[link_name="switch_core_session_get_id"]
    pub fn core_session_get_id(session: *mut core_session) -> usize;
    #[link_name="switch_core_session_id"]
    pub fn core_session_id() -> usize;
    #[link_name="switch_core_session_id_dec"]
    pub fn core_session_id_dec() -> usize;
    #[link_name="switch_core_session_request_by_name"]
    pub fn core_session_request_by_name(endpoint_name: *const c_char,
                                        direction: call_direction,
                                        pool: *mut *mut memory_pool)
                                        -> *mut core_session;
    #[link_name="switch_core_session_thread_launch"]
    pub fn core_session_thread_launch(session: *mut core_session) -> status;
    #[link_name="switch_thread_pool_launch_thread"]
    pub fn thread_pool_launch_thread(tdp: *mut *mut thread_data) -> status;
    #[link_name="switch_core_session_thread_pool_launch"]
    pub fn core_session_thread_pool_launch(session: *mut core_session) -> status;
    #[link_name="switch_core_session_get_channel"]
    pub fn core_session_get_channel(session: *mut core_session) -> *mut channel;
    #[link_name="switch_core_session_get_mutex"]
    pub fn core_session_get_mutex(session: *mut core_session) -> *mut mutex;
    #[link_name="switch_core_session_wake_session_thread"]
    pub fn core_session_wake_session_thread(session: *mut core_session) -> status;
    #[link_name="switch_core_session_signal_state_change"]
    pub fn core_session_signal_state_change(session: *mut core_session);
    #[link_name="switch_core_session_get_uuid"]
    pub fn core_session_get_uuid(session: *mut core_session) -> *mut c_char;
    #[link_name="switch_core_session_set_loglevel"]
    pub fn core_session_set_loglevel(session: *mut core_session, loglevel: log_level) -> status;
    #[link_name="switch_core_session_get_loglevel"]
    pub fn core_session_get_loglevel(session: *mut core_session) -> log_level;
    #[link_name="switch_core_session_get_jb"]
    pub fn core_session_get_jb(session: *mut core_session, type_: media_type) -> *mut jb;
    #[link_name="switch_core_session_soft_lock"]
    pub fn core_session_soft_lock(session: *mut core_session, sec: u32);
    #[link_name="switch_core_session_soft_unlock"]
    pub fn core_session_soft_unlock(session: *mut core_session);
    #[link_name="switch_core_session_set_dmachine"]
    pub fn core_session_set_dmachine(session: *mut core_session,
                                     dmachine: *mut ivr_dmachine,
                                     target: digit_action_target);
    #[link_name="switch_core_session_get_dmachine"]
    pub fn core_session_get_dmachine(session: *mut core_session,
                                     target: digit_action_target)
                                     -> *mut ivr_dmachine;
    #[link_name="switch_ivr_dmachine_get_target"]
    pub fn ivr_dmachine_get_target(dmachine: *mut ivr_dmachine) -> digit_action_target;
    #[link_name="switch_ivr_dmachine_set_target"]
    pub fn ivr_dmachine_set_target(dmachine: *mut ivr_dmachine, target: digit_action_target);
    #[link_name="switch_ivr_dmachine_set_terminators"]
    pub fn ivr_dmachine_set_terminators(dmachine: *mut ivr_dmachine,
                                        terminators: *const c_char)
                                        -> status;
    #[link_name="switch_core_session_set_codec_slin"]
    pub fn core_session_set_codec_slin(session: *mut core_session, data: *mut slin_data) -> status;
    #[link_name="switch_core_session_raw_read"]
    pub fn core_session_raw_read(session: *mut core_session);
    #[link_name="switch_core_get_uuid"]
    pub fn core_get_uuid() -> *mut c_char;
    #[link_name="switch_core_session_perform_locate"]
    pub fn core_session_perform_locate(uuid_str: *const c_char,
                                       file: *const c_char,
                                       func: *const c_char,
                                       line: c_int)
                                       -> *mut core_session;
    #[link_name="switch_core_session_perform_force_locate"]
    pub fn core_session_perform_force_locate(uuid_str: *const c_char,
                                             file: *const c_char,
                                             func: *const c_char,
                                             line: c_int)
                                             -> *mut core_session;
    #[link_name="switch_core_get_variable"]
    pub fn core_get_variable(varname: *const c_char) -> *mut c_char;
    #[link_name="switch_core_get_variable_dup"]
    pub fn core_get_variable_dup(varname: *const c_char) -> *mut c_char;
    #[link_name="switch_core_get_variable_pdup"]
    pub fn core_get_variable_pdup(varname: *const c_char, pool: *mut memory_pool) -> *mut c_char;
    #[link_name="switch_core_get_hostname"]
    pub fn core_get_hostname() -> *const c_char;
    #[link_name="switch_core_get_switchname"]
    pub fn core_get_switchname() -> *const c_char;
    #[link_name="switch_core_get_domain"]
    pub fn core_get_domain(dup: switch_bool) -> *mut c_char;
    #[link_name="switch_core_set_variable"]
    pub fn core_set_variable(varname: *const c_char, value: *const c_char);
    #[link_name="switch_core_get_variables"]
    pub fn core_get_variables(event: *mut *mut event) -> status;
    #[link_name="switch_core_set_var_conditional"]
    pub fn core_set_var_conditional(varname: *const c_char,
                                    value: *const c_char,
                                    val2: *const c_char)
                                    -> switch_bool;
    #[link_name="switch_core_dump_variables"]
    pub fn core_dump_variables(stream: *mut stream_handle);
    #[link_name="switch_core_session_hupall"]
    pub fn core_session_hupall(cause: call_cause);
    #[link_name="switch_core_session_hupall_matching_var_ans"]
    pub fn core_session_hupall_matching_var_ans(var_name: *const c_char,
                                                var_val: *const c_char,
                                                cause: call_cause,
                                                type_: hup_type)
                                                -> u32;
    #[link_name="switch_core_session_findall_matching_var"]
    pub fn core_session_findall_matching_var(var_name: *const c_char,
                                             var_val: *const c_char)
                                             -> *mut console_callback_match;
    #[link_name="switch_core_session_findall"]
    pub fn core_session_findall() -> *mut console_callback_match;
    #[link_name="switch_core_session_hupall_endpoint"]
    pub fn core_session_hupall_endpoint(endpoint_interface: *const endpoint_interface,
                                        cause: call_cause);
    #[link_name="switch_core_session_perform_get_partner"]
    pub fn core_session_perform_get_partner(session: *mut core_session,
                                            partner: *mut *mut core_session,
                                            file: *const c_char,
                                            func: *const c_char,
                                            line: c_int)
                                            -> status;
    #[link_name="switch_core_session_message_send"]
    pub fn core_session_message_send(uuid_str: *const c_char,
                                     message: *mut core_session_message)
                                     -> status;
    #[link_name="switch_core_session_queue_message"]
    pub fn core_session_queue_message(session: *mut core_session,
                                      message: *mut core_session_message)
                                      -> status;
    #[link_name="switch_core_session_free_message"]
    pub fn core_session_free_message(message: *mut *mut core_session_message);
    #[link_name="switch_core_session_queue_signal_data"]
    pub fn core_session_queue_signal_data(session: *mut core_session,
                                          signal_data: *mut c_void)
                                          -> status;
    #[link_name="switch_core_session_dequeue_signal_data"]
    pub fn core_session_dequeue_signal_data(session: *mut core_session,
                                            signal_data: *mut *mut c_void)
                                            -> status;
    #[link_name="switch_core_session_pass_indication"]
    pub fn core_session_pass_indication(session: *mut core_session,
                                        indication: core_session_message_types)
                                        -> status;
    #[link_name="switch_core_session_queue_indication"]
    pub fn core_session_queue_indication(session: *mut core_session,
                                         indication: core_session_message_types)
                                         -> status;
    #[link_name="switch_core_session_dequeue_message"]
    pub fn core_session_dequeue_message(session: *mut core_session,
                                        message: *mut *mut core_session_message)
                                        -> status;
    #[link_name="switch_core_session_flush_message"]
    pub fn core_session_flush_message(session: *mut core_session) -> status;
    #[link_name="switch_core_session_event_send"]
    pub fn core_session_event_send(uuid_str: *const c_char, event: *mut *mut event) -> status;
    #[link_name="switch_core_session_get_app_log"]
    pub fn core_session_get_app_log(session: *mut core_session) -> *mut app_log;
    #[link_name="switch_core_session_exec"]
    pub fn core_session_exec(session: *mut core_session,
                             application_interface: *const application_interface,
                             arg: *const c_char)
                             -> status;
    #[link_name="switch_core_session_video_reset"]
    pub fn core_session_video_reset(session: *mut core_session);
    #[link_name="switch_core_session_execute_application_get_flags"]
    pub fn core_session_execute_application_get_flags(session: *mut core_session,
                                                      app: *const c_char,
                                                      arg: *const c_char,
                                                      flags: *mut i32)
                                                      -> status;
    #[link_name="switch_core_session_execute_application_async"]
    pub fn core_session_execute_application_async(session: *mut core_session,
                                                  app: *const c_char,
                                                  arg: *const c_char)
                                                  -> status;
    #[link_name="switch_core_session_get_app_flags"]
    pub fn core_session_get_app_flags(app: *const c_char, flags: *mut i32) -> status;
    #[link_name="switch_core_session_execute_exten"]
    pub fn core_session_execute_exten(session: *mut core_session,
                                      exten: *const c_char,
                                      dialplan: *const c_char,
                                      context: *const c_char)
                                      -> status;
    #[link_name="switch_core_session_receive_event"]
    pub fn core_session_receive_event(session: *mut core_session,
                                      event: *mut *mut event)
                                      -> status;
    #[link_name="switch_core_session_get_private_class"]
    pub fn core_session_get_private_class(session: *mut core_session,
                                          index: pvt_class)
                                          -> *mut c_void;
    #[link_name="switch_core_session_set_private_class"]
    pub fn core_session_set_private_class(session: *mut core_session,
                                          private_info: *mut c_void,
                                          index: pvt_class)
                                          -> status;
    #[link_name="switch_core_session_add_stream"]
    pub fn core_session_add_stream(session: *mut core_session, private_info: *mut c_void) -> c_int;
    #[link_name="switch_core_session_get_stream"]
    pub fn core_session_get_stream(session: *mut core_session, index: c_int) -> *mut c_void;
    #[link_name="switch_core_session_get_stream_count"]
    pub fn core_session_get_stream_count(session: *mut core_session) -> c_int;
    #[link_name="switch_core_session_launch_thread"]
    pub fn core_session_launch_thread(session: *mut core_session,
                                      func: Option<unsafe extern "C" fn(arg1: *mut thread,
                                                                        arg2: *mut c_void)
                                                                        -> *mut c_void>,
                                      obj: *mut c_void);
    #[link_name="switch_core_thread_session_end"]
    pub fn core_thread_session_end(session: *mut core_session);
    #[link_name="switch_core_service_session_av"]
    pub fn core_service_session_av(session: *mut core_session,
                                   audio: switch_bool,
                                   video: switch_bool);
    #[link_name="switch_core_session_outgoing_channel"]
    pub fn core_session_outgoing_channel(session: *mut core_session,
                                         var_event: *mut event,
                                         endpoint_name: *const c_char,
                                         caller_profile: *mut caller_profile,
                                         new_session: *mut *mut core_session,
                                         pool: *mut *mut memory_pool,
                                         flags: originate_flag,
                                         cancel_cause: *mut call_cause)
                                         -> call_cause;
    #[link_name="switch_core_session_perform_receive_message"]
    pub fn core_session_perform_receive_message(session: *mut core_session,
                                                message: *mut core_session_message,
                                                file: *const c_char,
                                                func: *const c_char,
                                                line: c_int)
                                                -> status;
    #[link_name="switch_core_session_queue_event"]
    pub fn core_session_queue_event(session: *mut core_session, event: *mut *mut event) -> status;
    #[link_name="switch_core_session_event_count"]
    pub fn core_session_event_count(session: *mut core_session) -> u32;
    #[link_name="switch_core_session_messages_waiting"]
    pub fn core_session_messages_waiting(session: *mut core_session) -> u32;
    #[link_name="switch_core_session_dequeue_event"]
    pub fn core_session_dequeue_event(session: *mut core_session,
                                      event: *mut *mut event,
                                      force: switch_bool)
                                      -> status;
    #[link_name="switch_core_session_queue_private_event"]
    pub fn core_session_queue_private_event(session: *mut core_session,
                                            event: *mut *mut event,
                                            priority: switch_bool)
                                            -> status;
    #[link_name="switch_core_session_private_event_count"]
    pub fn core_session_private_event_count(session: *mut core_session) -> u32;
    #[link_name="switch_core_session_dequeue_private_event"]
    pub fn core_session_dequeue_private_event(session: *mut core_session,
                                              event: *mut *mut event)
                                              -> status;
    #[link_name="switch_core_session_flush_private_events"]
    pub fn core_session_flush_private_events(session: *mut core_session) -> u32;
    #[link_name="switch_core_session_read_frame"]
    pub fn core_session_read_frame(session: *mut core_session,
                                   frame: *mut *mut frame,
                                   flags: io_flag,
                                   stream_id: c_int)
                                   -> status;
    #[link_name="switch_core_session_read_video_frame"]
    pub fn core_session_read_video_frame(session: *mut core_session,
                                         frame: *mut *mut frame,
                                         flags: io_flag,
                                         stream_id: c_int)
                                         -> status;
    #[link_name="switch_core_session_write_video_frame"]
    pub fn core_session_write_video_frame(session: *mut core_session,
                                          frame: *mut frame,
                                          flags: io_flag,
                                          stream_id: c_int)
                                          -> status;
    #[link_name="switch_core_session_write_encoded_video_frame"]
    pub fn core_session_write_encoded_video_frame(session: *mut core_session,
                                                  frame: *mut frame,
                                                  flags: io_flag,
                                                  stream_id: c_int)
                                                  -> status;
    #[link_name="switch_core_session_set_read_impl"]
    pub fn core_session_set_read_impl(session: *mut core_session,
                                      impp: *const codec_implementation)
                                      -> status;
    #[link_name="switch_core_session_set_write_impl"]
    pub fn core_session_set_write_impl(session: *mut core_session,
                                       impp: *const codec_implementation)
                                       -> status;
    #[link_name="switch_core_session_set_video_read_impl"]
    pub fn core_session_set_video_read_impl(session: *mut core_session,
                                            impp: *const codec_implementation)
                                            -> status;
    #[link_name="switch_core_session_set_video_write_impl"]
    pub fn core_session_set_video_write_impl(session: *mut core_session,
                                             impp: *const codec_implementation)
                                             -> status;
    #[link_name="switch_core_session_reset"]
    pub fn core_session_reset(session: *mut core_session,
                              flush_dtmf: switch_bool,
                              reset_read_codec: switch_bool);
    #[link_name="switch_core_session_write_frame"]
    pub fn core_session_write_frame(session: *mut core_session,
                                    frame: *mut frame,
                                    flags: io_flag,
                                    stream_id: c_int)
                                    -> status;
    #[link_name="switch_core_session_perform_kill_channel"]
    pub fn core_session_perform_kill_channel(session: *mut core_session,
                                             file: *const c_char,
                                             func: *const c_char,
                                             line: c_int,
                                             sig: signal)
                                             -> status;
    #[link_name="switch_core_session_send_dtmf"]
    pub fn core_session_send_dtmf(session: *mut core_session, dtmf: *const dtmf) -> status;
    #[link_name="switch_core_session_send_dtmf_string"]
    pub fn core_session_send_dtmf_string(session: *mut core_session,
                                         dtmf_string: *const c_char)
                                         -> status;
    #[link_name="switch_core_session_recv_dtmf"]
    pub fn core_session_recv_dtmf(session: *mut core_session, dtmf: *const dtmf) -> status;
    #[link_name="switch_core_hash_init_case"]
    pub fn core_hash_init_case(hash: *mut *mut hash, case_sensitive: switch_bool) -> status;
    #[link_name="switch_core_hash_destroy"]
    pub fn core_hash_destroy(hash: *mut *mut hash) -> status;
    #[link_name="switch_core_hash_insert_destructor"]
    pub fn core_hash_insert_destructor(hash: *mut hash,
                                       key: *const c_char,
                                       data: *const c_void,
                                       destructor: hashtable_destructor)
                                       -> status;
    #[link_name="switch_core_hash_insert_locked"]
    pub fn core_hash_insert_locked(hash: *mut hash,
                                   key: *const c_char,
                                   data: *const c_void,
                                   mutex: *mut mutex)
                                   -> status;
    #[link_name="switch_core_hash_insert_wrlock"]
    pub fn core_hash_insert_wrlock(hash: *mut hash,
                                   key: *const c_char,
                                   data: *const c_void,
                                   rwlock: *mut thread_rwlock)
                                   -> status;
    #[link_name="switch_core_hash_delete"]
    pub fn core_hash_delete(hash: *mut hash, key: *const c_char) -> *mut c_void;
    #[link_name="switch_core_hash_delete_locked"]
    pub fn core_hash_delete_locked(hash: *mut hash,
                                   key: *const c_char,
                                   mutex: *mut mutex)
                                   -> *mut c_void;
    #[link_name="switch_core_hash_delete_wrlock"]
    pub fn core_hash_delete_wrlock(hash: *mut hash,
                                   key: *const c_char,
                                   rwlock: *mut thread_rwlock)
                                   -> *mut c_void;
    #[link_name="switch_core_hash_delete_multi"]
    pub fn core_hash_delete_multi(hash: *mut hash,
                                  callback: hash_delete_callback,
                                  pData: *mut c_void)
                                  -> status;
    #[link_name="switch_core_hash_find"]
    pub fn core_hash_find(hash: *mut hash, key: *const c_char) -> *mut c_void;
    #[link_name="switch_core_hash_find_locked"]
    pub fn core_hash_find_locked(hash: *mut hash,
                                 key: *const c_char,
                                 mutex: *mut mutex)
                                 -> *mut c_void;
    #[link_name="switch_core_hash_find_rdlock"]
    pub fn core_hash_find_rdlock(hash: *mut hash,
                                 key: *const c_char,
                                 rwlock: *mut thread_rwlock)
                                 -> *mut c_void;
    #[link_name="switch_core_hash_first_iter"]
    pub fn core_hash_first_iter(hash: *mut hash, hi: *mut hash_index) -> *mut hash_index;
    #[link_name="switch_core_hash_empty"]
    pub fn core_hash_empty(hash: *mut hash) -> switch_bool;
    #[link_name="switch_core_hash_next"]
    pub fn core_hash_next(hi: *mut *mut hash_index) -> *mut hash_index;
    #[link_name="switch_core_hash_this"]
    pub fn core_hash_this(hi: *mut hash_index,
                          key: *mut *const c_void,
                          klen: *mut isize,
                          val: *mut *mut c_void);
    #[link_name="switch_core_hash_this_val"]
    pub fn core_hash_this_val(hi: *mut hash_index, val: *mut c_void);
    #[link_name="switch_core_inthash_init"]
    pub fn core_inthash_init(hash: *mut *mut inthash) -> status;
    #[link_name="switch_core_inthash_destroy"]
    pub fn core_inthash_destroy(hash: *mut *mut inthash) -> status;
    #[link_name="switch_core_inthash_insert"]
    pub fn core_inthash_insert(hash: *mut inthash, key: u32, data: *const c_void) -> status;
    #[link_name="switch_core_inthash_delete"]
    pub fn core_inthash_delete(hash: *mut inthash, key: u32) -> *mut c_void;
    #[link_name="switch_core_inthash_find"]
    pub fn core_inthash_find(hash: *mut inthash, key: u32) -> *mut c_void;
    #[link_name="switch_core_timer_init"]
    pub fn core_timer_init(timer: *mut timer,
                           timer_name: *const c_char,
                           interval: c_int,
                           samples: c_int,
                           pool: *mut memory_pool)
                           -> status;
    #[link_name="switch_time_calibrate_clock"]
    pub fn time_calibrate_clock();
    #[link_name="switch_core_timer_next"]
    pub fn core_timer_next(timer: *mut timer) -> status;
    #[link_name="switch_core_timer_step"]
    pub fn core_timer_step(timer: *mut timer) -> status;
    #[link_name="switch_core_timer_sync"]
    pub fn core_timer_sync(timer: *mut timer) -> status;
    #[link_name="switch_core_timer_check"]
    pub fn core_timer_check(timer: *mut timer, step: switch_bool) -> status;
    #[link_name="switch_core_timer_destroy"]
    pub fn core_timer_destroy(timer: *mut timer) -> status;
    #[link_name="switch_core_codec_init_with_bitrate"]
    pub fn core_codec_init_with_bitrate(codec: *mut codec,
                                        codec_name: *const c_char,
                                        fmtp: *const c_char,
                                        modname: *const c_char,
                                        rate: u32,
                                        ms: c_int,
                                        channels: c_int,
                                        bitrate: u32,
                                        flags: u32,
                                        codec_settings: *const codec_settings,
                                        pool: *mut memory_pool)
                                        -> status;
    #[link_name="switch_core_codec_copy"]
    pub fn core_codec_copy(codec: *mut codec,
                           new_codec: *mut codec,
                           codec_settings: *const codec_settings,
                           pool: *mut memory_pool)
                           -> status;
    #[link_name="switch_core_codec_parse_fmtp"]
    pub fn core_codec_parse_fmtp(codec_name: *const c_char,
                                 fmtp: *const c_char,
                                 rate: u32,
                                 codec_fmtp: *mut codec_fmtp)
                                 -> status;
    #[link_name="switch_core_codec_reset"]
    pub fn core_codec_reset(codec: *mut codec) -> status;
    #[link_name="switch_core_codec_encode"]
    pub fn core_codec_encode(codec: *mut codec,
                             other_codec: *mut codec,
                             decoded_data: *mut c_void,
                             decoded_data_len: u32,
                             decoded_rate: u32,
                             encoded_data: *mut c_void,
                             encoded_data_len: *mut u32,
                             encoded_rate: *mut u32,
                             flag: *mut c_uint)
                             -> status;
    #[link_name="switch_core_codec_decode"]
    pub fn core_codec_decode(codec: *mut codec,
                             other_codec: *mut codec,
                             encoded_data: *mut c_void,
                             encoded_data_len: u32,
                             encoded_rate: u32,
                             decoded_data: *mut c_void,
                             decoded_data_len: *mut u32,
                             decoded_rate: *mut u32,
                             flag: *mut c_uint)
                             -> status;
    #[link_name="switch_core_codec_encode_video"]
    pub fn core_codec_encode_video(codec: *mut codec, frame: *mut frame) -> status;
    #[link_name="switch_core_codec_control"]
    pub fn core_codec_control(codec: *mut codec,
                              cmd: codec_control_command,
                              ctype: codec_control_type,
                              cmd_data: *mut c_void,
                              atype: codec_control_type,
                              cmd_arg: *mut c_void,
                              rtype: *mut codec_control_type,
                              ret_data: *mut *mut c_void)
                              -> status;
    #[link_name="switch_core_codec_decode_video"]
    pub fn core_codec_decode_video(codec: *mut codec, frame: *mut frame) -> status;
    #[link_name="switch_core_codec_destroy"]
    pub fn core_codec_destroy(codec: *mut codec) -> status;
    #[link_name="switch_core_session_set_read_codec"]
    pub fn core_session_set_read_codec(session: *mut core_session, codec: *mut codec) -> status;
    #[link_name="switch_core_session_set_real_read_codec"]
    pub fn core_session_set_real_read_codec(session: *mut core_session,
                                            codec: *mut codec)
                                            -> status;
    #[link_name="switch_core_session_unset_read_codec"]
    pub fn core_session_unset_read_codec(session: *mut core_session);
    #[link_name="switch_core_session_unset_write_codec"]
    pub fn core_session_unset_write_codec(session: *mut core_session);
    #[link_name="switch_core_session_lock_codec_write"]
    pub fn core_session_lock_codec_write(session: *mut core_session);
    #[link_name="switch_core_session_unlock_codec_write"]
    pub fn core_session_unlock_codec_write(session: *mut core_session);
    #[link_name="switch_core_session_lock_codec_read"]
    pub fn core_session_lock_codec_read(session: *mut core_session);
    #[link_name="switch_core_session_unlock_codec_read"]
    pub fn core_session_unlock_codec_read(session: *mut core_session);
    #[link_name="switch_core_session_get_read_impl"]
    pub fn core_session_get_read_impl(session: *mut core_session,
                                      impp: *mut codec_implementation)
                                      -> status;
    #[link_name="switch_core_session_get_real_read_impl"]
    pub fn core_session_get_real_read_impl(session: *mut core_session,
                                           impp: *mut codec_implementation)
                                           -> status;
    #[link_name="switch_core_session_get_write_impl"]
    pub fn core_session_get_write_impl(session: *mut core_session,
                                       impp: *mut codec_implementation)
                                       -> status;
    #[link_name="switch_core_session_get_video_read_impl"]
    pub fn core_session_get_video_read_impl(session: *mut core_session,
                                            impp: *mut codec_implementation)
                                            -> status;
    #[link_name="switch_core_session_get_video_write_impl"]
    pub fn core_session_get_video_write_impl(session: *mut core_session,
                                             impp: *mut codec_implementation)
                                             -> status;
    #[link_name="switch_core_session_get_read_codec"]
    pub fn core_session_get_read_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_session_get_effective_read_codec"]
    pub fn core_session_get_effective_read_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_session_set_write_codec"]
    pub fn core_session_set_write_codec(session: *mut core_session, codec: *mut codec) -> status;
    #[link_name="switch_core_session_get_write_codec"]
    pub fn core_session_get_write_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_session_get_effective_write_codec"]
    pub fn core_session_get_effective_write_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_session_set_video_read_codec"]
    pub fn core_session_set_video_read_codec(session: *mut core_session,
                                             codec: *mut codec)
                                             -> status;
    #[link_name="switch_core_session_get_video_read_codec"]
    pub fn core_session_get_video_read_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_session_set_video_write_codec"]
    pub fn core_session_set_video_write_codec(session: *mut core_session,
                                              codec: *mut codec)
                                              -> status;
    #[link_name="switch_core_session_get_video_write_codec"]
    pub fn core_session_get_video_write_codec(session: *mut core_session) -> *mut codec;
    #[link_name="switch_core_db_open_file"]
    pub fn core_db_open_file(filename: *const c_char) -> *mut core_db;
    #[link_name="switch_core_db_persistant_execute"]
    pub fn core_db_persistant_execute(db: *mut core_db, sql: *mut c_char, retries: u32) -> status;
    #[link_name="switch_core_db_persistant_execute_trans"]
    pub fn core_db_persistant_execute_trans(db: *mut core_db,
                                            sql: *mut c_char,
                                            retries: u32)
                                            -> status;
    #[link_name="switch_core_db_test_reactive"]
    pub fn core_db_test_reactive(db: *mut core_db,
                                 test_sql: *mut c_char,
                                 drop_sql: *mut c_char,
                                 reactive_sql: *mut c_char);
    #[link_name="switch_core_perform_file_open"]
    pub fn core_perform_file_open(file: *const c_char,
                                  func: *const c_char,
                                  line: c_int,
                                  fh: *mut file_handle,
                                  file_path: *const c_char,
                                  channels: u32,
                                  rate: u32,
                                  flags: c_uint,
                                  pool: *mut memory_pool)
                                  -> status;
    #[link_name="switch_core_file_read"]
    pub fn core_file_read(fh: *mut file_handle, data: *mut c_void, len: *mut usize) -> status;
    #[link_name="switch_core_file_write"]
    pub fn core_file_write(fh: *mut file_handle, data: *mut c_void, len: *mut usize) -> status;
    #[link_name="switch_core_file_write_video"]
    pub fn core_file_write_video(fh: *mut file_handle, frame: *mut frame) -> status;
    #[link_name="switch_core_file_read_video"]
    pub fn core_file_read_video(fh: *mut file_handle,
                                frame: *mut frame,
                                flags: video_read_flag)
                                -> status;
    #[link_name="switch_core_file_seek"]
    pub fn core_file_seek(fh: *mut file_handle,
                          cur_pos: *mut c_uint,
                          samples: i64,
                          whence: c_int)
                          -> status;
    #[link_name="switch_core_file_set_string"]
    pub fn core_file_set_string(fh: *mut file_handle,
                                col: audio_col,
                                string: *const c_char)
                                -> status;
    #[link_name="switch_core_file_get_string"]
    pub fn core_file_get_string(fh: *mut file_handle,
                                col: audio_col,
                                string: *mut *const c_char)
                                -> status;
    #[link_name="switch_core_file_close"]
    pub fn core_file_close(fh: *mut file_handle) -> status;
    #[link_name="switch_core_file_command"]
    pub fn core_file_command(fh: *mut file_handle, command: file_command) -> status;
    #[link_name="switch_core_file_truncate"]
    pub fn core_file_truncate(fh: *mut file_handle, offset: i64) -> status;
    #[link_name="switch_core_file_has_video"]
    pub fn core_file_has_video(fh: *mut file_handle, CHECK_OPEN: switch_bool) -> switch_bool;
    #[link_name="switch_core_speech_open"]
    pub fn core_speech_open(sh: *mut speech_handle,
                            module_name: *const c_char,
                            voice_name: *const c_char,
                            rate: c_uint,
                            interval: c_uint,
                            channels: c_uint,
                            flags: *mut speech_flag,
                            pool: *mut memory_pool)
                            -> status;
    #[link_name="switch_core_speech_feed_tts"]
    pub fn core_speech_feed_tts(sh: *mut speech_handle,
                                text: *mut c_char,
                                flags: *mut speech_flag)
                                -> status;
    #[link_name="switch_core_speech_flush_tts"]
    pub fn core_speech_flush_tts(sh: *mut speech_handle);
    #[link_name="switch_core_speech_text_param_tts"]
    pub fn core_speech_text_param_tts(sh: *mut speech_handle,
                                      param: *mut c_char,
                                      val: *const c_char);
    #[link_name="switch_core_speech_numeric_param_tts"]
    pub fn core_speech_numeric_param_tts(sh: *mut speech_handle, param: *mut c_char, val: c_int);
    #[link_name="switch_core_speech_float_param_tts"]
    pub fn core_speech_float_param_tts(sh: *mut speech_handle, param: *mut c_char, val: f64);
    #[link_name="switch_core_speech_read_tts"]
    pub fn core_speech_read_tts(sh: *mut speech_handle,
                                data: *mut c_void,
                                datalen: *mut usize,
                                flags: *mut speech_flag)
                                -> status;
    #[link_name="switch_core_speech_close"]
    pub fn core_speech_close(sh: *mut speech_handle, flags: *mut speech_flag) -> status;
    #[link_name="switch_core_asr_open"]
    pub fn core_asr_open(ah: *mut asr_handle,
                         module_name: *const c_char,
                         codec: *const c_char,
                         rate: c_int,
                         dest: *const c_char,
                         flags: *mut asr_flag,
                         pool: *mut memory_pool)
                         -> status;
    #[link_name="switch_core_asr_close"]
    pub fn core_asr_close(ah: *mut asr_handle, flags: *mut asr_flag) -> status;
    #[link_name="switch_core_asr_feed"]
    pub fn core_asr_feed(ah: *mut asr_handle,
                         data: *mut c_void,
                         len: c_uint,
                         flags: *mut asr_flag)
                         -> status;
    #[link_name="switch_core_asr_feed_dtmf"]
    pub fn core_asr_feed_dtmf(ah: *mut asr_handle,
                              dtmf: *const dtmf,
                              flags: *mut asr_flag)
                              -> status;
    #[link_name="switch_core_asr_check_results"]
    pub fn core_asr_check_results(ah: *mut asr_handle, flags: *mut asr_flag) -> status;
    #[link_name="switch_core_asr_get_results"]
    pub fn core_asr_get_results(ah: *mut asr_handle,
                                xmlstr: *mut *mut c_char,
                                flags: *mut asr_flag)
                                -> status;
    #[link_name="switch_core_asr_get_result_headers"]
    pub fn core_asr_get_result_headers(ah: *mut asr_handle,
                                       headers: *mut *mut event,
                                       flags: *mut asr_flag)
                                       -> status;
    #[link_name="switch_core_asr_load_grammar"]
    pub fn core_asr_load_grammar(ah: *mut asr_handle,
                                 grammar: *const c_char,
                                 name: *const c_char)
                                 -> status;
    #[link_name="switch_core_asr_unload_grammar"]
    pub fn core_asr_unload_grammar(ah: *mut asr_handle, name: *const c_char) -> status;
    #[link_name="switch_core_asr_enable_grammar"]
    pub fn core_asr_enable_grammar(ah: *mut asr_handle, name: *const c_char) -> status;
    #[link_name="switch_core_asr_disable_grammar"]
    pub fn core_asr_disable_grammar(ah: *mut asr_handle, name: *const c_char) -> status;
    #[link_name="switch_core_asr_disable_all_grammars"]
    pub fn core_asr_disable_all_grammars(ah: *mut asr_handle) -> status;
    #[link_name="switch_core_asr_pause"]
    pub fn core_asr_pause(ah: *mut asr_handle) -> status;
    #[link_name="switch_core_asr_resume"]
    pub fn core_asr_resume(ah: *mut asr_handle) -> status;
    #[link_name="switch_core_asr_start_input_timers"]
    pub fn core_asr_start_input_timers(ah: *mut asr_handle) -> status;
    #[link_name="switch_core_asr_text_param"]
    pub fn core_asr_text_param(ah: *mut asr_handle, param: *mut c_char, val: *const c_char);
    #[link_name="switch_core_asr_numeric_param"]
    pub fn core_asr_numeric_param(ah: *mut asr_handle, param: *mut c_char, val: c_int);
    #[link_name="switch_core_asr_float_param"]
    pub fn core_asr_float_param(ah: *mut asr_handle, param: *mut c_char, val: f64);
    #[link_name="switch_core_directory_open"]
    pub fn core_directory_open(dh: *mut directory_handle,
                               module_name: *mut c_char,
                               source: *mut c_char,
                               dsn: *mut c_char,
                               passwd: *mut c_char,
                               pool: *mut memory_pool)
                               -> status;
    #[link_name="switch_core_directory_query"]
    pub fn core_directory_query(dh: *mut directory_handle,
                                base: *mut c_char,
                                query: *mut c_char)
                                -> status;
    #[link_name="switch_core_directory_next"]
    pub fn core_directory_next(dh: *mut directory_handle) -> status;
    #[link_name="switch_core_directory_next_pair"]
    pub fn core_directory_next_pair(dh: *mut directory_handle,
                                    var: *mut *mut c_char,
                                    val: *mut *mut c_char)
                                    -> status;
    #[link_name="switch_core_directory_close"]
    pub fn core_directory_close(dh: *mut directory_handle) -> status;
    #[link_name="switch_core_data_channel"]
    pub fn core_data_channel(channel: text_channel) -> *mut FILE;
    #[link_name="switch_core_ready"]
    pub fn core_ready() -> switch_bool;
    #[link_name="switch_core_running"]
    pub fn core_running() -> switch_bool;
    #[link_name="switch_core_ready_inbound"]
    pub fn core_ready_inbound() -> switch_bool;
    #[link_name="switch_core_ready_outbound"]
    pub fn core_ready_outbound() -> switch_bool;
    #[link_name="switch_core_flags"]
    pub fn core_flags() -> core_flag;
    #[link_name="switch_core_management_exec"]
    pub fn core_management_exec(relative_oid: *mut c_char,
                                action: management_action,
                                data: *mut c_char,
                                datalen: usize)
                                -> status;
    #[link_name="switch_core_set_process_privileges"]
    pub fn core_set_process_privileges() -> i32;
    #[link_name="switch_set_normal_priority"]
    pub fn set_normal_priority() -> i32;
    #[link_name="switch_set_auto_priority"]
    pub fn set_auto_priority() -> i32;
    #[link_name="switch_set_realtime_priority"]
    pub fn set_realtime_priority() -> i32;
    #[link_name="switch_set_low_priority"]
    pub fn set_low_priority() -> i32;
    #[link_name="switch_core_runtime_loop"]
    pub fn core_runtime_loop(bg: c_int);
    #[link_name="switch_core_set_console"]
    pub fn core_set_console(console: *const c_char) -> status;
    #[link_name="switch_core_measure_time"]
    pub fn core_measure_time(total_ms: time_t, duration: *mut core_time_duration);
    #[link_name="switch_core_uptime"]
    pub fn core_uptime() -> time_t;
    #[link_name="switch_core_session_ctl"]
    pub fn core_session_ctl(cmd: session_ctl, val: *mut c_void) -> i32;
    #[link_name="switch_core_get_console"]
    pub fn core_get_console() -> *mut FILE;
    #[link_name="switch_core_launch_thread"]
    pub fn core_launch_thread(func: Option<unsafe extern "C" fn(arg1: *mut thread,
                                                                arg2: *mut c_void)
                                                                -> *mut c_void>,
                              obj: *mut c_void,
                              pool: *mut memory_pool)
                              -> *mut thread;
    #[link_name="switch_core_set_globals"]
    pub fn core_set_globals();
    #[link_name="switch_core_session_compare"]
    pub fn core_session_compare(a: *mut core_session, b: *mut core_session) -> u8;
    #[link_name="switch_core_session_check_interface"]
    pub fn core_session_check_interface(session: *mut core_session,
                                        endpoint_interface: *const endpoint_interface)
                                        -> u8;
    #[link_name="switch_core_session_set_video_read_callback"]
    pub fn core_session_set_video_read_callback(session: *mut core_session,
                                                func: core_video_thread_callback_func,
                                                user_data: *mut c_void)
                                                -> status;
    #[link_name="switch_core_session_video_read_callback"]
    pub fn core_session_video_read_callback(session: *mut core_session,
                                            frame: *mut frame)
                                            -> status;
    #[link_name="switch_core_mime_index"]
    pub fn core_mime_index() -> *mut hash_index;
    #[link_name="switch_core_mime_ext2type"]
    pub fn core_mime_ext2type(ext: *const c_char) -> *const c_char;
    #[link_name="switch_core_mime_type2ext"]
    pub fn core_mime_type2ext(type_: *const c_char) -> *const c_char;
    #[link_name="switch_core_mime_add_type"]
    pub fn core_mime_add_type(type_: *const c_char, ext: *const c_char) -> status;
    #[link_name="switch_loadable_module_create_module_interface"]
    #[link_name="switch_loadable_module_create_module_interface"]
    pub fn loadable_module_create_module_interface(pool: *mut memory_pool,
                                                   name: *const c_char)
                                                   -> *mut loadable_module_interface;
    #[link_name="switch_loadable_module_create_interface"]
    pub fn loadable_module_create_interface(mod_: *mut loadable_module_interface,
                                            iname: module_interface_name)
                                            -> *mut c_void;
    #[link_name="switch_micro_time_now"]
    pub fn micro_time_now() -> time_t;
    #[link_name="switch_mono_micro_time_now"]
    pub fn mono_micro_time_now() -> time_t;
    #[link_name="switch_core_memory_reclaim"]
    pub fn core_memory_reclaim();
    #[link_name="switch_core_memory_reclaim_events"]
    pub fn core_memory_reclaim_events();
    #[link_name="switch_core_memory_reclaim_logger"]
    pub fn core_memory_reclaim_logger();
    #[link_name="switch_core_memory_reclaim_all"]
    pub fn core_memory_reclaim_all();
    #[link_name="switch_core_setrlimits"]
    pub fn core_setrlimits();
    #[link_name="switch_time_ref"]
    pub fn time_ref() -> time_t;
    #[link_name="switch_time_sync"]
    pub fn time_sync();
    #[link_name="switch_epoch_time_now"]
    pub fn epoch_time_now(t: *mut time_t) -> time_t;
    #[link_name="switch_lookup_timezone"]
    pub fn lookup_timezone(tz_name: *const c_char) -> *const c_char;
    #[link_name="switch_strftime_tz"]
    pub fn strftime_tz(tz: *const c_char,
                       format: *const c_char,
                       date: *mut c_char,
                       len: usize,
                       thetime: time_t)
                       -> status;
    #[link_name="switch_time_exp_tz_name"]
    pub fn time_exp_tz_name(tz: *const c_char, tm: *mut time_exp, thetime: time_t) -> status;
    #[link_name="switch_load_network_lists"]
    pub fn load_network_lists(reload: switch_bool);
    #[link_name="switch_check_network_list_ip_token"]
    pub fn check_network_list_ip_token(ip_str: *const c_char,
                                       list_name: *const c_char,
                                       token: *mut *const c_char)
                                       -> switch_bool;
    #[link_name="switch_time_set_monotonic"]
    pub fn time_set_monotonic(enable: switch_bool);
    #[link_name="switch_time_set_timerfd"]
    pub fn time_set_timerfd(enable: c_int);
    #[link_name="switch_time_set_nanosleep"]
    pub fn time_set_nanosleep(enable: switch_bool);
    #[link_name="switch_time_set_matrix"]
    pub fn time_set_matrix(enable: switch_bool);
    #[link_name="switch_time_set_cond_yield"]
    pub fn time_set_cond_yield(enable: switch_bool);
    #[link_name="switch_time_set_use_system_time"]
    pub fn time_set_use_system_time(enable: switch_bool);
    #[link_name="switch_core_min_dtmf_duration"]
    pub fn core_min_dtmf_duration(duration: u32) -> u32;
    #[link_name="switch_core_max_dtmf_duration"]
    pub fn core_max_dtmf_duration(duration: u32) -> u32;
    #[link_name="switch_core_min_idle_cpu"]
    pub fn core_min_idle_cpu(new_limit: f64) -> f64;
    #[link_name="switch_core_idle_cpu"]
    pub fn core_idle_cpu() -> f64;
    #[link_name="switch_core_default_dtmf_duration"]
    pub fn core_default_dtmf_duration(duration: u32) -> u32;
    #[link_name="switch_console_set_complete"]
    pub fn console_set_complete(string: *const c_char) -> status;
    #[link_name="switch_console_set_alias"]
    pub fn console_set_alias(string: *const c_char) -> status;
    #[link_name="switch_system"]
    pub fn system(cmd: *const c_char, wait: switch_bool) -> c_int;
    #[link_name="switch_stream_system"]
    pub fn stream_system(cmd: *const c_char, stream: *mut stream_handle) -> c_int;
    #[link_name="switch_cond_yield"]
    pub fn cond_yield(t: interval_time_t);
    #[link_name="switch_cond_next"]
    pub fn cond_next();
    #[link_name="switch_core_chat_send_args"]
    pub fn core_chat_send_args(dest_proto: *const c_char,
                               proto: *const c_char,
                               from: *const c_char,
                               to: *const c_char,
                               subject: *const c_char,
                               body: *const c_char,
                               type_: *const c_char,
                               hint: *const c_char,
                               blocking: switch_bool)
                               -> status;
    #[link_name="switch_core_chat_send"]
    pub fn core_chat_send(dest_proto: *const c_char, message_event: *mut event) -> status;
    #[link_name="switch_core_chat_deliver"]
    pub fn core_chat_deliver(dest_proto: *const c_char, message_event: *mut *mut event) -> status;
    #[link_name="switch_ivr_preprocess_session"]
    pub fn ivr_preprocess_session(session: *mut core_session, cmds: *const c_char) -> status;
    #[link_name="switch_core_sqldb_pause"]
    pub fn core_sqldb_pause();
    #[link_name="switch_core_sqldb_resume"]
    pub fn core_sqldb_resume();
    #[link_name="switch_cache_db_get_type"]
    pub fn cache_db_get_type(dbh: *mut cache_db_handle) -> cache_db_handle_type;
    #[link_name="switch_cache_db_dismiss_db_handle"]
    pub fn cache_db_dismiss_db_handle(dbh: *mut *mut cache_db_handle);
    #[link_name="switch_cache_db_release_db_handle"]
    pub fn cache_db_release_db_handle(dbh: *mut *mut cache_db_handle);
    #[link_name="_switch_cache_db_get_db_handle"]
    pub fn _cache_db_get_db_handle(dbh: *mut *mut cache_db_handle,
                                   type_: cache_db_handle_type,
                                   connection_options: *mut cache_db_connection_options,
                                   file: *const c_char,
                                   func: *const c_char,
                                   line: c_int)
                                   -> status;
    #[link_name="_switch_cache_db_get_db_handle_dsn"]
    pub fn _cache_db_get_db_handle_dsn(dbh: *mut *mut cache_db_handle,
                                       dsn: *const c_char,
                                       file: *const c_char,
                                       func: *const c_char,
                                       line: c_int)
                                       -> status;
    #[link_name="switch_cache_db_create_schema"]
    pub fn cache_db_create_schema(dbh: *mut cache_db_handle,
                                  sql: *mut c_char,
                                  err: *mut *mut c_char)
                                  -> status;
    #[link_name="switch_cache_db_execute_sql2str"]
    pub fn cache_db_execute_sql2str(dbh: *mut cache_db_handle,
                                    sql: *mut c_char,
                                    str: *mut c_char,
                                    len: usize,
                                    err: *mut *mut c_char)
                                    -> *mut c_char;
    #[link_name="switch_cache_db_execute_sql"]
    pub fn cache_db_execute_sql(dbh: *mut cache_db_handle,
                                sql: *mut c_char,
                                err: *mut *mut c_char)
                                -> status;
    #[link_name="switch_cache_db_execute_sql_callback"]
    pub fn cache_db_execute_sql_callback(dbh: *mut cache_db_handle,
                                         sql: *const c_char,
                                         callback: core_db_callback_func,
                                         pdata: *mut c_void,
                                         err: *mut *mut c_char)
                                         -> status;
    #[link_name="switch_cache_db_execute_sql_callback_err"]
    pub fn cache_db_execute_sql_callback_err(dbh: *mut cache_db_handle,
                                             sql: *const c_char,
                                             callback: core_db_callback_func,
                                             err_callback: core_db_err_callback_func,
                                             pdata: *mut c_void,
                                             err: *mut *mut c_char)
                                             -> status;
    #[link_name="switch_cache_db_affected_rows"]
    pub fn cache_db_affected_rows(dbh: *mut cache_db_handle) -> c_int;
    #[link_name="switch_cache_db_load_extension"]
    pub fn cache_db_load_extension(dbh: *mut cache_db_handle, extension: *const c_char) -> c_int;
    #[link_name="switch_cache_db_status"]
    pub fn cache_db_status(stream: *mut stream_handle);
    #[link_name="_switch_core_db_handle"]
    pub fn _core_db_handle(dbh: *mut *mut cache_db_handle,
                           file: *const c_char,
                           func: *const c_char,
                           line: c_int)
                           -> status;
    #[link_name="switch_cache_db_test_reactive"]
    pub fn cache_db_test_reactive(db: *mut cache_db_handle,
                                  test_sql: *const c_char,
                                  drop_sql: *const c_char,
                                  reactive_sql: *const c_char)
                                  -> switch_bool;
    #[link_name="switch_cache_db_persistant_execute"]
    pub fn cache_db_persistant_execute(dbh: *mut cache_db_handle,
                                       sql: *const c_char,
                                       retries: u32)
                                       -> status;
    #[link_name="switch_cache_db_persistant_execute_trans_full"]
    pub fn cache_db_persistant_execute_trans_full(dbh: *mut cache_db_handle,
                                                  sql: *mut c_char,
                                                  retries: u32,
                                                  pre_trans_execute: *const c_char,
                                                  post_trans_execute: *const c_char,
                                                  inner_pre_trans_execute: *const c_char,
                                                  inner_post_trans_execute: *const c_char)
                                                  -> status;
    #[link_name="switch_core_set_signal_handlers"]
    pub fn core_set_signal_handlers();
    #[link_name="switch_core_debug_level"]
    pub fn core_debug_level() -> u32;
    #[link_name="switch_cache_db_flush_handles"]
    pub fn cache_db_flush_handles();
    #[link_name="switch_core_banner"]
    pub fn core_banner() -> *const c_char;
    #[link_name="switch_core_session_in_thread"]
    pub fn core_session_in_thread(session: *mut core_session) -> switch_bool;
    #[link_name="switch_default_ptime"]
    pub fn default_ptime(name: *const c_char, number: u32) -> u32;
    #[link_name="switch_default_rate"]
    pub fn default_rate(name: *const c_char, number: u32) -> u32;
    #[link_name="switch_core_add_registration"]
    pub fn core_add_registration(user: *const c_char,
                                 realm: *const c_char,
                                 token: *const c_char,
                                 url: *const c_char,
                                 expires: u32,
                                 network_ip: *const c_char,
                                 network_port: *const c_char,
                                 network_proto: *const c_char,
                                 metadata: *const c_char)
                                 -> status;
    #[link_name="switch_core_del_registration"]
    pub fn core_del_registration(user: *const c_char,
                                 realm: *const c_char,
                                 token: *const c_char)
                                 -> status;
    #[link_name="switch_core_expire_registration"]
    pub fn core_expire_registration(force: c_int) -> status;
    #[link_name="switch_core_get_rtp_port_range_start_port"]
    pub fn core_get_rtp_port_range_start_port() -> u16;
    #[link_name="switch_core_get_rtp_port_range_end_port"]
    pub fn core_get_rtp_port_range_end_port() -> u16;
    #[link_name="switch_say_file_handle_get_variable"]
    pub fn say_file_handle_get_variable(sh: *mut say_file_handle,
                                        var: *const c_char)
                                        -> *mut c_char;
    #[link_name="switch_say_file_handle_get_path"]
    pub fn say_file_handle_get_path(sh: *mut say_file_handle) -> *mut c_char;
    #[link_name="switch_say_file_handle_detach_path"]
    pub fn say_file_handle_detach_path(sh: *mut say_file_handle) -> *mut c_char;
    #[link_name="switch_say_file_handle_destroy"]
    pub fn say_file_handle_destroy(sh: *mut *mut say_file_handle);
    #[link_name="switch_say_file_handle_create"]
    pub fn say_file_handle_create(sh: *mut *mut say_file_handle,
                                  ext: *const c_char,
                                  var_event: *mut *mut event)
                                  -> status;
    #[link_name="switch_say_file"]
    pub fn say_file(sh: *mut say_file_handle, fmt: *const c_char, ...);
    #[link_name="switch_max_file_desc"]
    pub fn max_file_desc() -> c_int;
    #[link_name="switch_close_extra_files"]
    pub fn close_extra_files(keep: *mut c_int, keep_ttl: c_int);
    #[link_name="switch_core_thread_set_cpu_affinity"]
    pub fn core_thread_set_cpu_affinity(cpu: c_int) -> status;
    #[link_name="switch_os_yield"]
    pub fn os_yield();
    #[link_name="switch_core_get_stacksizes"]
    pub fn core_get_stacksizes(cur: *mut usize, max: *mut usize) -> status;
    #[link_name="switch_core_gen_encoded_silence"]
    pub fn core_gen_encoded_silence(data: *mut c_uchar,
                                    read_impl: *const codec_implementation,
                                    len: usize);
    #[link_name="switch_core_dbtype"]
    pub fn core_dbtype() -> cache_db_handle_type;
    #[link_name="switch_core_sql_exec"]
    pub fn core_sql_exec(sql: *const c_char);
    #[link_name="switch_core_recovery_recover"]
    pub fn core_recovery_recover(technology: *const c_char, profile_name: *const c_char) -> c_int;
    #[link_name="switch_core_recovery_untrack"]
    pub fn core_recovery_untrack(session: *mut core_session, force: switch_bool);
    #[link_name="switch_core_recovery_track"]
    pub fn core_recovery_track(session: *mut core_session);
    #[link_name="switch_core_recovery_flush"]
    pub fn core_recovery_flush(technology: *const c_char, profile_name: *const c_char);
    #[link_name="switch_sql_queue_manager_pause"]
    pub fn sql_queue_manager_pause(qm: *mut sql_queue_manager, flush: switch_bool);
    #[link_name="switch_sql_queue_manager_resume"]
    pub fn sql_queue_manager_resume(qm: *mut sql_queue_manager);
    #[link_name="switch_sql_queue_manager_size"]
    pub fn sql_queue_manager_size(qm: *mut sql_queue_manager, index: u32) -> c_int;
    #[link_name="switch_sql_queue_manager_push_confirm"]
    pub fn sql_queue_manager_push_confirm(qm: *mut sql_queue_manager,
                                          sql: *const c_char,
                                          pos: u32,
                                          dup: switch_bool)
                                          -> status;
    #[link_name="switch_sql_queue_manager_push"]
    pub fn sql_queue_manager_push(qm: *mut sql_queue_manager,
                                  sql: *const c_char,
                                  pos: u32,
                                  dup: switch_bool)
                                  -> status;
    #[link_name="switch_sql_queue_manager_destroy"]
    pub fn sql_queue_manager_destroy(qmp: *mut *mut sql_queue_manager) -> status;
    #[link_name="switch_sql_queue_manager_init_name"]
    pub fn sql_queue_manager_init_name(name: *const c_char,
                                       qmp: *mut *mut sql_queue_manager,
                                       numq: u32,
                                       dsn: *const c_char,
                                       max_trans: u32,
                                       pre_trans_execute: *const c_char,
                                       post_trans_execute: *const c_char,
                                       inner_pre_trans_execute: *const c_char,
                                       inner_post_trans_execute: *const c_char)
                                       -> status;
    #[link_name="switch_sql_queue_manager_start"]
    pub fn sql_queue_manager_start(qm: *mut sql_queue_manager) -> status;
    #[link_name="switch_sql_queue_manager_stop"]
    pub fn sql_queue_manager_stop(qm: *mut sql_queue_manager) -> status;
    #[link_name="switch_cache_db_execute_sql_event_callback"]
    pub fn cache_db_execute_sql_event_callback(dbh: *mut cache_db_handle,
                                               sql: *const c_char,
                                               callback: core_db_event_callback_func,
                                               pdata: *mut c_void,
                                               err: *mut *mut c_char)
                                               -> status;
    #[link_name="switch_sql_queue_manager_execute_sql_callback"]
    pub fn sql_queue_manager_execute_sql_callback(qm: *mut sql_queue_manager,
                                                  sql: *const c_char,
                                                  callback: core_db_callback_func,
                                                  pdata: *mut c_void);
    #[link_name="switch_sql_queue_manager_execute_sql_callback_err"]
    pub fn sql_queue_manager_execute_sql_callback_err(qm: *mut sql_queue_manager,
                                                      sql: *const c_char,
                                                      callback: core_db_callback_func,
                                                      err_callback: core_db_err_callback_func,
                                                      pdata: *mut c_void);
    #[link_name="switch_sql_queue_manager_execute_sql_event_callback"]
    pub fn sql_queue_manager_execute_sql_event_callback(qm: *mut sql_queue_manager,
                                                        sql: *const c_char,
                                                        callback: core_db_event_callback_func,
                                                        pdata: *mut c_void);
    #[link_name="switch_sql_queue_manager_execute_sql_event_callback_err"]
    pub fn sql_queue_manager_execute_sql_event_callback_err(qm: *mut sql_queue_manager,
                                                            sql: *const c_char,
                                                            callback: core_db_event_callback_func,
                                                            err_callback: core_db_err_callback_func,
                                                            pdata: *mut c_void);
    #[link_name="switch_core_gen_certs"]
    pub fn core_gen_certs(prefix: *const c_char) -> c_int;
    #[link_name="switch_core_cert_gen_fingerprint"]
    pub fn core_cert_gen_fingerprint(prefix: *const c_char, fp: *mut dtls_fingerprint) -> c_int;
    #[link_name="switch_core_cert_expand_fingerprint"]
    pub fn core_cert_expand_fingerprint(fp: *mut dtls_fingerprint, str: *const c_char) -> c_int;
    #[link_name="switch_core_cert_verify"]
    pub fn core_cert_verify(fp: *mut dtls_fingerprint) -> c_int;
    #[link_name="switch_core_session_request_video_refresh"]
    pub fn core_session_request_video_refresh(session: *mut core_session) -> status;
    #[link_name="switch_core_session_send_and_request_video_refresh"]
    pub fn core_session_send_and_request_video_refresh(session: *mut core_session) -> status;
    #[link_name="switch_stream_system_fork"]
    pub fn stream_system_fork(cmd: *const c_char, stream: *mut stream_handle) -> c_int;
    #[link_name="switch_ice_direction"]
    pub fn ice_direction(session: *mut core_session) -> call_direction;
    #[link_name="switch_core_session_debug_pool"]
    pub fn core_session_debug_pool(stream: *mut stream_handle);
    #[link_name="switch_version_major"]
    pub fn version_major() -> *const c_char;
    #[link_name="switch_version_minor"]
    pub fn version_minor() -> *const c_char;
    #[link_name="switch_version_micro"]
    pub fn version_micro() -> *const c_char;
    #[link_name="switch_version_revision"]
    pub fn version_revision() -> *const c_char;
    #[link_name="switch_version_revision_human"]
    pub fn version_revision_human() -> *const c_char;
    #[link_name="switch_version_full"]
    pub fn version_full() -> *const c_char;
    #[link_name="switch_version_full_human"]
    pub fn version_full_human() -> *const c_char;
    #[link_name="switch_core_autobind_cpu"]
    pub fn core_autobind_cpu();
    #[link_name="switch_log_init"]
    pub fn log_init(pool: *mut memory_pool, colorize: switch_bool) -> status;
    #[link_name="switch_log_shutdown"]
    pub fn log_shutdown() -> status;
    #[link_name="switch_log_printf"]
    pub fn log_printf(channel: text_channel,
                      file: *const c_char,
                      func: *const c_char,
                      line: c_int,
                      userdata: *const c_char,
                      level: log_level,
                      fmt: *const c_char,
                      ...);
    #[link_name="switch_log_bind_logger"]
    pub fn log_bind_logger(function: log_function,
                           level: log_level,
                           is_console: switch_bool)
                           -> status;
    #[link_name="switch_log_unbind_logger"]
    pub fn log_unbind_logger(function: log_function) -> status;
    #[link_name="switch_log_level2str"]
    pub fn log_level2str(level: log_level) -> *const c_char;
    #[link_name="switch_log_str2level"]
    pub fn log_str2level(str: *const c_char) -> log_level;
    #[link_name="switch_log_str2mask"]
    pub fn log_str2mask(str: *const c_char) -> u32;
    #[link_name="switch_log_node_dup"]
    pub fn log_node_dup(node: *const log_node) -> *mut log_node;
    #[link_name="switch_log_node_free"]
    pub fn log_node_free(pnode: *mut *mut log_node);
    #[link_name="switch_resample_perform_create"]
    pub fn resample_perform_create(new_resampler: *mut *mut audio_resampler,
                                   from_rate: u32,
                                   to_rate: u32,
                                   to_size: u32,
                                   quality: c_int,
                                   channels: u32,
                                   file: *const c_char,
                                   func: *const c_char,
                                   line: c_int)
                                   -> status;
    #[link_name="switch_resample_destroy"]
    pub fn resample_destroy(resampler: *mut *mut audio_resampler);
    #[link_name="switch_resample_process"]
    pub fn resample_process(resampler: *mut audio_resampler, src: *mut i16, srclen: u32) -> u32;
    #[link_name="switch_float_to_short"]
    pub fn float_to_short(f: *mut f32, s: *mut c_short, len: usize) -> usize;
    #[link_name="switch_char_to_float"]
    pub fn char_to_float(c: *mut c_char, f: *mut f32, len: c_int) -> c_int;
    #[link_name="switch_float_to_char"]
    pub fn float_to_char(f: *mut f32, c: *mut c_char, len: c_int) -> c_int;
    #[link_name="switch_short_to_float"]
    pub fn short_to_float(s: *mut c_short, f: *mut f32, len: c_int) -> c_int;
    #[link_name="switch_swap_linear"]
    pub fn swap_linear(buf: *mut i16, len: c_int);
    #[link_name="switch_generate_sln_silence"]
    pub fn generate_sln_silence(data: *mut i16, samples: u32, channels: u32, divisor: u32);
    #[link_name="switch_change_sln_volume"]
    pub fn change_sln_volume(data: *mut i16, samples: u32, vol: i32);
    #[link_name="switch_change_sln_volume_granular"]
    pub fn change_sln_volume_granular(data: *mut i16, samples: u32, vol: i32);
    #[link_name="switch_merge_sln"]
    pub fn merge_sln(data: *mut i16,
                     samples: u32,
                     other_data: *mut i16,
                     other_samples: u32,
                     channels: c_int)
                     -> u32;
    #[link_name="switch_unmerge_sln"]
    pub fn unmerge_sln(data: *mut i16,
                       samples: u32,
                       other_data: *mut i16,
                       other_samples: u32,
                       channels: c_int)
                       -> u32;
    #[link_name="switch_mux_channels"]
    pub fn mux_channels(data: *mut i16, samples: usize, orig_channels: u32, channels: u32);
    #[link_name="switch_loadable_module_init"]
    pub fn loadable_module_init(autoload: switch_bool) -> status;
    #[link_name="switch_loadable_module_shutdown"]
    pub fn loadable_module_shutdown();
    #[link_name="switch_loadable_module_get_endpoint_interface"]
    pub fn loadable_module_get_endpoint_interface(name: *const c_char) -> *mut endpoint_interface;
    #[link_name="switch_loadable_module_get_codec_interface"]
    pub fn loadable_module_get_codec_interface(name: *const c_char,
                                               modname: *const c_char)
                                               -> *mut codec_interface;
    #[link_name="switch_parse_codec_buf"]
    pub fn parse_codec_buf(buf: *mut c_char,
                           interval: *mut u32,
                           rate: *mut u32,
                           bit: *mut u32,
                           channels: *mut u32,
                           modname: *mut *mut c_char,
                           fmtp: *mut *mut c_char)
                           -> *mut c_char;
    #[link_name="switch_loadable_module_get_dialplan_interface"]
    pub fn loadable_module_get_dialplan_interface(name: *const c_char) -> *mut dialplan_interface;
    #[link_name="switch_loadable_module_enumerate_available"]
    pub fn loadable_module_enumerate_available(dir_path: *const c_char,
                                               callback: modulename_callback_func,
                                               user_data: *mut c_void)
                                               -> status;
    #[link_name="switch_loadable_module_enumerate_loaded"]
    pub fn loadable_module_enumerate_loaded(callback: modulename_callback_func,
                                            user_data: *mut c_void)
                                            -> status;
    #[link_name="switch_loadable_module_build_dynamic"]
    pub fn loadable_module_build_dynamic(filename: *mut c_char,
                                         module_load: module_load,
                                         module_runtime: module_runtime,
                                         module_shutdown: module_shutdown,
                                         runtime: switch_bool)
                                         -> status;
    #[link_name="switch_loadable_module_get_timer_interface"]
    pub fn loadable_module_get_timer_interface(name: *const c_char) -> *mut timer_interface;
    #[link_name="switch_loadable_module_get_application_interface"]
    pub fn loadable_module_get_application_interface(name: *const c_char)
                                                     -> *mut application_interface;
    #[link_name="switch_loadable_module_get_chat_application_interface"]
    pub fn loadable_module_get_chat_application_interface(name: *const c_char)
                                                          -> *mut chat_application_interface;
    #[link_name="switch_core_execute_chat_app"]
    pub fn core_execute_chat_app(message: *mut event,
                                 app: *const c_char,
                                 data: *const c_char)
                                 -> status;
    #[link_name="switch_loadable_module_get_api_interface"]
    pub fn loadable_module_get_api_interface(name: *const c_char) -> *mut api_interface;
    #[link_name="switch_loadable_module_get_json_api_interface"]
    pub fn loadable_module_get_json_api_interface(name: *const c_char) -> *mut json_api_interface;
    #[link_name="switch_loadable_module_get_file_interface"]
    pub fn loadable_module_get_file_interface(name: *const c_char,
                                              modname: *const c_char)
                                              -> *mut file_interface;
    #[link_name="switch_loadable_module_get_speech_interface"]
    pub fn loadable_module_get_speech_interface(name: *const c_char) -> *mut speech_interface;
    #[link_name="switch_loadable_module_get_asr_interface"]
    pub fn loadable_module_get_asr_interface(name: *const c_char) -> *mut asr_interface;
    #[link_name="switch_loadable_module_get_directory_interface"]
    pub fn loadable_module_get_directory_interface(name: *const c_char) -> *mut directory_interface;
    #[link_name="switch_loadable_module_get_chat_interface"]
    pub fn loadable_module_get_chat_interface(name: *const c_char) -> *mut chat_interface;
    #[link_name="switch_loadable_module_get_say_interface"]
    pub fn loadable_module_get_say_interface(name: *const c_char) -> *mut say_interface;
    #[link_name="switch_loadable_module_get_management_interface"]
    pub fn loadable_module_get_management_interface(relative_oid: *const c_char)
                                                    -> *mut management_interface;
    #[link_name="switch_loadable_module_get_limit_interface"]
    pub fn loadable_module_get_limit_interface(name: *const c_char) -> *mut limit_interface;
    #[link_name="switch_loadable_module_get_codecs"]
    pub fn loadable_module_get_codecs(array: *mut *const codec_implementation,
                                      arraylen: c_int)
                                      -> c_int;
    #[link_name="switch_loadable_module_get_codecs_sorted"]
    pub fn loadable_module_get_codecs_sorted(array: *mut *const codec_implementation,
                                             fmtp_array: *mut [c_char; 256usize],
                                             arraylen: c_int,
                                             prefs: *mut *mut c_char,
                                             preflen: c_int)
                                             -> c_int;
    #[link_name="switch_api_execute"]
    pub fn api_execute(cmd: *const c_char,
                       arg: *const c_char,
                       session: *mut core_session,
                       stream: *mut stream_handle)
                       -> status;
    #[link_name="switch_json_api_execute"]
    pub fn json_api_execute(json: *mut cJSON,
                            session: *mut core_session,
                            retval: *mut *mut cJSON)
                            -> status;
    #[link_name="switch_loadable_module_load_module"]
    pub fn loadable_module_load_module(dir: *mut c_char,
                                       fname: *mut c_char,
                                       runtime: switch_bool,
                                       err: *mut *const c_char)
                                       -> status;
    #[link_name="switch_loadable_module_exists"]
    pub fn loadable_module_exists(mod_: *const c_char) -> status;
    #[link_name="switch_loadable_module_unload_module"]
    pub fn loadable_module_unload_module(dir: *mut c_char,
                                         fname: *mut c_char,
                                         force: switch_bool,
                                         err: *mut *const c_char)
                                         -> status;
    #[link_name="switch_module_load"]
    pub fn module_load(module_interface: *mut *mut loadable_module_interface,
                       filename: *mut c_char)
                       -> status;
    #[link_name="switch_module_runtime"]
    pub fn module_runtime() -> status;
    #[link_name="switch_module_shutdown"]
    pub fn module_shutdown() -> status;
    #[link_name="switch_core_codec_next_id"]
    pub fn core_codec_next_id() -> u32;
    #[link_name="switch_core_get_secondary_recover_callback"]
    pub fn core_get_secondary_recover_callback(key: *const c_char) -> core_recover_callback;
    #[link_name="switch_core_register_secondary_recover_callback"]
    pub fn core_register_secondary_recover_callback(key: *const c_char,
                                                    cb: core_recover_callback)
                                                    -> status;
    #[link_name="switch_core_unregister_secondary_recover_callback"]
    pub fn core_unregister_secondary_recover_callback(key: *const c_char);
    #[link_name="switch_console_loop"]
    pub fn console_loop();
    #[link_name="switch_console_printf"]
    pub fn console_printf(channel: text_channel,
                          file: *const c_char,
                          func: *const c_char,
                          line: c_int,
                          fmt: *const c_char,
                          ...);
    #[link_name="switch_console_stream_raw_write"]
    pub fn console_stream_raw_write(handle: *mut stream_handle,
                                    data: *mut u8,
                                    datalen: usize)
                                    -> status;
    #[link_name="switch_console_stream_write"]
    pub fn console_stream_write(handle: *mut stream_handle, fmt: *const c_char, ...) -> status;
    #[link_name="switch_stream_write_file_contents"]
    pub fn stream_write_file_contents(stream: *mut stream_handle, path: *const c_char) -> status;
    #[link_name="switch_console_init"]
    pub fn console_init(pool: *mut memory_pool) -> status;
    #[link_name="switch_console_shutdown"]
    pub fn console_shutdown() -> status;
    #[link_name="switch_console_add_complete_func"]
    pub fn console_add_complete_func(name: *const c_char, cb: console_complete_callback) -> status;
    #[link_name="switch_console_del_complete_func"]
    pub fn console_del_complete_func(name: *const c_char) -> status;
    #[link_name="switch_console_run_complete_func"]
    pub fn console_run_complete_func(func: *const c_char,
                                     line: *const c_char,
                                     last_word: *const c_char,
                                     matches: *mut *mut console_callback_match)
                                     -> status;
    #[link_name="switch_console_push_match_unique"]
    pub fn console_push_match_unique(matches: *mut *mut console_callback_match,
                                     new_val: *const c_char);
    #[link_name="switch_console_push_match"]
    pub fn console_push_match(matches: *mut *mut console_callback_match, new_val: *const c_char);
    #[link_name="switch_console_free_matches"]
    pub fn console_free_matches(matches: *mut *mut console_callback_match);
    #[link_name="switch_console_complete"]
    pub fn console_complete(line: *const c_char,
                            last_word: *const c_char,
                            console_out: *mut FILE,
                            stream: *mut stream_handle,
                            xml: xml_t)
                            -> c_uchar;
    #[link_name="switch_console_sort_matches"]
    pub fn console_sort_matches(matches: *mut console_callback_match);
    #[link_name="switch_console_save_history"]
    pub fn console_save_history();
    #[link_name="switch_console_expand_alias"]
    pub fn console_expand_alias(cmd: *mut c_char, arg: *mut c_char) -> *mut c_char;
    #[link_name="switch_console_execute"]
    pub fn console_execute(xcmd: *mut c_char, rec: c_int, istream: *mut stream_handle) -> status;
    #[link_name="switch_testv6_subnet"]
    pub fn testv6_subnet(_ip: ip, _net: ip, _mask: ip) -> switch_bool;
    #[link_name="switch_print_host"]
    pub fn print_host(addr: *mut sockaddr, buf: *mut c_char, len: isize) -> *mut c_char;
    #[link_name="switch_b64_encode"]
    pub fn b64_encode(in_: *mut c_uchar, ilen: isize, out: *mut c_uchar, olen: isize) -> status;
    #[link_name="switch_b64_decode"]
    pub fn b64_decode(in_: *mut c_char, out: *mut c_char, olen: isize) -> isize;
    #[link_name="switch_amp_encode"]
    pub fn amp_encode(s: *mut c_char, buf: *mut c_char, len: isize) -> *mut c_char;
    #[link_name="switch_fd_read_line"]
    pub fn fd_read_line(fd: c_int, buf: *mut c_char, len: isize) -> isize;
    #[link_name="switch_fd_read_dline"]
    pub fn fd_read_dline(fd: c_int, buf: *mut *mut c_char, len: *mut isize) -> isize;
    #[link_name="switch_fp_read_dline"]
    pub fn fp_read_dline(fd: *mut FILE, buf: *mut *mut c_char, len: *mut isize) -> isize;
    #[link_name="switch_frame_alloc"]
    pub fn frame_alloc(frame: *mut *mut frame, size: usize) -> status;
    #[link_name="switch_frame_dup"]
    pub fn frame_dup(orig: *mut frame, clone: *mut *mut frame) -> status;
    #[link_name="switch_frame_free"]
    pub fn frame_free(frame: *mut *mut frame) -> status;
    #[link_name="switch_is_number"]
    pub fn is_number(str: *const c_char) -> switch_bool;
    #[link_name="switch_is_leading_number"]
    pub fn is_leading_number(str: *const c_char) -> switch_bool;
    #[link_name="switch_find_parameter"]
    pub fn find_parameter(str: *const c_char,
                          param: *const c_char,
                          pool: *mut memory_pool)
                          -> *mut c_char;
    #[link_name="switch_resolve_host"]
    pub fn resolve_host(host: *const c_char, buf: *mut c_char, buflen: usize) -> status;
    #[link_name="switch_find_local_ip"]
    pub fn find_local_ip(buf: *mut c_char, len: c_int, mask: *mut c_int, family: c_int) -> status;
    #[link_name="switch_find_interface_ip"]
    pub fn find_interface_ip(buf: *mut c_char,
                             len: c_int,
                             mask: *mut c_int,
                             ifname: *const c_char,
                             family: c_int)
                             -> status;
    #[link_name="switch_cmp_addr"]
    pub fn cmp_addr(sa1: *mut sockaddr, sa2: *mut sockaddr) -> c_int;
    #[link_name="switch_cp_addr"]
    pub fn cp_addr(sa1: *mut sockaddr, sa2: *mut sockaddr) -> c_int;
    #[link_name="switch_build_uri"]
    pub fn build_uri(uri: *mut c_char,
                     size: usize,
                     scheme: *const c_char,
                     user: *const c_char,
                     sa: *const sockaddr,
                     flags: c_int)
                     -> c_int;
    #[link_name="switch_priority_name"]
    pub fn priority_name(priority: priority) -> *const c_char;
    #[link_name="switch_rfc2833_to_char"]
    pub fn rfc2833_to_char(event: c_int) -> c_char;
    #[link_name="switch_char_to_rfc2833"]
    pub fn char_to_rfc2833(key: c_char) -> c_uchar;
    #[link_name="switch_str_time"]
    pub fn str_time(in_: *const c_char) -> time_t;
    #[link_name="switch_separate_string"]
    pub fn separate_string(buf: *mut c_char,
                           delim: c_char,
                           array: *mut *mut c_char,
                           arraylen: c_uint)
                           -> c_uint;
    #[link_name="switch_separate_string_string"]
    pub fn separate_string_string(buf: *mut c_char,
                                  delim: *mut c_char,
                                  array: *mut *mut c_char,
                                  arraylen: c_uint)
                                  -> c_uint;
    #[link_name="switch_strip_spaces"]
    pub fn strip_spaces(str: *mut c_char, dup: switch_bool) -> *mut c_char;
    #[link_name="switch_strip_whitespace"]
    pub fn strip_whitespace(str: *const c_char) -> *mut c_char;
    #[link_name="switch_strip_commas"]
    pub fn strip_commas(in_: *mut c_char, out: *mut c_char, len: usize) -> *mut c_char;
    #[link_name="switch_strip_nonnumerics"]
    pub fn strip_nonnumerics(in_: *mut c_char, out: *mut c_char, len: usize) -> *mut c_char;
    #[link_name="switch_separate_paren_args"]
    pub fn separate_paren_args(str: *mut c_char) -> *mut c_char;
    #[link_name="switch_stristr"]
    pub fn stristr(instr: *const c_char, str: *const c_char) -> *const c_char;
    #[link_name="switch_is_lan_addr"]
    pub fn is_lan_addr(ip: *const c_char) -> switch_bool;
    #[link_name="switch_replace_char"]
    pub fn replace_char(str: *mut c_char,
                        from: c_char,
                        to: c_char,
                        dup: switch_bool)
                        -> *mut c_char;
    #[link_name="switch_ast2regex"]
    pub fn ast2regex(pat: *const c_char, rbuf: *mut c_char, len: usize) -> switch_bool;
    #[link_name="switch_escape_char"]
    pub fn escape_char(pool: *mut memory_pool,
                       in_: *mut c_char,
                       delim: *const c_char,
                       esc: c_char)
                       -> *mut c_char;
    #[link_name="switch_escape_string"]
    pub fn escape_string(in_: *const c_char, out: *mut c_char, outlen: usize) -> *mut c_char;
    #[link_name="switch_escape_string_pool"]
    pub fn escape_string_pool(in_: *const c_char, pool: *mut memory_pool) -> *mut c_char;
    #[link_name="switch_socket_waitfor"]
    pub fn socket_waitfor(poll: *mut pollfd, ms: c_int) -> c_int;
    #[link_name="switch_cut_path"]
    pub fn cut_path(in_: *const c_char) -> *const c_char;
    #[link_name="switch_string_replace"]
    pub fn string_replace(string: *const c_char,
                          search: *const c_char,
                          replace: *const c_char)
                          -> *mut c_char;
    #[link_name="switch_string_match"]
    pub fn string_match(string: *const c_char,
                        string_len: usize,
                        search: *const c_char,
                        search_len: usize)
                        -> status;
    #[link_name="switch_strcasecmp_any"]
    pub fn strcasecmp_any(str: *const c_char, ...) -> c_int;
    #[link_name="switch_util_quote_shell_arg"]
    pub fn util_quote_shell_arg(string: *const c_char) -> *mut c_char;
    #[link_name="switch_util_quote_shell_arg_pool"]
    pub fn util_quote_shell_arg_pool(string: *const c_char, pool: *mut memory_pool) -> *mut c_char;
    #[link_name="switch_url_encode_opt"]
    pub fn url_encode_opt(url: *const c_char,
                          buf: *mut c_char,
                          len: usize,
                          double_encode: switch_bool)
                          -> *mut c_char;
    #[link_name="switch_url_encode"]
    pub fn url_encode(url: *const c_char, buf: *mut c_char, len: usize) -> *mut c_char;
    #[link_name="switch_url_decode"]
    pub fn url_decode(s: *mut c_char) -> *mut c_char;
    #[link_name="switch_simple_email"]
    pub fn simple_email(to: *const c_char,
                        from: *const c_char,
                        headers: *const c_char,
                        body: *const c_char,
                        file: *const c_char,
                        convert_cmd: *const c_char,
                        convert_ext: *const c_char)
                        -> switch_bool;
    #[link_name="switch_find_end_paren"]
    pub fn find_end_paren(s: *const c_char, open: c_char, close: c_char) -> *mut c_char;
    #[link_name="switch_parse_cidr"]
    pub fn parse_cidr(string: *const c_char, ip: *mut ip, mask: *mut ip, bitp: *mut u32) -> c_int;
    #[link_name="switch_network_list_create"]
    pub fn network_list_create(list: *mut *mut network_list,
                               name: *const c_char,
                               default_type: switch_bool,
                               pool: *mut memory_pool)
                               -> status;
    #[link_name="switch_network_list_add_cidr_token"]
    pub fn network_list_add_cidr_token(list: *mut network_list,
                                       cidr_str: *const c_char,
                                       ok: switch_bool,
                                       token: *const c_char)
                                       -> status;
    #[link_name="switch_network_ipv4_mapped_ipv6_addr"]
    pub fn network_ipv4_mapped_ipv6_addr(ip_str: *const c_char) -> *mut c_char;
    #[link_name="switch_network_list_add_host_mask"]
    pub fn network_list_add_host_mask(list: *mut network_list,
                                      host: *const c_char,
                                      mask_str: *const c_char,
                                      ok: switch_bool)
                                      -> status;
    #[link_name="switch_network_list_validate_ip_token"]
    pub fn network_list_validate_ip_token(list: *mut network_list,
                                          ip: u32,
                                          token: *mut *const c_char)
                                          -> switch_bool;
    #[link_name="switch_network_list_validate_ip6_token"]
    pub fn network_list_validate_ip6_token(list: *mut network_list,
                                           ip: ip,
                                           token: *mut *const c_char)
                                           -> switch_bool;
    #[link_name="switch_inet_pton"]
    pub fn inet_pton(af: c_int, src: *const c_char, dst: *mut c_void) -> c_int;
    #[link_name="switch_dow_int2str"]
    pub fn dow_int2str(val: c_int) -> *const c_char;
    #[link_name="switch_dow_str2int"]
    pub fn dow_str2int(exp: *const c_char) -> c_int;
    #[link_name="switch_dow_cmp"]
    pub fn dow_cmp(exp: *const c_char, val: c_int) -> switch_bool;
    #[link_name="switch_number_cmp"]
    pub fn number_cmp(exp: *const c_char, val: c_int) -> c_int;
    #[link_name="switch_tod_cmp"]
    pub fn tod_cmp(exp: *const c_char, val: c_int) -> c_int;
    #[link_name="switch_fulldate_cmp"]
    pub fn fulldate_cmp(exp: *const c_char, ts: *mut time_t) -> c_int;
    #[link_name="switch_split_date"]
    pub fn split_date(exp: *const c_char, year: *mut c_int, month: *mut c_int, day: *mut c_int);
    #[link_name="switch_split_time"]
    pub fn split_time(exp: *const c_char, hour: *mut c_int, min: *mut c_int, sec: *mut c_int);
    #[link_name="switch_split_user_domain"]
    pub fn split_user_domain(in_: *mut c_char,
                             user: *mut *mut c_char,
                             domain: *mut *mut c_char)
                             -> c_int;
    #[link_name="switch_uuid_str"]
    pub fn uuid_str(buf: *mut c_char, len: usize) -> *mut c_char;
    #[link_name="switch_format_number"]
    pub fn format_number(num: *const c_char) -> *mut c_char;
    #[link_name="switch_atoui"]
    pub fn atoui(nptr: *const c_char) -> c_uint;
    #[link_name="switch_atoul"]
    pub fn atoul(nptr: *const c_char) -> c_ulong;
    #[link_name="switch_strerror_r"]
    pub fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: usize) -> *mut c_char;
    #[link_name="switch_wait_sock"]
    pub fn wait_sock(sock: os_socket, ms: u32, flags: poll) -> c_int;
    #[link_name="switch_wait_socklist"]
    pub fn wait_socklist(waitlist: *mut waitlist, len: u32, ms: u32) -> c_int;
    #[link_name="switch_http_parse_header"]
    pub fn http_parse_header(buffer: *mut c_char,
                             datalen: u32,
                             request: *mut http_request)
                             -> status;
    #[link_name="switch_http_free_request"]
    pub fn http_free_request(request: *mut http_request);
    #[link_name="switch_http_dump_request"]
    pub fn http_dump_request(request: *mut http_request);
    #[link_name="switch_http_parse_qs"]
    pub fn http_parse_qs(request: *mut http_request, qs: *mut c_char);
    #[link_name="switch_frame_buffer_free"]
    pub fn frame_buffer_free(fb: *mut frame_buffer, frameP: *mut *mut frame) -> status;
    #[link_name="switch_frame_buffer_dup"]
    pub fn frame_buffer_dup(fb: *mut frame_buffer,
                            orig: *mut frame,
                            clone: *mut *mut frame)
                            -> status;
    #[link_name="switch_frame_buffer_destroy"]
    pub fn frame_buffer_destroy(fbP: *mut *mut frame_buffer) -> status;
    #[link_name="switch_frame_buffer_create"]
    pub fn frame_buffer_create(fbP: *mut *mut frame_buffer) -> status;
    #[link_name="switch_getcputime"]
    pub fn getcputime(t: *mut cputime);
    #[link_name="switch_caller_extension_new"]
    pub fn caller_extension_new(session: *mut core_session,
                                extension_name: *const c_char,
                                extension_number: *const c_char)
                                -> *mut caller_extension;
    #[link_name="switch_caller_extension_clone"]
    pub fn caller_extension_clone(new_ext: *mut *mut caller_extension,
                                  orig: *mut caller_extension,
                                  pool: *mut memory_pool)
                                  -> status;
    #[link_name="switch_caller_extension_add_application"]
    pub fn caller_extension_add_application(session: *mut core_session,
                                            caller_extension: *mut caller_extension,
                                            application_name: *const c_char,
                                            extra_data: *const c_char);
    #[link_name="switch_caller_extension_add_application_printf"]
    pub fn caller_extension_add_application_printf(session: *mut core_session,
                                                   caller_extension: *mut caller_extension,
                                                   application_name: *const c_char,
                                                   fmt: *const c_char,
                                                   ...);
    #[link_name="switch_caller_get_field_by_name"]
    pub fn caller_get_field_by_name(caller_profile: *mut caller_profile,
                                    name: *const c_char)
                                    -> *const c_char;
    #[link_name="switch_caller_profile_new"]
    pub fn caller_profile_new(pool: *mut memory_pool,
                              username: *const c_char,
                              dialplan: *const c_char,
                              caller_id_name: *const c_char,
                              caller_id_number: *const c_char,
                              network_addr: *const c_char,
                              ani: *const c_char,
                              aniii: *const c_char,
                              rdnis: *const c_char,
                              source: *const c_char,
                              context: *const c_char,
                              destination_number: *const c_char)
                              -> *mut caller_profile;
    #[link_name="switch_caller_profile_clone"]
    pub fn caller_profile_clone(session: *mut core_session,
                                tocopy: *mut caller_profile)
                                -> *mut caller_profile;
    #[link_name="switch_caller_profile_dup"]
    pub fn caller_profile_dup(pool: *mut memory_pool,
                              tocopy: *mut caller_profile)
                              -> *mut caller_profile;
    #[link_name="switch_caller_profile_event_set_data"]
    pub fn caller_profile_event_set_data(caller_profile: *mut caller_profile,
                                         prefix: *const c_char,
                                         event: *mut event);
    #[link_name="switch_channel_get_state"]
    pub fn channel_get_state(channel: *mut channel) -> channel_state;
    #[link_name="switch_channel_get_running_state"]
    pub fn channel_get_running_state(channel: *mut channel) -> channel_state;
    #[link_name="switch_channel_check_signal"]
    pub fn channel_check_signal(channel: *mut channel, in_thread_only: switch_bool) -> c_int;
    #[link_name="switch_channel_test_ready"]
    pub fn channel_test_ready(channel: *mut channel,
                              check_ready: switch_bool,
                              check_media: switch_bool)
                              -> c_int;
    #[link_name="switch_channel_wait_for_state"]
    pub fn channel_wait_for_state(channel: *mut channel,
                                  other_channel: *mut channel,
                                  want_state: channel_state);
    #[link_name="switch_channel_wait_for_state_timeout"]
    pub fn channel_wait_for_state_timeout(other_channel: *mut channel,
                                          want_state: channel_state,
                                          timeout: u32);
    #[link_name="switch_channel_wait_for_flag"]
    pub fn channel_wait_for_flag(channel: *mut channel,
                                 want_flag: channel_flag,
                                 pres: switch_bool,
                                 to: u32,
                                 super_channel: *mut channel)
                                 -> status;
    #[link_name="switch_channel_perform_set_state"]
    pub fn channel_perform_set_state(channel: *mut channel,
                                     file: *const c_char,
                                     func: *const c_char,
                                     line: c_int,
                                     state: channel_state)
                                     -> channel_state;
    #[link_name="switch_channel_perform_set_running_state"]
    pub fn channel_perform_set_running_state(channel: *mut channel,
                                             state: channel_state,
                                             file: *const c_char,
                                             func: *const c_char,
                                             line: c_int)
                                             -> channel_state;
    #[link_name="switch_channel_str2cause"]
    pub fn channel_str2cause(str: *const c_char) -> call_cause;
    #[link_name="switch_channel_get_cause"]
    pub fn channel_get_cause(channel: *mut channel) -> call_cause;
    #[link_name="switch_channel_cause_q850"]
    pub fn channel_cause_q850(cause: call_cause) -> call_cause;
    #[link_name="switch_channel_get_cause_q850"]
    pub fn channel_get_cause_q850(channel: *mut channel) -> call_cause;
    #[link_name="switch_channel_get_cause_ptr"]
    pub fn channel_get_cause_ptr(channel: *mut channel) -> *mut call_cause;
    #[link_name="switch_channel_cause2str"]
    pub fn channel_cause2str(cause: call_cause) -> *const c_char;
    #[link_name="switch_channel_get_timetable"]
    pub fn channel_get_timetable(channel: *mut channel) -> *mut channel_timetable;
    #[link_name="switch_channel_alloc"]
    pub fn channel_alloc(channel: *mut *mut channel,
                         direction: call_direction,
                         pool: *mut memory_pool)
                         -> status;
    #[link_name="switch_channel_init"]
    pub fn channel_init(channel: *mut channel,
                        session: *mut core_session,
                        state: channel_state,
                        flag: channel_flag)
                        -> status;
    #[link_name="switch_channel_set_presence_data_vals"]
    pub fn channel_set_presence_data_vals(channel: *mut channel,
                                          presence_data_cols: *const c_char);
    #[link_name="switch_channel_perform_presence"]
    pub fn channel_perform_presence(channel: *mut channel,
                                    rpid: *const c_char,
                                    status: *const c_char,
                                    id: *const c_char,
                                    file: *const c_char,
                                    func: *const c_char,
                                    line: c_int);
    #[link_name="switch_channel_uninit"]
    pub fn channel_uninit(channel: *mut channel);
    #[link_name="switch_channel_set_caller_profile"]
    pub fn channel_set_caller_profile(channel: *mut channel, caller_profile: *mut caller_profile);
    #[link_name="switch_channel_step_caller_profile"]
    pub fn channel_step_caller_profile(channel: *mut channel);
    #[link_name="switch_channel_get_caller_profile"]
    pub fn channel_get_caller_profile(channel: *mut channel) -> *mut caller_profile;
    #[link_name="switch_channel_set_originator_caller_profile"]
    pub fn channel_set_originator_caller_profile(channel: *mut channel,
                                                 caller_profile: *mut caller_profile);
    #[link_name="switch_channel_set_hunt_caller_profile"]
    pub fn channel_set_hunt_caller_profile(channel: *mut channel,
                                           caller_profile: *mut caller_profile);
    #[link_name="switch_channel_get_originator_caller_profile"]
    pub fn channel_get_originator_caller_profile(channel: *mut channel) -> *mut caller_profile;
    #[link_name="switch_channel_set_originatee_caller_profile"]
    pub fn channel_set_originatee_caller_profile(channel: *mut channel,
                                                 caller_profile: *mut caller_profile);
    #[link_name="switch_channel_get_originatee_caller_profile"]
    pub fn channel_get_originatee_caller_profile(channel: *mut channel) -> *mut caller_profile;
    #[link_name="switch_channel_set_origination_caller_profile"]
    pub fn channel_set_origination_caller_profile(channel: *mut channel,
                                                  caller_profile: *mut caller_profile);
    #[link_name="switch_channel_get_origination_caller_profile"]
    pub fn channel_get_origination_caller_profile(channel: *mut channel) -> *mut caller_profile;
    #[link_name="switch_channel_get_uuid"]
    pub fn channel_get_uuid(channel: *mut channel) -> *mut c_char;
    #[link_name="switch_channel_set_profile_var"]
    pub fn channel_set_profile_var(channel: *mut channel,
                                   name: *const c_char,
                                   val: *const c_char)
                                   -> status;
    #[link_name="switch_channel_set_variable_var_check"]
    pub fn channel_set_variable_var_check(channel: *mut channel,
                                          varname: *const c_char,
                                          value: *const c_char,
                                          var_check: switch_bool)
                                          -> status;
    #[link_name="switch_channel_add_variable_var_check"]
    pub fn channel_add_variable_var_check(channel: *mut channel,
                                          varname: *const c_char,
                                          value: *const c_char,
                                          var_check: switch_bool,
                                          stack: stack)
                                          -> status;
    #[link_name="switch_channel_set_variable_printf"]
    pub fn channel_set_variable_printf(channel: *mut channel,
                                       varname: *const c_char,
                                       fmt: *const c_char,
                                       ...)
                                       -> status;
    #[link_name="switch_channel_set_variable_name_printf"]
    pub fn channel_set_variable_name_printf(channel: *mut channel,
                                            val: *const c_char,
                                            fmt: *const c_char,
                                            ...)
                                            -> status;
    #[link_name="switch_channel_set_variable_partner_var_check"]
    pub fn channel_set_variable_partner_var_check(channel: *mut channel,
                                                  varname: *const c_char,
                                                  value: *const c_char,
                                                  var_check: switch_bool)
                                                  -> status;
    #[link_name="switch_channel_get_variable_partner"]
    pub fn channel_get_variable_partner(channel: *mut channel,
                                        varname: *const c_char)
                                        -> *const c_char;
    #[link_name="switch_channel_get_hold_music"]
    pub fn channel_get_hold_music(channel: *mut channel) -> *const c_char;
    #[link_name="switch_channel_get_hold_music_partner"]
    pub fn channel_get_hold_music_partner(channel: *mut channel) -> *const c_char;
    #[link_name="switch_channel_del_variable_prefix"]
    pub fn channel_del_variable_prefix(channel: *mut channel, prefix: *const c_char) -> u32;
    #[link_name="switch_channel_transfer_variable_prefix"]
    pub fn channel_transfer_variable_prefix(orig_channel: *mut channel,
                                            new_channel: *mut channel,
                                            prefix: *const c_char)
                                            -> status;
    #[link_name="switch_channel_export_variable_var_check"]
    pub fn channel_export_variable_var_check(channel: *mut channel,
                                             varname: *const c_char,
                                             val: *const c_char,
                                             export_varname: *const c_char,
                                             var_check: switch_bool)
                                             -> status;
    #[link_name="switch_channel_process_export"]
    pub fn channel_process_export(channel: *mut channel,
                                  peer_channel: *mut channel,
                                  var_event: *mut event,
                                  export_varname: *const c_char);
    #[link_name="switch_channel_export_variable_printf"]
    pub fn channel_export_variable_printf(channel: *mut channel,
                                          varname: *const c_char,
                                          export_varname: *const c_char,
                                          fmt: *const c_char,
                                          ...)
                                          -> status;
    #[link_name="switch_channel_set_scope_variables"]
    pub fn channel_set_scope_variables(channel: *mut channel, event: *mut *mut event);
    #[link_name="switch_channel_get_scope_variables"]
    pub fn channel_get_scope_variables(channel: *mut channel, event: *mut *mut event) -> status;
    #[link_name="switch_channel_get_variable_dup"]
    pub fn channel_get_variable_dup(channel: *mut channel,
                                    varname: *const c_char,
                                    dup: switch_bool,
                                    idx: c_int)
                                    -> *const c_char;
    #[link_name="switch_channel_get_variables"]
    pub fn channel_get_variables(channel: *mut channel, event: *mut *mut event) -> status;
    #[link_name="switch_channel_pass_callee_id"]
    pub fn channel_pass_callee_id(channel: *mut channel, other_channel: *mut channel) -> status;
    #[link_name="switch_channel_variable_first"]
    pub fn channel_variable_first(channel: *mut channel) -> *mut event_header;
    #[link_name="switch_channel_variable_last"]
    pub fn channel_variable_last(channel: *mut channel);
    #[link_name="switch_channel_restart"]
    pub fn channel_restart(channel: *mut channel);
    #[link_name="switch_channel_caller_extension_masquerade"]
    pub fn channel_caller_extension_masquerade(orig_channel: *mut channel,
                                               new_channel: *mut channel,
                                               offset: u32)
                                               -> status;
    #[link_name="switch_channel_set_caller_extension"]
    pub fn channel_set_caller_extension(channel: *mut channel,
                                        caller_extension: *mut caller_extension);
    #[link_name="switch_channel_invert_cid"]
    pub fn channel_invert_cid(channel: *mut channel);
    #[link_name="switch_channel_flip_cid"]
    pub fn channel_flip_cid(channel: *mut channel);
    #[link_name="switch_channel_sort_cid"]
    pub fn channel_sort_cid(channel: *mut channel);
    #[link_name="switch_channel_get_caller_extension"]
    pub fn channel_get_caller_extension(channel: *mut channel) -> *mut caller_extension;
    #[link_name="switch_channel_test_flag"]
    pub fn channel_test_flag(channel: *mut channel, flag: channel_flag) -> u32;
    #[link_name="switch_channel_set_flag_value"]
    pub fn channel_set_flag_value(channel: *mut channel, flag: channel_flag, value: u32);
    #[link_name="switch_channel_set_flag_recursive"]
    pub fn channel_set_flag_recursive(channel: *mut channel, flag: channel_flag);
    #[link_name="switch_channel_set_cap_value"]
    pub fn channel_set_cap_value(channel: *mut channel, cap: channel_cap, value: u32);
    #[link_name="switch_channel_clear_cap"]
    pub fn channel_clear_cap(channel: *mut channel, cap: channel_cap);
    #[link_name="switch_channel_test_cap"]
    pub fn channel_test_cap(channel: *mut channel, cap: channel_cap) -> u32;
    #[link_name="switch_channel_test_cap_partner"]
    pub fn channel_test_cap_partner(channel: *mut channel, cap: channel_cap) -> u32;
    #[link_name="switch_channel_set_flag_partner"]
    pub fn channel_set_flag_partner(channel: *mut channel, flag: channel_flag) -> switch_bool;
    #[link_name="switch_channel_clear_flag_partner"]
    pub fn channel_clear_flag_partner(channel: *mut channel, flag: channel_flag) -> switch_bool;
    #[link_name="switch_channel_test_flag_partner"]
    pub fn channel_test_flag_partner(channel: *mut channel, flag: channel_flag) -> u32;
    #[link_name="switch_channel_set_state_flag"]
    pub fn channel_set_state_flag(channel: *mut channel, flag: channel_flag);
    #[link_name="switch_channel_clear_state_flag"]
    pub fn channel_clear_state_flag(channel: *mut channel, flag: channel_flag);
    #[link_name="switch_channel_clear_flag"]
    pub fn channel_clear_flag(channel: *mut channel, flag: channel_flag);
    #[link_name="switch_channel_clear_flag_recursive"]
    pub fn channel_clear_flag_recursive(channel: *mut channel, flag: channel_flag);
    #[link_name="switch_channel_perform_answer"]
    pub fn channel_perform_answer(channel: *mut channel,
                                  file: *const c_char,
                                  func: *const c_char,
                                  line: c_int)
                                  -> status;
    #[link_name="switch_channel_perform_mark_answered"]
    pub fn channel_perform_mark_answered(channel: *mut channel,
                                         file: *const c_char,
                                         func: *const c_char,
                                         line: c_int)
                                         -> status;
    #[link_name="switch_channel_check_zrtp"]
    pub fn channel_check_zrtp(channel: *mut channel);
    #[link_name="switch_channel_perform_ring_ready_value"]
    pub fn channel_perform_ring_ready_value(channel: *mut channel,
                                            rv: ring_ready,
                                            file: *const c_char,
                                            func: *const c_char,
                                            line: c_int)
                                            -> status;
    #[link_name="switch_channel_perform_pre_answer"]
    pub fn channel_perform_pre_answer(channel: *mut channel,
                                      file: *const c_char,
                                      func: *const c_char,
                                      line: c_int)
                                      -> status;
    #[link_name="switch_channel_perform_mark_pre_answered"]
    pub fn channel_perform_mark_pre_answered(channel: *mut channel,
                                             file: *const c_char,
                                             func: *const c_char,
                                             line: c_int)
                                             -> status;
    #[link_name="switch_channel_perform_mark_ring_ready_value"]
    pub fn channel_perform_mark_ring_ready_value(channel: *mut channel,
                                                 rv: ring_ready,
                                                 file: *const c_char,
                                                 func: *const c_char,
                                                 line: c_int)
                                                 -> status;
    #[link_name="switch_channel_add_state_handler"]
    pub fn channel_add_state_handler(channel: *mut channel,
                                     state_handler: *const state_handler_table)
                                     -> c_int;
    #[link_name="switch_channel_clear_state_handler"]
    pub fn channel_clear_state_handler(channel: *mut channel,
                                       state_handler: *const state_handler_table);
    #[link_name="switch_channel_get_state_handler"]
    pub fn channel_get_state_handler(channel: *mut channel,
                                     index: c_int)
                                     -> *const state_handler_table;
    #[link_name="switch_channel_set_private"]
    pub fn channel_set_private(channel: *mut channel,
                               key: *const c_char,
                               private_info: *const c_void)
                               -> status;
    #[link_name="switch_channel_get_private"]
    pub fn channel_get_private(channel: *mut channel, key: *const c_char) -> *mut c_void;
    #[link_name="switch_channel_get_private_partner"]
    pub fn channel_get_private_partner(channel: *mut channel, key: *const c_char) -> *mut c_void;
    #[link_name="switch_channel_set_name"]
    pub fn channel_set_name(channel: *mut channel, name: *const c_char) -> status;
    #[link_name="switch_channel_get_name"]
    pub fn channel_get_name(channel: *mut channel) -> *mut c_char;
    #[link_name="switch_channel_perform_hangup"]
    pub fn channel_perform_hangup(channel: *mut channel,
                                  file: *const c_char,
                                  func: *const c_char,
                                  line: c_int,
                                  hangup_cause: call_cause)
                                  -> channel_state;
    #[link_name="switch_channel_has_dtmf"]
    pub fn channel_has_dtmf(channel: *mut channel) -> usize;
    #[link_name="switch_channel_dtmf_lock"]
    pub fn channel_dtmf_lock(channel: *mut channel) -> status;
    #[link_name="switch_channel_try_dtmf_lock"]
    pub fn channel_try_dtmf_lock(channel: *mut channel) -> status;
    #[link_name="switch_channel_dtmf_unlock"]
    pub fn channel_dtmf_unlock(channel: *mut channel) -> status;
    #[link_name="switch_channel_queue_dtmf"]
    pub fn channel_queue_dtmf(channel: *mut channel, dtmf: *const dtmf) -> status;
    #[link_name="switch_channel_queue_dtmf_string"]
    pub fn channel_queue_dtmf_string(channel: *mut channel, dtmf_string: *const c_char) -> status;
    #[link_name="switch_channel_dequeue_dtmf"]
    pub fn channel_dequeue_dtmf(channel: *mut channel, dtmf: *mut dtmf) -> status;
    #[link_name="switch_channel_flush_dtmf"]
    pub fn channel_flush_dtmf(channel: *mut channel);
    #[link_name="switch_channel_dequeue_dtmf_string"]
    pub fn channel_dequeue_dtmf_string(channel: *mut channel,
                                       dtmf_str: *mut c_char,
                                       len: usize)
                                       -> usize;
    #[link_name="switch_channel_state_name"]
    pub fn channel_state_name(state: channel_state) -> *const c_char;
    #[link_name="switch_channel_name_state"]
    pub fn channel_name_state(name: *const c_char) -> channel_state;
    #[link_name="switch_channel_event_set_data"]
    pub fn channel_event_set_data(channel: *mut channel, event: *mut event);
    #[link_name="switch_channel_event_set_basic_data"]
    pub fn channel_event_set_basic_data(channel: *mut channel, event: *mut event);
    #[link_name="switch_channel_event_set_extended_data"]
    pub fn channel_event_set_extended_data(channel: *mut channel, event: *mut event);
    #[link_name="switch_channel_expand_variables_check"]
    pub fn channel_expand_variables_check(channel: *mut channel,
                                          in_: *const c_char,
                                          var_list: *mut event,
                                          api_list: *mut event,
                                          recur: u32)
                                          -> *mut c_char;
    #[link_name="switch_channel_build_param_string"]
    pub fn channel_build_param_string(channel: *mut channel,
                                      caller_profile: *mut caller_profile,
                                      prefix: *const c_char)
                                      -> *mut c_char;
    #[link_name="switch_channel_set_timestamps"]
    pub fn channel_set_timestamps(channel: *mut channel) -> status;
    #[link_name="switch_channel_perform_audio_sync"]
    pub fn channel_perform_audio_sync(channel: *mut channel,
                                      file: *const c_char,
                                      func: *const c_char,
                                      line: c_int);
    #[link_name="switch_channel_perform_video_sync"]
    pub fn channel_perform_video_sync(channel: *mut channel,
                                      file: *const c_char,
                                      func: *const c_char,
                                      line: c_int);
    #[link_name="switch_channel_set_private_flag"]
    pub fn channel_set_private_flag(channel: *mut channel, flags: u32);
    #[link_name="switch_channel_clear_private_flag"]
    pub fn channel_clear_private_flag(channel: *mut channel, flags: u32);
    #[link_name="switch_channel_test_private_flag"]
    pub fn channel_test_private_flag(channel: *mut channel, flags: u32) -> c_int;
    #[link_name="switch_channel_set_app_flag_key"]
    pub fn channel_set_app_flag_key(app: *const c_char, channel: *mut channel, flags: u32);
    #[link_name="switch_channel_clear_app_flag_key"]
    pub fn channel_clear_app_flag_key(app: *const c_char, channel: *mut channel, flags: u32);
    #[link_name="switch_channel_test_app_flag_key"]
    pub fn channel_test_app_flag_key(app: *const c_char,
                                     channel: *mut channel,
                                     flags: u32)
                                     -> c_int;
    #[link_name="switch_channel_set_bridge_time"]
    pub fn channel_set_bridge_time(channel: *mut channel);
    #[link_name="switch_channel_set_hangup_time"]
    pub fn channel_set_hangup_time(channel: *mut channel);
    #[link_name="switch_channel_direction"]
    pub fn channel_direction(channel: *mut channel) -> call_direction;
    #[link_name="switch_channel_logical_direction"]
    pub fn channel_logical_direction(channel: *mut channel) -> call_direction;
    #[link_name="switch_channel_set_direction"]
    pub fn channel_set_direction(channel: *mut channel, direction: call_direction);
    #[link_name="switch_channel_get_session"]
    pub fn channel_get_session(channel: *mut channel) -> *mut core_session;
    #[link_name="switch_channel_get_flag_string"]
    pub fn channel_get_flag_string(channel: *mut channel) -> *mut c_char;
    #[link_name="switch_channel_get_cap_string"]
    pub fn channel_get_cap_string(channel: *mut channel) -> *mut c_char;
    #[link_name="switch_channel_state_change_pending"]
    pub fn channel_state_change_pending(channel: *mut channel) -> c_int;
    #[link_name="switch_channel_perform_set_callstate"]
    pub fn channel_perform_set_callstate(channel: *mut channel,
                                         callstate: channel_callstate,
                                         file: *const c_char,
                                         func: *const c_char,
                                         line: c_int);
    #[link_name="switch_channel_get_callstate"]
    pub fn channel_get_callstate(channel: *mut channel) -> channel_callstate;
    #[link_name="switch_channel_callstate2str"]
    pub fn channel_callstate2str(callstate: channel_callstate) -> *const c_char;
    #[link_name="switch_channel_str2callstate"]
    pub fn channel_str2callstate(str: *const c_char) -> channel_callstate;
    #[link_name="switch_channel_mark_hold"]
    pub fn channel_mark_hold(channel: *mut channel, on: switch_bool);
    #[link_name="switch_channel_execute_on"]
    pub fn channel_execute_on(channel: *mut channel, variable_prefix: *const c_char) -> status;
    #[link_name="switch_channel_api_on"]
    pub fn channel_api_on(channel: *mut channel, variable_prefix: *const c_char) -> status;
    #[link_name="switch_channel_process_device_hangup"]
    pub fn channel_process_device_hangup(channel: *mut channel);
    #[link_name="switch_channel_get_queued_extension"]
    pub fn channel_get_queued_extension(channel: *mut channel) -> *mut caller_extension;
    #[link_name="switch_channel_transfer_to_extension"]
    pub fn channel_transfer_to_extension(channel: *mut channel,
                                         caller_extension: *mut caller_extension);
    #[link_name="switch_channel_get_partner_uuid"]
    pub fn channel_get_partner_uuid(channel: *mut channel) -> *const c_char;
    #[link_name="switch_channel_get_hold_record"]
    pub fn channel_get_hold_record(channel: *mut channel) -> *mut hold_record;
    #[link_name="switch_channel_state_thread_lock"]
    pub fn channel_state_thread_lock(channel: *mut channel);
    #[link_name="switch_channel_state_thread_unlock"]
    pub fn channel_state_thread_unlock(channel: *mut channel);
    #[link_name="switch_channel_state_thread_trylock"]
    pub fn channel_state_thread_trylock(channel: *mut channel) -> status;
    #[link_name="switch_channel_handle_cause"]
    pub fn channel_handle_cause(channel: *mut channel, cause: call_cause);
    #[link_name="switch_channel_global_init"]
    pub fn channel_global_init(pool: *mut memory_pool);
    #[link_name="switch_channel_global_uninit"]
    pub fn channel_global_uninit();
    #[link_name="switch_channel_set_device_id"]
    pub fn channel_set_device_id(channel: *mut channel, device_id: *const c_char) -> *const c_char;
    #[link_name="switch_channel_clear_device_record"]
    pub fn channel_clear_device_record(channel: *mut channel);
    #[link_name="switch_channel_get_device_record"]
    pub fn channel_get_device_record(channel: *mut channel) -> *mut device_record;
    #[link_name="switch_channel_release_device_record"]
    pub fn channel_release_device_record(dcdrp: *mut *mut device_record);
    #[link_name="switch_channel_bind_device_state_handler"]
    pub fn channel_bind_device_state_handler(function: device_state_function,
                                             user_data: *mut c_void)
                                             -> status;
    #[link_name="switch_channel_unbind_device_state_handler"]
    pub fn channel_unbind_device_state_handler(function: device_state_function) -> status;
    #[link_name="switch_channel_device_state2str"]
    pub fn channel_device_state2str(device_state: device_state) -> *const c_char;
    #[link_name="switch_channel_pass_sdp"]
    pub fn channel_pass_sdp(from_channel: *mut channel,
                            to_channel: *mut channel,
                            sdp: *const c_char)
                            -> status;
    #[link_name="switch_buffer_create_partition"]
    pub fn buffer_create_partition(pool: *mut memory_pool,
                                   buffer: *mut *mut buffer,
                                   data: *mut c_void,
                                   datalen: usize)
                                   -> status;
    #[link_name="switch_buffer_set_partition_data"]
    pub fn buffer_set_partition_data(buffer: *mut buffer,
                                     data: *mut c_void,
                                     datalen: usize)
                                     -> status;
    #[link_name="switch_buffer_reset_partition_data"]
    pub fn buffer_reset_partition_data(buffer: *mut buffer) -> status;
    #[link_name="switch_buffer_create"]
    pub fn buffer_create(pool: *mut memory_pool,
                         buffer: *mut *mut buffer,
                         max_len: usize)
                         -> status;
    #[link_name="switch_buffer_create_dynamic"]
    pub fn buffer_create_dynamic(buffer: *mut *mut buffer,
                                 blocksize: usize,
                                 start_len: usize,
                                 max_len: usize)
                                 -> status;
    #[link_name="switch_buffer_add_mutex"]
    pub fn buffer_add_mutex(buffer: *mut buffer, mutex: *mut mutex);
    #[link_name="switch_buffer_lock"]
    pub fn buffer_lock(buffer: *mut buffer);
    #[link_name="switch_buffer_trylock"]
    pub fn buffer_trylock(buffer: *mut buffer) -> status;
    #[link_name="switch_buffer_unlock"]
    pub fn buffer_unlock(buffer: *mut buffer);
    #[link_name="switch_buffer_len"]
    pub fn buffer_len(buffer: *mut buffer) -> usize;
    #[link_name="switch_buffer_freespace"]
    pub fn buffer_freespace(buffer: *mut buffer) -> usize;
    #[link_name="switch_buffer_inuse"]
    pub fn buffer_inuse(buffer: *mut buffer) -> usize;
    #[link_name="switch_buffer_read"]
    pub fn buffer_read(buffer: *mut buffer, data: *mut c_void, datalen: usize) -> usize;
    #[link_name="switch_buffer_peek"]
    pub fn buffer_peek(buffer: *mut buffer, data: *mut c_void, datalen: usize) -> usize;
    #[link_name="switch_buffer_peek_zerocopy"]
    pub fn buffer_peek_zerocopy(buffer: *mut buffer, ptr: *mut *const c_void) -> usize;
    #[link_name="switch_buffer_read_loop"]
    pub fn buffer_read_loop(buffer: *mut buffer, data: *mut c_void, datalen: usize) -> usize;
    #[link_name="switch_buffer_set_loops"]
    pub fn buffer_set_loops(buffer: *mut buffer, loops: i32);
    #[link_name="switch_buffer_write"]
    pub fn buffer_write(buffer: *mut buffer, data: *const c_void, datalen: usize) -> usize;
    #[link_name="switch_buffer_toss"]
    pub fn buffer_toss(buffer: *mut buffer, datalen: usize) -> usize;
    #[link_name="switch_buffer_zero"]
    pub fn buffer_zero(buffer: *mut buffer);
    #[link_name="switch_buffer_slide_write"]
    pub fn buffer_slide_write(buffer: *mut buffer, data: *const c_void, datalen: usize) -> usize;
    #[link_name="switch_buffer_destroy"]
    pub fn buffer_destroy(buffer: *mut *mut buffer);
    #[link_name="switch_buffer_zwrite"]
    pub fn buffer_zwrite(buffer: *mut buffer, data: *const c_void, datalen: usize) -> usize;
    #[link_name="switch_event_init"]
    pub fn event_init(pool: *mut memory_pool) -> status;
    #[link_name="switch_event_shutdown"]
    pub fn event_shutdown() -> status;
    #[link_name="switch_event_create_subclass_detailed"]
    pub fn event_create_subclass_detailed(file: *const c_char,
                                          func: *const c_char,
                                          line: c_int,
                                          event: *mut *mut event,
                                          event_id: event_types,
                                          subclass_name: *const c_char)
                                          -> status;
    #[link_name="switch_event_set_priority"]
    pub fn event_set_priority(event: *mut event, priority: priority) -> status;
    #[link_name="switch_event_get_header_ptr"]
    pub fn event_get_header_ptr(event: *mut event,
                                header_name: *const c_char)
                                -> *mut event_header;
    #[link_name="switch_event_get_header_idx"]
    pub fn event_get_header_idx(event: *mut event,
                                header_name: *const c_char,
                                idx: c_int)
                                -> *mut c_char;
    #[link_name="switch_event_rename_header"]
    pub fn event_rename_header(event: *mut event,
                               header_name: *const c_char,
                               new_header_name: *const c_char)
                               -> status;
    #[link_name="switch_event_get_body"]
    pub fn event_get_body(event: *mut event) -> *mut c_char;
    #[link_name="switch_event_add_header"]
    pub fn event_add_header(event: *mut event,
                            stack: stack,
                            header_name: *const c_char,
                            fmt: *const c_char,
                            ...)
                            -> status;
    #[link_name="switch_event_set_subclass_name"]
    pub fn event_set_subclass_name(event: *mut event, subclass_name: *const c_char) -> status;
    #[link_name="switch_event_add_header_string"]
    pub fn event_add_header_string(event: *mut event,
                                   stack: stack,
                                   header_name: *const c_char,
                                   data: *const c_char)
                                   -> status;
    #[link_name="switch_event_del_header_val"]
    pub fn event_del_header_val(event: *mut event,
                                header_name: *const c_char,
                                val: *const c_char)
                                -> status;
    #[link_name="switch_event_add_array"]
    pub fn event_add_array(event: *mut event, var: *const c_char, val: *const c_char) -> c_int;
    #[link_name="switch_event_destroy"]
    pub fn event_destroy(event: *mut *mut event);
    #[link_name="switch_event_dup"]
    pub fn event_dup(event: *mut *mut event, todup: *mut event) -> status;
    #[link_name="switch_event_merge"]
    pub fn event_merge(event: *mut event, tomerge: *mut event);
    #[link_name="switch_event_dup_reply"]
    pub fn event_dup_reply(event: *mut *mut event, todup: *mut event) -> status;
    #[link_name="switch_event_fire_detailed"]
    pub fn event_fire_detailed(file: *const c_char,
                               func: *const c_char,
                               line: c_int,
                               event: *mut *mut event,
                               user_data: *mut c_void)
                               -> status;
    #[link_name="switch_event_prep_for_delivery_detailed"]
    pub fn event_prep_for_delivery_detailed(file: *const c_char,
                                            func: *const c_char,
                                            line: c_int,
                                            event: *mut event);
    #[link_name="switch_event_bind"]
    pub fn event_bind(id: *const c_char,
                      event: event_types,
                      subclass_name: *const c_char,
                      callback: event_callback,
                      user_data: *mut c_void)
                      -> status;
    #[link_name="switch_event_get_custom_events"]
    pub fn event_get_custom_events(matches: *mut *mut console_callback_match) -> status;
    #[link_name="switch_event_bind_removable"]
    pub fn event_bind_removable(id: *const c_char,
                                event: event_types,
                                subclass_name: *const c_char,
                                callback: event_callback,
                                user_data: *mut c_void,
                                node: *mut *mut event_node)
                                -> status;
    #[link_name="switch_event_unbind"]
    pub fn event_unbind(node: *mut *mut event_node) -> status;
    #[link_name="switch_event_unbind_callback"]
    pub fn event_unbind_callback(callback: event_callback) -> status;
    #[link_name="switch_event_name"]
    pub fn event_name(event: event_types) -> *const c_char;
    #[link_name="switch_name_event"]
    pub fn name_event(name: *const c_char, type_: *mut event_types) -> status;
    #[link_name="switch_event_reserve_subclass_detailed"]
    pub fn event_reserve_subclass_detailed(owner: *const c_char,
                                           subclass_name: *const c_char)
                                           -> status;
    #[link_name="switch_event_free_subclass_detailed"]
    pub fn event_free_subclass_detailed(owner: *const c_char,
                                        subclass_name: *const c_char)
                                        -> status;
    #[link_name="switch_event_binary_deserialize"]
    pub fn event_binary_deserialize(eventp: *mut *mut event,
                                    data: *mut *mut c_void,
                                    len: usize,
                                    duplicate: switch_bool)
                                    -> status;
    #[link_name="switch_event_binary_serialize"]
    pub fn event_binary_serialize(event: *mut event,
                                  data: *mut *mut c_void,
                                  len: *mut usize)
                                  -> status;
    #[link_name="switch_event_serialize"]
    pub fn event_serialize(event: *mut event,
                           str: *mut *mut c_char,
                           encode: switch_bool)
                           -> status;
    #[link_name="switch_event_serialize_json"]
    pub fn event_serialize_json(event: *mut event, str: *mut *mut c_char) -> status;
    #[link_name="switch_event_serialize_json_obj"]
    pub fn event_serialize_json_obj(event: *mut event, json: *mut *mut cJSON) -> status;
    #[link_name="switch_event_create_json"]
    pub fn event_create_json(event: *mut *mut event, json: *const c_char) -> status;
    #[link_name="switch_event_create_brackets"]
    pub fn event_create_brackets(data: *mut c_char,
                                 a: c_char,
                                 b: c_char,
                                 c: c_char,
                                 event: *mut *mut event,
                                 new_data: *mut *mut c_char,
                                 dup: switch_bool)
                                 -> status;
    #[link_name="switch_event_create_array_pair"]
    pub fn event_create_array_pair(event: *mut *mut event,
                                   names: *mut *mut c_char,
                                   vals: *mut *mut c_char,
                                   len: c_int)
                                   -> status;
    #[link_name="switch_event_xmlize"]
    pub fn event_xmlize(event: *mut event, fmt: *const c_char, ...) -> xml_t;
    #[link_name="switch_event_running"]
    pub fn event_running() -> status;
    #[link_name="switch_event_add_body"]
    pub fn event_add_body(event: *mut event, fmt: *const c_char, ...) -> status;
    #[link_name="switch_event_set_body"]
    pub fn event_set_body(event: *mut event, body: *const c_char) -> status;
    #[link_name="switch_event_expand_headers_check"]
    pub fn event_expand_headers_check(event: *mut event,
                                      in_: *const c_char,
                                      var_list: *mut event,
                                      api_list: *mut event,
                                      recur: u32)
                                      -> *mut c_char;
    #[link_name="switch_event_create_pres_in_detailed"]
    pub fn event_create_pres_in_detailed(file: *mut c_char,
                                         func: *mut c_char,
                                         line: c_int,
                                         proto: *const c_char,
                                         login: *const c_char,
                                         from: *const c_char,
                                         from_domain: *const c_char,
                                         status: *const c_char,
                                         event_type: *const c_char,
                                         alt_event_type: *const c_char,
                                         event_count: c_int,
                                         unique_id: *const c_char,
                                         channel_state: *const c_char,
                                         answer_state: *const c_char,
                                         call_direction: *const c_char)
                                         -> status;
    #[link_name="switch_event_deliver"]
    pub fn event_deliver(event: *mut *mut event);
    #[link_name="switch_event_build_param_string"]
    pub fn event_build_param_string(event: *mut event,
                                    prefix: *const c_char,
                                    vars_map: *mut hash)
                                    -> *mut c_char;
    #[link_name="switch_event_check_permission_list"]
    pub fn event_check_permission_list(list: *mut event, name: *const c_char) -> c_int;
    #[link_name="switch_event_add_presence_data_cols"]
    pub fn event_add_presence_data_cols(channel: *mut channel,
                                        event: *mut event,
                                        prefix: *const c_char);
    #[link_name="switch_json_add_presence_data_cols"]
    pub fn json_add_presence_data_cols(event: *mut event,
                                       json: *mut cJSON,
                                       prefix: *const c_char);
    #[link_name="switch_event_launch_dispatch_threads"]
    pub fn event_launch_dispatch_threads(max: u32);
    #[link_name="switch_event_channel_broadcast"]
    pub fn event_channel_broadcast(event_channel: *const c_char,
                                   json: *mut *mut cJSON,
                                   key: *const c_char,
                                   id: event_channel_id)
                                   -> status;
    #[link_name="switch_event_channel_unbind"]
    pub fn event_channel_unbind(event_channel: *const c_char, func: event_channel_func) -> u32;
    #[link_name="switch_event_channel_bind"]
    pub fn event_channel_bind(event_channel: *const c_char,
                              func: event_channel_func,
                              id: *mut event_channel_id)
                              -> status;
    #[link_name="switch_live_array_clear"]
    pub fn live_array_clear(la: *mut live_array) -> status;
    #[link_name="switch_live_array_bootstrap"]
    pub fn live_array_bootstrap(la: *mut live_array,
                                sessid: *const c_char,
                                channel_id: event_channel_id)
                                -> status;
    #[link_name="switch_live_array_destroy"]
    pub fn live_array_destroy(live_arrayP: *mut *mut live_array) -> status;
    #[link_name="switch_live_array_create"]
    pub fn live_array_create(event_channel: *const c_char,
                             name: *const c_char,
                             channel_id: event_channel_id,
                             live_arrayP: *mut *mut live_array)
                             -> status;
    #[link_name="switch_live_array_get"]
    pub fn live_array_get(la: *mut live_array, name: *const c_char) -> *mut cJSON;
    #[link_name="switch_live_array_get_idx"]
    pub fn live_array_get_idx(la: *mut live_array, idx: c_int) -> *mut cJSON;
    #[link_name="switch_live_array_del"]
    pub fn live_array_del(la: *mut live_array, name: *const c_char) -> status;
    #[link_name="switch_live_array_add"]
    pub fn live_array_add(la: *mut live_array,
                          name: *const c_char,
                          index: c_int,
                          obj: *mut *mut cJSON,
                          destroy: switch_bool)
                          -> status;
    #[link_name="switch_live_array_visible"]
    pub fn live_array_visible(la: *mut live_array,
                              visible: switch_bool,
                              force: switch_bool)
                              -> status;
    #[link_name="switch_live_array_isnew"]
    pub fn live_array_isnew(la: *mut live_array) -> switch_bool;
    #[link_name="switch_live_array_lock"]
    pub fn live_array_lock(la: *mut live_array);
    #[link_name="switch_live_array_unlock"]
    pub fn live_array_unlock(la: *mut live_array);
    #[link_name="switch_live_array_set_user_data"]
    pub fn live_array_set_user_data(la: *mut live_array, user_data: *mut c_void);
    #[link_name="switch_live_array_set_command_handler"]
    pub fn live_array_set_command_handler(la: *mut live_array,
                                          command_handler: live_array_command_handler);
    #[link_name="switch_live_array_parse_json"]
    pub fn live_array_parse_json(json: *mut cJSON, channel_id: event_channel_id);
    #[link_name="switch_live_array_add_alias"]
    pub fn live_array_add_alias(la: *mut live_array,
                                event_channel: *const c_char,
                                name: *const c_char)
                                -> switch_bool;
    #[link_name="switch_live_array_clear_alias"]
    pub fn live_array_clear_alias(la: *mut live_array,
                                  event_channel: *const c_char,
                                  name: *const c_char)
                                  -> switch_bool;
    #[link_name="switch_event_channel_permission_verify"]
    pub fn event_channel_permission_verify(cookie: *const c_char,
                                           event_channel: *const c_char)
                                           -> switch_bool;
    #[link_name="switch_event_channel_permission_modify"]
    pub fn event_channel_permission_modify(cookie: *const c_char,
                                           event_channel: *const c_char,
                                           set: switch_bool);
    #[link_name="switch_event_channel_permission_clear"]
    pub fn event_channel_permission_clear(cookie: *const c_char);
    #[link_name="switch_img_alloc"]
    pub fn img_alloc(img: *mut vpx_image,
                     fmt: vpx_img_fmt,
                     d_w: c_uint,
                     d_h: c_uint,
                     align: c_uint)
                     -> *mut vpx_image;
    #[link_name="switch_img_wrap"]
    pub fn img_wrap(img: *mut vpx_image,
                    fmt: vpx_img_fmt,
                    d_w: c_uint,
                    d_h: c_uint,
                    align: c_uint,
                    img_data: *mut c_uchar)
                    -> *mut vpx_image;
    #[link_name="switch_img_set_rect"]
    pub fn img_set_rect(img: *mut vpx_image, x: c_uint, y: c_uint, w: c_uint, h: c_uint) -> c_int;
    #[link_name="switch_img_patch"]
    pub fn img_patch(IMG: *mut vpx_image, img: *mut vpx_image, x: c_int, y: c_int);
    #[link_name="switch_img_patch_rect"]
    pub fn img_patch_rect(IMG: *mut vpx_image,
                          X: c_int,
                          Y: c_int,
                          img: *mut vpx_image,
                          x: u32,
                          y: u32,
                          w: u32,
                          h: u32);
    #[link_name="switch_img_copy"]
    pub fn img_copy(img: *mut vpx_image, new_img: *mut *mut vpx_image);
    #[link_name="switch_img_rotate_copy"]
    pub fn img_rotate_copy(img: *mut vpx_image,
                           new_img: *mut *mut vpx_image,
                           mode: image_rotation_mode);
    #[link_name="switch_img_rotate"]
    pub fn img_rotate(img: *mut *mut vpx_image, mode: image_rotation_mode);
    #[link_name="switch_img_free"]
    pub fn img_free(img: *mut *mut vpx_image);
    #[link_name="switch_img_draw_text"]
    pub fn img_draw_text(IMG: *mut vpx_image,
                         x: c_int,
                         y: c_int,
                         color: rgb_color,
                         font_size: u16,
                         text: *mut c_char);
    #[link_name="switch_img_add_text"]
    pub fn img_add_text(buffer: *mut c_void, w: c_int, x: c_int, y: c_int, s: *mut c_char);
    #[link_name="switch_img_copy_rect"]
    pub fn img_copy_rect(img: *mut vpx_image, x: u32, y: u32, w: u32, h: u32) -> *mut vpx_image;
    #[link_name="switch_img_fill"]
    pub fn img_fill(img: *mut vpx_image,
                    x: c_int,
                    y: c_int,
                    w: c_int,
                    h: c_int,
                    color: *mut rgb_color);
    #[link_name="switch_color_set_rgb"]
    pub fn color_set_rgb(color: *mut rgb_color, color_str: *const c_char);
    #[link_name="switch_color_set_yuv"]
    pub fn color_set_yuv(color: *mut yuv_color, color_str: *const c_char);
    #[link_name="switch_img_txt_handle_create"]
    pub fn img_txt_handle_create(handleP: *mut *mut img_txt_handle,
                                 font_family: *const c_char,
                                 font_color: *const c_char,
                                 bgcolor: *const c_char,
                                 font_size: u16,
                                 angle: f64,
                                 pool: *mut memory_pool)
                                 -> status;
    #[link_name="switch_img_txt_handle_destroy"]
    pub fn img_txt_handle_destroy(handleP: *mut *mut img_txt_handle);
    #[link_name="switch_img_txt_handle_render"]
    pub fn img_txt_handle_render(handle: *mut img_txt_handle,
                                 img: *mut vpx_image,
                                 x: c_int,
                                 y: c_int,
                                 text: *const c_char,
                                 font_family: *const c_char,
                                 font_color: *const c_char,
                                 bgcolor: *const c_char,
                                 font_size: u16,
                                 angle: f64)
                                 -> u32;
    #[link_name="switch_img_patch_hole"]
    pub fn img_patch_hole(IMG: *mut vpx_image,
                          img: *mut vpx_image,
                          x: c_int,
                          y: c_int,
                          rect: *mut vpx_image_rect);
    #[link_name="switch_png_patch_img"]
    pub fn png_patch_img(use_png: *mut png, img: *mut vpx_image, x: c_int, y: c_int) -> status;
    #[link_name="switch_img_read_png"]
    pub fn img_read_png(file_name: *const c_char, img_fmt: vpx_img_fmt) -> *mut vpx_image;
    #[link_name="switch_img_write_png"]
    pub fn img_write_png(img: *mut vpx_image, file_name: *mut c_char) -> status;
    #[link_name="switch_png_open"]
    pub fn png_open(pngP: *mut *mut png, file_name: *const c_char) -> status;
    #[link_name="switch_png_free"]
    pub fn png_free(pngP: *mut *mut png);
    #[link_name="switch_img_overlay"]
    pub fn img_overlay(IMG: *mut vpx_image, img: *mut vpx_image, x: c_int, y: c_int, percent: u8);
    #[link_name="switch_img_scale"]
    pub fn img_scale(src: *mut vpx_image,
                     destP: *mut *mut vpx_image,
                     width: c_int,
                     height: c_int)
                     -> status;
    #[link_name="switch_img_fit"]
    pub fn img_fit(srcP: *mut *mut vpx_image, width: c_int, height: c_int, fit: img_fit) -> status;
    pub fn parse_img_position(name: *const c_char) -> img_position;
    pub fn parse_img_fit(name: *const c_char) -> img_fit;
    #[link_name="switch_img_find_position"]
    pub fn img_find_position(pos: img_position,
                             sw: c_int,
                             sh: c_int,
                             iw: c_int,
                             ih: c_int,
                             xP: *mut c_int,
                             yP: *mut c_int);
    #[link_name="switch_img_to_raw"]
    pub fn img_to_raw(src: *mut vpx_image,
                      dest: *mut c_void,
                      stride: c_int,
                      fmt: vpx_img_fmt)
                      -> status;
    #[link_name="switch_img_from_raw"]
    pub fn img_from_raw(dest: *mut vpx_image,
                        src: *mut c_void,
                        fmt: vpx_img_fmt,
                        width: c_int,
                        height: c_int)
                        -> status;
    #[link_name="switch_img_write_text_img"]
    pub fn img_write_text_img(w: c_int,
                              h: c_int,
                              full: switch_bool,
                              text: *const c_char)
                              -> *mut vpx_image;
    #[link_name="switch_img_read_file"]
    pub fn img_read_file(file_name: *const c_char) -> *mut vpx_image;
    #[link_name="switch_img_letterbox"]
    pub fn img_letterbox(img: *mut vpx_image,
                         imgP: *mut *mut vpx_image,
                         width: c_int,
                         height: c_int,
                         color: *const c_char)
                         -> status;
    #[link_name="switch_core_has_video"]
    pub fn core_has_video() -> switch_bool;
    #[link_name="switch_I420_copy"]
    pub fn I420_copy(src_y: *const u8,
                     src_stride_y: c_int,
                     src_u: *const u8,
                     src_stride_u: c_int,
                     src_v: *const u8,
                     src_stride_v: c_int,
                     dst_y: *mut u8,
                     dst_stride_y: c_int,
                     dst_u: *mut u8,
                     dst_stride_u: c_int,
                     dst_v: *mut u8,
                     dst_stride_v: c_int,
                     width: c_int,
                     height: c_int)
                     -> status;
    #[link_name="switch_I420_copy2"]
    pub fn I420_copy2(src_planes: *mut *mut u8,
                      src_stride: *mut c_int,
                      dst_planes: *mut *mut u8,
                      dst_stride: *mut c_int,
                      width: c_int,
                      height: c_int)
                      -> status;
    #[link_name="switch_ivr_deactivate_unicast"]
    pub fn ivr_deactivate_unicast(session: *mut core_session) -> status;
    #[link_name="switch_ivr_activate_unicast"]
    pub fn ivr_activate_unicast(session: *mut core_session,
                                local_ip: *mut c_char,
                                local_port: port,
                                remote_ip: *mut c_char,
                                remote_port: port,
                                transport: *mut c_char,
                                flags: *mut c_char)
                                -> status;
    #[link_name="switch_ivr_generate_json_cdr"]
    pub fn ivr_generate_json_cdr(session: *mut core_session,
                                 json_cdr: *mut *mut cJSON,
                                 urlencode: switch_bool)
                                 -> status;
    #[link_name="switch_ivr_generate_xml_cdr"]
    pub fn ivr_generate_xml_cdr(session: *mut core_session, xml_cdr: *mut xml_t) -> status;
    #[link_name="switch_ivr_set_xml_profile_data"]
    pub fn ivr_set_xml_profile_data(xml: xml_t,
                                    caller_profile: *mut caller_profile,
                                    off: c_int)
                                    -> c_int;
    #[link_name="switch_ivr_set_xml_chan_vars"]
    pub fn ivr_set_xml_chan_vars(xml: xml_t, channel: *mut channel, off: c_int) -> c_int;
    #[link_name="switch_ivr_parse_event"]
    pub fn ivr_parse_event(session: *mut core_session, event: *mut event) -> status;
    #[link_name="switch_ivr_parse_all_events"]
    pub fn ivr_parse_all_events(session: *mut core_session) -> status;
    #[link_name="switch_ivr_parse_next_event"]
    pub fn ivr_parse_next_event(session: *mut core_session) -> status;
    #[link_name="switch_ivr_parse_all_messages"]
    pub fn ivr_parse_all_messages(session: *mut core_session) -> status;
    #[link_name="switch_ivr_parse_all_signal_data"]
    pub fn ivr_parse_all_signal_data(session: *mut core_session) -> status;
    #[link_name="switch_ivr_parse_signal_data"]
    pub fn ivr_parse_signal_data(session: *mut core_session,
                                 all: switch_bool,
                                 only_session_thread: switch_bool)
                                 -> status;
    #[link_name="switch_ivr_parse_next_signal_data"]
    pub fn ivr_parse_next_signal_data(session: *mut core_session) -> status;
    #[link_name="switch_ivr_process_indications"]
    pub fn ivr_process_indications(session: *mut core_session,
                                   message: *mut core_session_message)
                                   -> status;
    #[link_name="switch_ivr_sleep"]
    pub fn ivr_sleep(session: *mut core_session,
                     ms: u32,
                     sync: switch_bool,
                     args: *mut input_args)
                     -> status;
    #[link_name="switch_ivr_park"]
    pub fn ivr_park(session: *mut core_session, args: *mut input_args) -> status;
    #[link_name="switch_ivr_collect_digits_callback"]
    pub fn ivr_collect_digits_callback(session: *mut core_session,
                                       args: *mut input_args,
                                       digit_timeout: u32,
                                       abs_timeout: u32)
                                       -> status;
    #[link_name="switch_ivr_collect_digits_count"]
    pub fn ivr_collect_digits_count(session: *mut core_session,
                                    buf: *mut c_char,
                                    buflen: usize,
                                    maxdigits: usize,
                                    terminators: *const c_char,
                                    terminator: *mut c_char,
                                    first_timeout: u32,
                                    digit_timeout: u32,
                                    abs_timeout: u32)
                                    -> status;
    #[link_name="switch_ivr_play_and_detect_speech"]
    pub fn ivr_play_and_detect_speech(session: *mut core_session,
                                      file: *const c_char,
                                      mod_name: *const c_char,
                                      grammar: *const c_char,
                                      result: *mut *mut c_char,
                                      input_timeout: u32,
                                      args: *mut input_args)
                                      -> status;
    #[link_name="switch_ivr_detect_speech_init"]
    pub fn ivr_detect_speech_init(session: *mut core_session,
                                  mod_name: *const c_char,
                                  dest: *const c_char,
                                  ah: *mut asr_handle)
                                  -> status;
    #[link_name="switch_ivr_detect_speech"]
    pub fn ivr_detect_speech(session: *mut core_session,
                             mod_name: *const c_char,
                             grammar: *const c_char,
                             name: *const c_char,
                             dest: *const c_char,
                             ah: *mut asr_handle)
                             -> status;
    #[link_name="switch_ivr_stop_detect_speech"]
    pub fn ivr_stop_detect_speech(session: *mut core_session) -> status;
    #[link_name="switch_ivr_pause_detect_speech"]
    pub fn ivr_pause_detect_speech(session: *mut core_session) -> status;
    #[link_name="switch_ivr_resume_detect_speech"]
    pub fn ivr_resume_detect_speech(session: *mut core_session) -> status;
    #[link_name="switch_ivr_detect_speech_load_grammar"]
    pub fn ivr_detect_speech_load_grammar(session: *mut core_session,
                                          grammar: *const c_char,
                                          name: *const c_char)
                                          -> status;
    #[link_name="switch_ivr_detect_speech_unload_grammar"]
    pub fn ivr_detect_speech_unload_grammar(session: *mut core_session,
                                            name: *const c_char)
                                            -> status;
    #[link_name="switch_ivr_detect_speech_enable_grammar"]
    pub fn ivr_detect_speech_enable_grammar(session: *mut core_session,
                                            name: *const c_char)
                                            -> status;
    #[link_name="switch_ivr_detect_speech_disable_grammar"]
    pub fn ivr_detect_speech_disable_grammar(session: *mut core_session,
                                             name: *const c_char)
                                             -> status;
    #[link_name="switch_ivr_detect_speech_disable_all_grammars"]
    pub fn ivr_detect_speech_disable_all_grammars(session: *mut core_session) -> status;
    #[link_name="switch_ivr_set_param_detect_speech"]
    pub fn ivr_set_param_detect_speech(session: *mut core_session,
                                       name: *const c_char,
                                       val: *const c_char)
                                       -> status;
    #[link_name="switch_ivr_detect_speech_start_input_timers"]
    pub fn ivr_detect_speech_start_input_timers(session: *mut core_session) -> status;
    #[link_name="switch_ivr_record_session"]
    pub fn ivr_record_session(session: *mut core_session,
                              file: *mut c_char,
                              limit: u32,
                              fh: *mut file_handle)
                              -> status;
    #[link_name="switch_ivr_transfer_recordings"]
    pub fn ivr_transfer_recordings(orig_session: *mut core_session,
                                   new_session: *mut core_session)
                                   -> status;
    #[link_name="switch_ivr_eavesdrop_pop_eavesdropper"]
    pub fn ivr_eavesdrop_pop_eavesdropper(session: *mut core_session,
                                          sessionp: *mut *mut core_session)
                                          -> status;
    #[link_name="switch_ivr_eavesdrop_exec_all"]
    pub fn ivr_eavesdrop_exec_all(session: *mut core_session,
                                  app: *const c_char,
                                  arg: *const c_char)
                                  -> status;
    #[link_name="switch_ivr_eavesdrop_update_display"]
    pub fn ivr_eavesdrop_update_display(session: *mut core_session,
                                        name: *const c_char,
                                        number: *const c_char)
                                        -> status;
    #[link_name="switch_ivr_eavesdrop_session"]
    pub fn ivr_eavesdrop_session(session: *mut core_session,
                                 uuid: *const c_char,
                                 require_group: *const c_char,
                                 flags: eavesdrop_flag)
                                 -> status;
    #[link_name="switch_ivr_displace_session"]
    pub fn ivr_displace_session(session: *mut core_session,
                                file: *const c_char,
                                limit: u32,
                                flags: *const c_char)
                                -> status;
    #[link_name="switch_ivr_stop_displace_session"]
    pub fn ivr_stop_displace_session(session: *mut core_session, file: *const c_char) -> status;
    #[link_name="switch_ivr_stop_record_session"]
    pub fn ivr_stop_record_session(session: *mut core_session, file: *const c_char) -> status;
    #[link_name="switch_ivr_session_audio"]
    pub fn ivr_session_audio(session: *mut core_session,
                             cmd: *const c_char,
                             direction: *const c_char,
                             level: c_int)
                             -> status;
    #[link_name="switch_ivr_stop_session_audio"]
    pub fn ivr_stop_session_audio(session: *mut core_session) -> status;
    #[link_name="switch_ivr_inband_dtmf_session"]
    pub fn ivr_inband_dtmf_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_stop_inband_dtmf_session"]
    pub fn ivr_stop_inband_dtmf_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_inband_dtmf_generate_session"]
    pub fn ivr_inband_dtmf_generate_session(session: *mut core_session,
                                            read_stream: switch_bool)
                                            -> status;
    #[link_name="switch_ivr_stop_inband_dtmf_generate_session"]
    pub fn ivr_stop_inband_dtmf_generate_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_session_echo"]
    pub fn ivr_session_echo(session: *mut core_session, args: *mut input_args) -> status;
    #[link_name="switch_ivr_stop_tone_detect_session"]
    pub fn ivr_stop_tone_detect_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_tone_detect_session"]
    pub fn ivr_tone_detect_session(session: *mut core_session,
                                   key: *const c_char,
                                   tone_spec: *const c_char,
                                   flags: *const c_char,
                                   timeout: time_t,
                                   hits: c_int,
                                   app: *const c_char,
                                   data: *const c_char,
                                   callback: tone_detect_callback)
                                   -> status;
    #[link_name="switch_ivr_play_file"]
    pub fn ivr_play_file(session: *mut core_session,
                         fh: *mut file_handle,
                         file: *const c_char,
                         args: *mut input_args)
                         -> status;
    #[link_name="switch_ivr_wait_for_silence"]
    pub fn ivr_wait_for_silence(session: *mut core_session,
                                thresh: u32,
                                silence_hits: u32,
                                listen_hits: u32,
                                timeout_ms: u32,
                                file: *const c_char)
                                -> status;
    #[link_name="switch_ivr_gentones"]
    pub fn ivr_gentones(session: *mut core_session,
                        script: *const c_char,
                        loops: i32,
                        args: *mut input_args)
                        -> status;
    #[link_name="switch_ivr_record_file"]
    pub fn ivr_record_file(session: *mut core_session,
                           fh: *mut file_handle,
                           file: *const c_char,
                           args: *mut input_args,
                           limit: u32)
                           -> status;
    #[link_name="switch_play_and_get_digits"]
    pub fn play_and_get_digits(session: *mut core_session,
                               min_digits: u32,
                               max_digits: u32,
                               max_tries: u32,
                               timeout: u32,
                               valid_terminators: *const c_char,
                               audio_file: *const c_char,
                               bad_input_audio_file: *const c_char,
                               var_name: *const c_char,
                               digit_buffer: *mut c_char,
                               digit_buffer_length: u32,
                               digits_regex: *const c_char,
                               digit_timeout: u32,
                               transfer_on_failure: *const c_char)
                               -> status;
    #[link_name="switch_ivr_speak_text_handle"]
    pub fn ivr_speak_text_handle(session: *mut core_session,
                                 sh: *mut speech_handle,
                                 codec: *mut codec,
                                 timer: *mut timer,
                                 text: *mut c_char,
                                 args: *mut input_args)
                                 -> status;
    #[link_name="switch_ivr_clear_speech_cache"]
    pub fn ivr_clear_speech_cache(session: *mut core_session);
    #[link_name="switch_ivr_speak_text"]
    pub fn ivr_speak_text(session: *mut core_session,
                          tts_name: *const c_char,
                          voice_name: *const c_char,
                          text: *mut c_char,
                          args: *mut input_args)
                          -> status;
    #[link_name="switch_ivr_originate"]
    pub fn ivr_originate(session: *mut core_session,
                         bleg: *mut *mut core_session,
                         cause: *mut call_cause,
                         bridgeto: *const c_char,
                         timelimit_sec: u32,
                         table: *const state_handler_table,
                         cid_name_override: *const c_char,
                         cid_num_override: *const c_char,
                         caller_profile_override: *mut caller_profile,
                         ovars: *mut event,
                         flags: originate_flag,
                         cancel_cause: *mut call_cause)
                         -> status;
    #[link_name="switch_ivr_enterprise_originate"]
    pub fn ivr_enterprise_originate(session: *mut core_session,
                                    bleg: *mut *mut core_session,
                                    cause: *mut call_cause,
                                    bridgeto: *const c_char,
                                    timelimit_sec: u32,
                                    table: *const state_handler_table,
                                    cid_name_override: *const c_char,
                                    cid_num_override: *const c_char,
                                    caller_profile_override: *mut caller_profile,
                                    ovars: *mut event,
                                    flags: originate_flag,
                                    cancel_cause: *mut call_cause)
                                    -> status;
    #[link_name="switch_ivr_bridge_display"]
    pub fn ivr_bridge_display(session: *mut core_session, peer_session: *mut core_session);
    #[link_name="switch_ivr_multi_threaded_bridge"]
    pub fn ivr_multi_threaded_bridge(session: *mut core_session,
                                     peer_session: *mut core_session,
                                     dtmf_callback: input_callback_function,
                                     session_data: *mut c_void,
                                     peer_session_data: *mut c_void)
                                     -> status;
    #[link_name="switch_ivr_signal_bridge"]
    pub fn ivr_signal_bridge(session: *mut core_session,
                             peer_session: *mut core_session)
                             -> status;
    #[link_name="switch_ivr_session_transfer"]
    pub fn ivr_session_transfer(session: *mut core_session,
                                extension: *const c_char,
                                dialplan: *const c_char,
                                context: *const c_char)
                                -> status;
    #[link_name="switch_ivr_schedule_transfer"]
    pub fn ivr_schedule_transfer(runtime: time_t,
                                 uuid: *const c_char,
                                 extension: *mut c_char,
                                 dialplan: *mut c_char,
                                 context: *mut c_char)
                                 -> u32;
    #[link_name="switch_ivr_schedule_hangup"]
    pub fn ivr_schedule_hangup(runtime: time_t,
                               uuid: *const c_char,
                               cause: call_cause,
                               bleg: switch_bool)
                               -> u32;
    #[link_name="switch_ivr_uuid_bridge"]
    pub fn ivr_uuid_bridge(originator_uuid: *const c_char,
                           originatee_uuid: *const c_char)
                           -> status;
    #[link_name="switch_ivr_media"]
    pub fn ivr_media(uuid: *const c_char, flags: media_flag) -> status;
    #[link_name="switch_ivr_3p_media"]
    pub fn ivr_3p_media(uuid: *const c_char, flags: media_flag) -> status;
    #[link_name="switch_ivr_nomedia"]
    pub fn ivr_nomedia(uuid: *const c_char, flags: media_flag) -> status;
    #[link_name="switch_ivr_3p_nomedia"]
    pub fn ivr_3p_nomedia(uuid: *const c_char, flags: media_flag) -> status;
    #[link_name="switch_ivr_bg_media"]
    pub fn ivr_bg_media(uuid: *const c_char,
                        flags: media_flag,
                        on: switch_bool,
                        is3p: switch_bool,
                        delay: u32);
    #[link_name="switch_ivr_hold_uuid"]
    pub fn ivr_hold_uuid(uuid: *const c_char, message: *const c_char, moh: switch_bool) -> status;
    #[link_name="switch_ivr_hold_toggle_uuid"]
    pub fn ivr_hold_toggle_uuid(uuid: *const c_char,
                                message: *const c_char,
                                moh: switch_bool)
                                -> status;
    #[link_name="switch_ivr_unhold_uuid"]
    pub fn ivr_unhold_uuid(uuid: *const c_char) -> status;
    #[link_name="switch_ivr_hold"]
    pub fn ivr_hold(session: *mut core_session,
                    message: *const c_char,
                    moh: switch_bool)
                    -> status;
    #[link_name="switch_ivr_unhold"]
    pub fn ivr_unhold(session: *mut core_session) -> status;
    #[link_name="switch_ivr_schedule_broadcast"]
    pub fn ivr_schedule_broadcast(runtime: time_t,
                                  uuid: *const c_char,
                                  path: *const c_char,
                                  flags: media_flag)
                                  -> u32;
    #[link_name="switch_ivr_broadcast"]
    pub fn ivr_broadcast(uuid: *const c_char, path: *const c_char, flags: media_flag) -> status;
    #[link_name="switch_ivr_broadcast_in_thread"]
    pub fn ivr_broadcast_in_thread(session: *mut core_session, app: *const c_char, flags: c_int);
    #[link_name="switch_ivr_transfer_variable"]
    pub fn ivr_transfer_variable(sessa: *mut core_session,
                                 sessb: *mut core_session,
                                 var: *mut c_char)
                                 -> status;
    #[link_name="switch_ivr_digit_stream_parser_new"]
    pub fn ivr_digit_stream_parser_new(pool: *mut memory_pool,
                                       parser: *mut *mut ivr_digit_stream_parser)
                                       -> status;
    #[link_name="switch_ivr_digit_stream_parser_destroy"]
    pub fn ivr_digit_stream_parser_destroy(parser: *mut ivr_digit_stream_parser) -> status;
    #[link_name="switch_ivr_digit_stream_new"]
    pub fn ivr_digit_stream_new(parser: *mut ivr_digit_stream_parser,
                                stream: *mut *mut ivr_digit_stream)
                                -> status;
    #[link_name="switch_ivr_digit_stream_destroy"]
    pub fn ivr_digit_stream_destroy(stream: *mut *mut ivr_digit_stream) -> status;
    #[link_name="switch_ivr_digit_stream_parser_set_event"]
    pub fn ivr_digit_stream_parser_set_event(parser: *mut ivr_digit_stream_parser,
                                             digits: *mut c_char,
                                             data: *mut c_void)
                                             -> status;
    #[link_name="switch_ivr_digit_stream_parser_del_event"]
    pub fn ivr_digit_stream_parser_del_event(parser: *mut ivr_digit_stream_parser,
                                             digits: *mut c_char)
                                             -> status;
    #[link_name="switch_ivr_digit_stream_parser_feed"]
    pub fn ivr_digit_stream_parser_feed(parser: *mut ivr_digit_stream_parser,
                                        stream: *mut ivr_digit_stream,
                                        digit: c_char)
                                        -> *mut c_void;
    #[link_name="switch_ivr_digit_stream_reset"]
    pub fn ivr_digit_stream_reset(stream: *mut ivr_digit_stream) -> status;
    #[link_name="switch_ivr_digit_stream_parser_set_terminator"]
    pub fn ivr_digit_stream_parser_set_terminator(parser: *mut ivr_digit_stream_parser,
                                                  digit: c_char)
                                                  -> status;
    #[link_name="switch_ivr_menu_init"]
    pub fn ivr_menu_init(new_menu: *mut *mut ivr_menu,
                         main: *mut ivr_menu,
                         name: *const c_char,
                         greeting_sound: *const c_char,
                         short_greeting_sound: *const c_char,
                         invalid_sound: *const c_char,
                         exit_sound: *const c_char,
                         transfer_sound: *const c_char,
                         confirm_macro: *const c_char,
                         confirm_key: *const c_char,
                         tts_engine: *const c_char,
                         tts_voice: *const c_char,
                         confirm_attempts: c_int,
                         inter_timeout: c_int,
                         digit_len: c_int,
                         timeout: c_int,
                         max_failures: c_int,
                         max_timeouts: c_int,
                         pool: *mut memory_pool)
                         -> status;
    #[link_name="switch_ivr_menu_bind_action"]
    pub fn ivr_menu_bind_action(menu: *mut ivr_menu,
                                ivr_action: ivr_action,
                                arg: *const c_char,
                                bind: *const c_char)
                                -> status;
    #[link_name="switch_ivr_menu_bind_function"]
    pub fn ivr_menu_bind_function(menu: *mut ivr_menu,
                                  function: *mut ivr_menu_action_function,
                                  arg: *const c_char,
                                  bind: *const c_char)
                                  -> status;
    #[link_name="switch_ivr_menu_execute"]
    pub fn ivr_menu_execute(session: *mut core_session,
                            stack: *mut ivr_menu,
                            name: *mut c_char,
                            obj: *mut c_void)
                            -> status;
    #[link_name="switch_ivr_menu_stack_free"]
    pub fn ivr_menu_stack_free(stack: *mut ivr_menu) -> status;
    #[link_name="switch_ivr_menu_stack_xml_build"]
    pub fn ivr_menu_stack_xml_build(xml_menu_ctx: *mut ivr_menu_xml_ctx,
                                    menu_stack: *mut *mut ivr_menu,
                                    xml_menus: xml_t,
                                    xml_menu: xml_t)
                                    -> status;
    #[link_name="switch_ivr_menu_str2action"]
    pub fn ivr_menu_str2action(action_name: *const c_char, action: *mut ivr_action) -> status;
    #[link_name="switch_ivr_menu_stack_xml_add_custom"]
    pub fn ivr_menu_stack_xml_add_custom(xml_menu_ctx: *mut ivr_menu_xml_ctx,
                                         name: *const c_char,
                                         function: *mut ivr_menu_action_function)
                                         -> status;
    #[link_name="switch_ivr_menu_stack_xml_init"]
    pub fn ivr_menu_stack_xml_init(xml_menu_ctx: *mut *mut ivr_menu_xml_ctx,
                                   pool: *mut memory_pool)
                                   -> status;
    #[link_name="switch_ivr_phrase_macro_event"]
    pub fn ivr_phrase_macro_event(session: *mut core_session,
                                  macro_name: *const c_char,
                                  data: *const c_char,
                                  event: *mut event,
                                  lang: *const c_char,
                                  args: *mut input_args)
                                  -> status;
    #[link_name="switch_ivr_delay_echo"]
    pub fn ivr_delay_echo(session: *mut core_session, delay_ms: u32);
    #[link_name="switch_ivr_find_bridged_uuid"]
    pub fn ivr_find_bridged_uuid(uuid: *const c_char, b_uuid: *mut c_char, blen: usize) -> status;
    #[link_name="switch_ivr_intercept_session"]
    pub fn ivr_intercept_session(session: *mut core_session,
                                 uuid: *const c_char,
                                 bleg: switch_bool);
    #[link_name="switch_ivr_park_session"]
    pub fn ivr_park_session(session: *mut core_session);
    #[link_name="switch_ivr_wait_for_answer"]
    pub fn ivr_wait_for_answer(session: *mut core_session,
                               peer_session: *mut core_session)
                               -> status;
    #[link_name="switch_ivr_read"]
    pub fn ivr_read(session: *mut core_session,
                    min_digits: u32,
                    max_digits: u32,
                    prompt_audio_file: *const c_char,
                    var_name: *const c_char,
                    digit_buffer: *mut c_char,
                    digit_buffer_length: usize,
                    timeout: u32,
                    valid_terminators: *const c_char,
                    digit_timeout: u32)
                    -> status;
    #[link_name="switch_ivr_block_dtmf_session"]
    pub fn ivr_block_dtmf_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_unblock_dtmf_session"]
    pub fn ivr_unblock_dtmf_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_bind_dtmf_meta_session"]
    pub fn ivr_bind_dtmf_meta_session(session: *mut core_session,
                                      key: u32,
                                      bind_flags: bind_flag,
                                      app: *const c_char)
                                      -> status;
    #[link_name="switch_ivr_unbind_dtmf_meta_session"]
    pub fn ivr_unbind_dtmf_meta_session(session: *mut core_session, key: u32) -> status;
    #[link_name="switch_ivr_soft_hold"]
    pub fn ivr_soft_hold(session: *mut core_session,
                         unhold_key: *const c_char,
                         moh_a: *const c_char,
                         moh_b: *const c_char)
                         -> status;
    #[link_name="switch_ivr_say"]
    pub fn ivr_say(session: *mut core_session,
                   tosay: *const c_char,
                   module_name: *const c_char,
                   say_type: *const c_char,
                   say_method: *const c_char,
                   say_gender: *const c_char,
                   args: *mut input_args)
                   -> status;
    #[link_name="switch_ivr_say_string"]
    pub fn ivr_say_string(session: *mut core_session,
                          lang: *const c_char,
                          ext: *const c_char,
                          tosay: *const c_char,
                          module_name: *const c_char,
                          say_type: *const c_char,
                          say_method: *const c_char,
                          say_gender: *const c_char,
                          rstr: *mut *mut c_char)
                          -> status;
    #[link_name="switch_ivr_get_say_method_by_name"]
    pub fn ivr_get_say_method_by_name(name: *const c_char) -> say_method;
    #[link_name="switch_ivr_get_say_gender_by_name"]
    pub fn ivr_get_say_gender_by_name(name: *const c_char) -> say_gender;
    #[link_name="switch_ivr_get_say_type_by_name"]
    pub fn ivr_get_say_type_by_name(name: *const c_char) -> say_type;
    #[link_name="switch_ivr_say_spell"]
    pub fn ivr_say_spell(session: *mut core_session,
                         tosay: *mut c_char,
                         say_args: *mut say_args,
                         args: *mut input_args)
                         -> status;
    #[link_name="switch_ivr_say_ip"]
    pub fn ivr_say_ip(session: *mut core_session,
                      tosay: *mut c_char,
                      number_func: say_callback,
                      say_args: *mut say_args,
                      args: *mut input_args)
                      -> status;
    #[link_name="switch_ivr_set_user"]
    pub fn ivr_set_user(session: *mut core_session, data: *const c_char) -> status;
    #[link_name="switch_ivr_set_user_xml"]
    pub fn ivr_set_user_xml(session: *mut core_session,
                            prefix: *const c_char,
                            user: *const c_char,
                            domain: *const c_char,
                            x_user: xml_t)
                            -> status;
    #[link_name="switch_ivr_sound_test"]
    pub fn ivr_sound_test(session: *mut core_session) -> status;
    #[link_name="switch_process_import"]
    pub fn process_import(session: *mut core_session,
                          peer_channel: *mut channel,
                          varname: *const c_char,
                          prefix: *const c_char);
    #[link_name="switch_ivr_uuid_exists"]
    pub fn ivr_uuid_exists(uuid: *const c_char) -> switch_bool;
    #[link_name="switch_ivr_uuid_force_exists"]
    pub fn ivr_uuid_force_exists(uuid: *const c_char) -> switch_bool;
    #[link_name="switch_ivr_dmachine_is_parsing"]
    pub fn ivr_dmachine_is_parsing(dmachine: *mut ivr_dmachine) -> switch_bool;
    #[link_name="switch_ivr_dmachine_last_ping"]
    pub fn ivr_dmachine_last_ping(dmachine: *mut ivr_dmachine) -> status;
    #[link_name="switch_ivr_dmachine_get_name"]
    pub fn ivr_dmachine_get_name(dmachine: *mut ivr_dmachine) -> *const c_char;
    #[link_name="switch_ivr_dmachine_set_match_callback"]
    pub fn ivr_dmachine_set_match_callback(dmachine: *mut ivr_dmachine,
                                           match_callback: ivr_dmachine_callback);
    #[link_name="switch_ivr_dmachine_set_nonmatch_callback"]
    pub fn ivr_dmachine_set_nonmatch_callback(dmachine: *mut ivr_dmachine,
                                              nonmatch_callback: ivr_dmachine_callback);
    #[link_name="switch_ivr_dmachine_create"]
    pub fn ivr_dmachine_create(dmachine_p: *mut *mut ivr_dmachine,
                               name: *const c_char,
                               pool: *mut memory_pool,
                               digit_timeout: u32,
                               input_timeout: u32,
                               match_callback: ivr_dmachine_callback,
                               nonmatch_callback: ivr_dmachine_callback,
                               user_data: *mut c_void)
                               -> status;
    #[link_name="switch_ivr_dmachine_destroy"]
    pub fn ivr_dmachine_destroy(dmachine: *mut *mut ivr_dmachine);
    #[link_name="switch_ivr_dmachine_bind"]
    pub fn ivr_dmachine_bind(dmachine: *mut ivr_dmachine,
                             realm: *const c_char,
                             digits: *const c_char,
                             key: i32,
                             callback: ivr_dmachine_callback,
                             user_data: *mut c_void)
                             -> status;
    #[link_name="switch_ivr_dmachine_feed"]
    pub fn ivr_dmachine_feed(dmachine: *mut ivr_dmachine,
                             digits: *const c_char,
                             match_: *mut *mut ivr_dmachine_match)
                             -> status;
    #[link_name="switch_ivr_dmachine_clear"]
    pub fn ivr_dmachine_clear(dmachine: *mut ivr_dmachine) -> status;
    #[link_name="switch_ivr_dmachine_ping"]
    pub fn ivr_dmachine_ping(dmachine: *mut ivr_dmachine,
                             match_p: *mut *mut ivr_dmachine_match)
                             -> status;
    #[link_name="switch_ivr_dmachine_get_match"]
    pub fn ivr_dmachine_get_match(dmachine: *mut ivr_dmachine) -> *mut ivr_dmachine_match;
    #[link_name="switch_ivr_dmachine_get_failed_digits"]
    pub fn ivr_dmachine_get_failed_digits(dmachine: *mut ivr_dmachine) -> *const c_char;
    #[link_name="switch_ivr_dmachine_set_digit_timeout_ms"]
    pub fn ivr_dmachine_set_digit_timeout_ms(dmachine: *mut ivr_dmachine, digit_timeout_ms: u32);
    #[link_name="switch_ivr_dmachine_set_input_timeout_ms"]
    pub fn ivr_dmachine_set_input_timeout_ms(dmachine: *mut ivr_dmachine, input_timeout_ms: u32);
    #[link_name="switch_ivr_dmachine_clear_realm"]
    pub fn ivr_dmachine_clear_realm(dmachine: *mut ivr_dmachine, realm: *const c_char) -> status;
    #[link_name="switch_ivr_dmachine_set_realm"]
    pub fn ivr_dmachine_set_realm(dmachine: *mut ivr_dmachine, realm: *const c_char) -> status;
    #[link_name="switch_ivr_get_file_handle"]
    pub fn ivr_get_file_handle(session: *mut core_session, fh: *mut *mut file_handle) -> status;
    #[link_name="switch_ivr_release_file_handle"]
    pub fn ivr_release_file_handle(session: *mut core_session,
                                   fh: *mut *mut file_handle)
                                   -> status;
    #[link_name="switch_ivr_process_fh"]
    pub fn ivr_process_fh(session: *mut core_session,
                          cmd: *const c_char,
                          fhp: *mut file_handle)
                          -> status;
    #[link_name="switch_ivr_insert_file"]
    pub fn ivr_insert_file(session: *mut core_session,
                           file: *const c_char,
                           insert_file: *const c_char,
                           sample_point: usize)
                           -> status;
    #[link_name="switch_ivr_create_message_reply"]
    pub fn ivr_create_message_reply(reply: *mut *mut event,
                                    message: *mut event,
                                    new_proto: *const c_char)
                                    -> status;
    #[link_name="switch_ivr_check_presence_mapping"]
    pub fn ivr_check_presence_mapping(exten_name: *const c_char,
                                      domain_name: *const c_char)
                                      -> *mut c_char;
    #[link_name="switch_ivr_kill_uuid"]
    pub fn ivr_kill_uuid(uuid: *const c_char, cause: call_cause) -> status;
    #[link_name="switch_ivr_blind_transfer_ack"]
    pub fn ivr_blind_transfer_ack(session: *mut core_session, success: switch_bool) -> status;
    #[link_name="switch_ivr_record_session_mask"]
    pub fn ivr_record_session_mask(session: *mut core_session,
                                   file: *const c_char,
                                   on: switch_bool)
                                   -> status;
    #[link_name="switch_ivr_stop_video_write_overlay_session"]
    pub fn ivr_stop_video_write_overlay_session(session: *mut core_session) -> status;
    #[link_name="switch_ivr_video_write_overlay_session"]
    pub fn ivr_video_write_overlay_session(session: *mut core_session,
                                           img_path: *const c_char,
                                           pos: img_position,
                                           alpha: u8)
                                           -> status;
    #[link_name="switch_rtp_add_crypto_key"]
    pub fn rtp_add_crypto_key(rtp_session: *mut rtp,
                              direction: rtp_crypto_direction,
                              index: u32,
                              type_: rtp_crypto_key_type,
                              key: *mut c_uchar,
                              keylen: usize)
                              -> status;
    #[link_name="switch_rtp_get_random"]
    pub fn rtp_get_random(buf: *mut c_void, len: u32);
    #[link_name="switch_rtp_init"]
    pub fn rtp_init(pool: *mut memory_pool);
    #[link_name="switch_rtp_shutdown"]
    pub fn rtp_shutdown();
    #[link_name="switch_rtp_set_start_port"]
    pub fn rtp_set_start_port(port: port) -> port;
    #[link_name="switch_rtp_set_ssrc"]
    pub fn rtp_set_ssrc(rtp_session: *mut rtp, ssrc: u32) -> status;
    #[link_name="switch_rtp_set_remote_ssrc"]
    pub fn rtp_set_remote_ssrc(rtp_session: *mut rtp, ssrc: u32) -> status;
    #[link_name="switch_rtp_set_end_port"]
    pub fn rtp_set_end_port(port: port) -> port;
    #[link_name="switch_rtp_request_port"]
    pub fn rtp_request_port(ip: *const c_char) -> port;
    #[link_name="switch_rtp_release_port"]
    pub fn rtp_release_port(ip: *const c_char, port: port);
    #[link_name="switch_rtp_set_interval"]
    pub fn rtp_set_interval(rtp_session: *mut rtp,
                            ms_per_packet: u32,
                            samples_per_interval: u32)
                            -> status;
    #[link_name="switch_rtp_change_interval"]
    pub fn rtp_change_interval(rtp_session: *mut rtp,
                               ms_per_packet: u32,
                               samples_per_interval: u32)
                               -> status;
    #[link_name="switch_rtp_create"]
    pub fn rtp_create(new_rtp_session: *mut *mut rtp,
                      payload: payload,
                      samples_per_interval: u32,
                      ms_per_packet: u32,
                      flags: *mut rtp_flag,
                      timer_name: *mut c_char,
                      err: *mut *const c_char,
                      pool: *mut memory_pool)
                      -> status;
    #[link_name="switch_rtp_new"]
    pub fn rtp_new(rx_host: *const c_char,
                   rx_port: port,
                   tx_host: *const c_char,
                   tx_port: port,
                   payload: payload,
                   samples_per_interval: u32,
                   ms_per_packet: u32,
                   flags: *mut rtp_flag,
                   timer_name: *mut c_char,
                   err: *mut *const c_char,
                   pool: *mut memory_pool)
                   -> *mut rtp;
    #[link_name="switch_rtp_set_remote_address"]
    pub fn rtp_set_remote_address(rtp_session: *mut rtp,
                                  host: *const c_char,
                                  port: port,
                                  remote_rtcp_port: port,
                                  change_adv_addr: switch_bool,
                                  err: *mut *const c_char)
                                  -> status;
    #[link_name="switch_rtp_reset_jb"]
    pub fn rtp_reset_jb(rtp_session: *mut rtp);
    #[link_name="switch_rtp_get_remote_host"]
    pub fn rtp_get_remote_host(rtp_session: *mut rtp) -> *mut c_char;
    #[link_name="switch_rtp_get_remote_port"]
    pub fn rtp_get_remote_port(rtp_session: *mut rtp) -> port;
    #[link_name="switch_rtp_reset_media_timer"]
    pub fn rtp_reset_media_timer(rtp_session: *mut rtp);
    #[link_name="switch_rtp_set_max_missed_packets"]
    pub fn rtp_set_max_missed_packets(rtp_session: *mut rtp, max: u32);
    #[link_name="switch_rtp_udptl_mode"]
    pub fn rtp_udptl_mode(rtp_session: *mut rtp) -> status;
    #[link_name="switch_rtp_reset"]
    pub fn rtp_reset(rtp_session: *mut rtp);
    #[link_name="switch_rtp_set_local_address"]
    pub fn rtp_set_local_address(rtp_session: *mut rtp,
                                 host: *const c_char,
                                 port: port,
                                 err: *mut *const c_char)
                                 -> status;
    #[link_name="switch_rtp_kill_socket"]
    pub fn rtp_kill_socket(rtp_session: *mut rtp);
    #[link_name="switch_rtp_break"]
    pub fn rtp_break(rtp_session: *mut rtp);
    #[link_name="switch_rtp_flush"]
    pub fn rtp_flush(rtp_session: *mut rtp);
    #[link_name="switch_rtp_ready"]
    pub fn rtp_ready(rtp_session: *mut rtp) -> u8;
    #[link_name="switch_rtp_destroy"]
    pub fn rtp_destroy(rtp_session: *mut *mut rtp);
    #[link_name="switch_rtp_sync_stats"]
    pub fn rtp_sync_stats(rtp_session: *mut rtp) -> status;
    #[link_name="switch_rtp_activate_ice"]
    pub fn rtp_activate_ice(rtp_session: *mut rtp,
                            login: *mut c_char,
                            rlogin: *mut c_char,
                            password: *const c_char,
                            rpassword: *const c_char,
                            proto: ice_proto,
                            type_: core_media_ice_type,
                            ice_params: *mut ice)
                            -> status;
    #[link_name="switch_rtp_activate_rtcp"]
    pub fn rtp_activate_rtcp(rtp_session: *mut rtp,
                             send_rate: c_int,
                             remote_port: port,
                             mux: switch_bool)
                             -> status;
    #[link_name="switch_rtp_get_media_timer"]
    pub fn rtp_get_media_timer(rtp_session: *mut rtp) -> *mut timer;
    #[link_name="switch_rtp_set_video_buffer_size"]
    pub fn rtp_set_video_buffer_size(rtp_session: *mut rtp,
                                     frames: u32,
                                     max_frames: u32)
                                     -> status;
    #[link_name="switch_rtp_get_video_buffer_size"]
    pub fn rtp_get_video_buffer_size(rtp_session: *mut rtp,
                                     min_frame_len: *mut u32,
                                     max_frame_len: *mut u32,
                                     cur_frame_len: *mut u32,
                                     highest_frame_len: *mut u32)
                                     -> status;
    #[link_name="switch_rtp_activate_jitter_buffer"]
    pub fn rtp_activate_jitter_buffer(rtp_session: *mut rtp,
                                      queue_frames: u32,
                                      max_queue_frames: u32,
                                      samples_per_packet: u32,
                                      samples_per_second: u32)
                                      -> status;
    #[link_name="switch_rtp_debug_jitter_buffer"]
    pub fn rtp_debug_jitter_buffer(rtp_session: *mut rtp, name: *const c_char) -> status;
    #[link_name="switch_rtp_deactivate_jitter_buffer"]
    pub fn rtp_deactivate_jitter_buffer(rtp_session: *mut rtp) -> status;
    #[link_name="switch_rtp_pause_jitter_buffer"]
    pub fn rtp_pause_jitter_buffer(rtp_session: *mut rtp, pause: switch_bool) -> status;
    #[link_name="switch_rtp_get_jitter_buffer"]
    pub fn rtp_get_jitter_buffer(rtp_session: *mut rtp) -> *mut jb;
    #[link_name="switch_rtp_set_flag"]
    pub fn rtp_set_flag(rtp_session: *mut rtp, flag: rtp_flag);
    #[link_name="switch_rtp_set_flags"]
    pub fn rtp_set_flags(rtp_session: *mut rtp, flags: *mut rtp_flag);
    #[link_name="switch_rtp_clear_flags"]
    pub fn rtp_clear_flags(rtp_session: *mut rtp, flags: *mut rtp_flag);
    #[link_name="switch_rtp_test_flag"]
    pub fn rtp_test_flag(rtp_session: *mut rtp, flags: rtp_flag) -> u32;
    #[link_name="switch_rtp_clear_flag"]
    pub fn rtp_clear_flag(rtp_session: *mut rtp, flag: rtp_flag);
    #[link_name="switch_rtp_get_rtp_socket"]
    pub fn rtp_get_rtp_socket(rtp_session: *mut rtp) -> *mut socket;
    #[link_name="switch_rtp_ping"]
    pub fn rtp_ping(rtp_session: *mut rtp);
    #[link_name="switch_rtp_get_default_samples_per_interval"]
    pub fn rtp_get_default_samples_per_interval(rtp_session: *mut rtp) -> u32;
    #[link_name="switch_rtp_set_default_payload"]
    pub fn rtp_set_default_payload(rtp_session: *mut rtp, payload: payload);
    #[link_name="switch_rtp_get_default_payload"]
    pub fn rtp_get_default_payload(rtp_session: *mut rtp) -> u32;
    #[link_name="switch_rtp_set_invalid_handler"]
    pub fn rtp_set_invalid_handler(rtp_session: *mut rtp, on_invalid: rtp_invalid_handler);
    #[link_name="switch_rtp_read"]
    pub fn rtp_read(rtp_session: *mut rtp,
                    data: *mut c_void,
                    datalen: *mut u32,
                    payload_type: *mut payload,
                    flags: *mut frame_flag,
                    io_flags: io_flag)
                    -> status;
    #[link_name="switch_rtp_queue_rfc2833"]
    pub fn rtp_queue_rfc2833(rtp_session: *mut rtp, dtmf: *const dtmf) -> status;
    #[link_name="switch_rtp_queue_rfc2833_in"]
    pub fn rtp_queue_rfc2833_in(rtp_session: *mut rtp, dtmf: *const dtmf) -> status;
    #[link_name="switch_rtp_has_dtmf"]
    pub fn rtp_has_dtmf(rtp_session: *mut rtp) -> usize;
    #[link_name="switch_rtp_dequeue_dtmf"]
    pub fn rtp_dequeue_dtmf(rtp_session: *mut rtp, dtmf: *mut dtmf) -> usize;
    #[link_name="switch_rtp_zerocopy_read"]
    pub fn rtp_zerocopy_read(rtp_session: *mut rtp,
                             data: *mut *mut c_void,
                             datalen: *mut u32,
                             payload_type: *mut payload,
                             flags: *mut frame_flag,
                             io_flags: io_flag)
                             -> status;
    #[link_name="switch_rtp_zerocopy_read_frame"]
    pub fn rtp_zerocopy_read_frame(rtp_session: *mut rtp,
                                   frame: *mut frame,
                                   io_flags: io_flag)
                                   -> status;
    #[link_name="switch_rtcp_zerocopy_read_frame"]
    pub fn rtcp_zerocopy_read_frame(rtp_session: *mut rtp, frame: *mut rtcp_frame) -> status;
    pub fn rtp_flush_read_buffer(rtp_session: *mut rtp, flush: rtp_flush);
    #[link_name="switch_rtp_enable_vad"]
    pub fn rtp_enable_vad(rtp_session: *mut rtp,
                          session: *mut core_session,
                          codec: *mut codec,
                          flags: vad_flag)
                          -> status;
    #[link_name="switch_rtp_disable_vad"]
    pub fn rtp_disable_vad(rtp_session: *mut rtp) -> status;
    #[link_name="switch_rtp_write_frame"]
    pub fn rtp_write_frame(rtp_session: *mut rtp, frame: *mut frame) -> c_int;
    #[link_name="switch_rtp_write_manual"]
    pub fn rtp_write_manual(rtp_session: *mut rtp,
                            data: *mut c_void,
                            datalen: u32,
                            m: u8,
                            payload: payload,
                            ts: u32,
                            flags: *mut frame_flag)
                            -> c_int;
    #[link_name="switch_rtp_write_raw"]
    pub fn rtp_write_raw(rtp_session: *mut rtp,
                         data: *mut c_void,
                         bytes: *mut usize,
                         process_encryption: switch_bool)
                         -> status;
    #[link_name="switch_rtp_get_ssrc"]
    pub fn rtp_get_ssrc(rtp_session: *mut rtp) -> u32;
    #[link_name="switch_rtp_set_private"]
    pub fn rtp_set_private(rtp_session: *mut rtp, private_data: *mut c_void);
    #[link_name="switch_rtp_set_telephony_event"]
    pub fn rtp_set_telephony_event(rtp_session: *mut rtp, te: payload);
    #[link_name="switch_rtp_set_telephony_recv_event"]
    pub fn rtp_set_telephony_recv_event(rtp_session: *mut rtp, te: payload);
    #[link_name="switch_rtp_set_cng_pt"]
    pub fn rtp_set_cng_pt(rtp_session: *mut rtp, pt: payload);
    #[link_name="switch_rtp_get_private"]
    pub fn rtp_get_private(rtp_session: *mut rtp) -> *mut c_void;
    #[link_name="switch_rtp_set_payload_map"]
    pub fn rtp_set_payload_map(rtp_session: *mut rtp, pmap: *mut *mut payload_map) -> status;
    #[link_name="switch_rtp_intentional_bugs"]
    pub fn rtp_intentional_bugs(rtp_session: *mut rtp, bugs: rtp_bug_flag);
    #[link_name="switch_rtp_get_stats"]
    pub fn rtp_get_stats(rtp_session: *mut rtp, pool: *mut memory_pool) -> *mut rtp_stats;
    #[link_name="switch_rtp_check_auto_adj"]
    pub fn rtp_check_auto_adj(rtp_session: *mut rtp) -> u8;
    #[link_name="switch_rtp_set_interdigit_delay"]
    pub fn rtp_set_interdigit_delay(rtp_session: *mut rtp, delay: u32);
    #[link_name="switch_rtp_add_dtls"]
    pub fn rtp_add_dtls(rtp_session: *mut rtp,
                        local_fp: *mut dtls_fingerprint,
                        remote_fp: *mut dtls_fingerprint,
                        type_: dtls_type)
                        -> status;
    #[link_name="switch_rtp_del_dtls"]
    pub fn rtp_del_dtls(rtp_session: *mut rtp, type_: dtls_type) -> status;
    #[link_name="switch_rtp_dtls_state"]
    pub fn rtp_dtls_state(rtp_session: *mut rtp, type_: dtls_type) -> dtls_state;
    #[link_name="switch_rtp_has_dtls"]
    pub fn rtp_has_dtls() -> c_int;
    #[link_name="switch_rtp_req_bitrate"]
    pub fn rtp_req_bitrate(rtp_session: *mut rtp, bps: u32) -> status;
    #[link_name="switch_rtp_ack_bitrate"]
    pub fn rtp_ack_bitrate(rtp_session: *mut rtp, bps: u32) -> status;
    #[link_name="switch_rtp_video_refresh"]
    pub fn rtp_video_refresh(rtp_session: *mut rtp);
    #[link_name="switch_rtp_video_loss"]
    pub fn rtp_video_loss(rtp_session: *mut rtp);
    #[link_name="switch_xml_parse_str_dynamic"]
    pub fn xml_parse_str_dynamic(s: *mut c_char, dup: switch_bool) -> xml_t;
    #[link_name="switch_xml_parse_str"]
    pub fn xml_parse_str(s: *mut c_char, len: usize) -> xml_t;
    #[link_name="switch_xml_parse_fd"]
    pub fn xml_parse_fd(fd: c_int) -> xml_t;
    #[link_name="switch_xml_parse_file"]
    pub fn xml_parse_file(file: *const c_char) -> xml_t;
    #[link_name="switch_xml_parse_file_simple"]
    pub fn xml_parse_file_simple(file: *const c_char) -> xml_t;
    #[link_name="switch_xml_parse_fp"]
    pub fn xml_parse_fp(fp: *mut FILE) -> xml_t;
    #[link_name="switch_xml_child"]
    pub fn xml_child(xml: xml_t, name: *const c_char) -> xml_t;
    #[link_name="switch_xml_find_child"]
    pub fn xml_find_child(node: xml_t,
                          childname: *const c_char,
                          attrname: *const c_char,
                          value: *const c_char)
                          -> xml_t;
    #[link_name="switch_xml_find_child_multi"]
    pub fn xml_find_child_multi(node: xml_t, childname: *const c_char, ...) -> xml_t;
    #[link_name="switch_xml_idx"]
    pub fn xml_idx(xml: xml_t, idx: c_int) -> xml_t;
    #[link_name="switch_xml_attr"]
    pub fn xml_attr(xml: xml_t, attr: *const c_char) -> *const c_char;
    #[link_name="switch_xml_attr_soft"]
    pub fn xml_attr_soft(xml: xml_t, attr: *const c_char) -> *const c_char;
    #[link_name="switch_xml_get"]
    pub fn xml_get(xml: xml_t, ...) -> xml_t;
    #[link_name="switch_xml_toxml"]
    pub fn xml_toxml(xml: xml_t, prn_header: switch_bool) -> *mut c_char;
    #[link_name="switch_xml_toxml_nolock"]
    pub fn xml_toxml_nolock(xml: xml_t, prn_header: switch_bool) -> *mut c_char;
    #[link_name="switch_xml_tohtml"]
    pub fn xml_tohtml(xml: xml_t, prn_header: switch_bool) -> *mut c_char;
    #[link_name="switch_xml_toxml_buf"]
    pub fn xml_toxml_buf(xml: xml_t,
                         buf: *mut c_char,
                         buflen: usize,
                         offset: usize,
                         prn_header: switch_bool)
                         -> *mut c_char;
    #[link_name="switch_xml_pi"]
    pub fn xml_pi(xml: xml_t, target: *const c_char) -> *mut *const c_char;
    #[link_name="switch_xml_free"]
    pub fn xml_free(xml: xml_t);
    #[link_name="switch_xml_free_in_thread"]
    pub fn xml_free_in_thread(xml: xml_t, stacksize: c_int);
    #[link_name="switch_xml_error"]
    pub fn xml_error(xml: xml_t) -> *const c_char;
    #[link_name="switch_xml_new"]
    pub fn xml_new(name: *const c_char) -> xml_t;
    #[link_name="switch_xml_add_child"]
    pub fn xml_add_child(xml: xml_t, name: *const c_char, off: usize) -> xml_t;
    #[link_name="switch_xml_set_txt"]
    pub fn xml_set_txt(xml: xml_t, txt: *const c_char) -> xml_t;
    #[link_name="switch_xml_set_attr"]
    pub fn xml_set_attr(xml: xml_t, name: *const c_char, value: *const c_char) -> xml_t;
    #[link_name="switch_xml_set_flag"]
    pub fn xml_set_flag(xml: xml_t, flag: xml_flag) -> xml_t;
    #[link_name="switch_xml_cut"]
    pub fn xml_cut(xml: xml_t) -> xml_t;
    #[link_name="switch_xml_insert"]
    pub fn xml_insert(xml: xml_t, dest: xml_t, off: usize) -> xml_t;
    #[link_name="switch_xml_set_root"]
    pub fn xml_set_root(new_main: xml_t) -> status;
    #[link_name="switch_xml_set_open_root_function"]
    pub fn xml_set_open_root_function(func: xml_open_root_function,
                                      user_data: *mut c_void)
                                      -> status;
    #[link_name="switch_xml_open_root"]
    pub fn xml_open_root(reload: u8, err: *mut *const c_char) -> xml_t;
    #[link_name="switch_xml_init"]
    pub fn xml_init(pool: *mut memory_pool, err: *mut *const c_char) -> status;
    #[link_name="switch_xml_reload"]
    pub fn xml_reload(err: *mut *const c_char) -> status;
    #[link_name="switch_xml_destroy"]
    pub fn xml_destroy() -> status;
    #[link_name="switch_xml_root"]
    pub fn xml_root() -> xml_t;
    #[link_name="switch_xml_locate"]
    pub fn xml_locate(section: *const c_char,
                      tag_name: *const c_char,
                      key_name: *const c_char,
                      key_value: *const c_char,
                      root: *mut xml_t,
                      node: *mut xml_t,
                      params: *mut event,
                      clone: switch_bool)
                      -> status;
    #[link_name="switch_xml_locate_domain"]
    pub fn xml_locate_domain(domain_name: *const c_char,
                             params: *mut event,
                             root: *mut xml_t,
                             domain: *mut xml_t)
                             -> status;
    #[link_name="switch_xml_locate_group"]
    pub fn xml_locate_group(group_name: *const c_char,
                            domain_name: *const c_char,
                            root: *mut xml_t,
                            domain: *mut xml_t,
                            group: *mut xml_t,
                            params: *mut event)
                            -> status;
    #[link_name="switch_xml_locate_user"]
    pub fn xml_locate_user(key: *const c_char,
                           user_name: *const c_char,
                           domain_name: *const c_char,
                           ip: *const c_char,
                           root: *mut xml_t,
                           domain: *mut xml_t,
                           user: *mut xml_t,
                           ingroup: *mut xml_t,
                           params: *mut event)
                           -> status;
    #[link_name="switch_xml_locate_user_in_domain"]
    pub fn xml_locate_user_in_domain(user_name: *const c_char,
                                     domain: xml_t,
                                     user: *mut xml_t,
                                     ingroup: *mut xml_t)
                                     -> status;
    #[link_name="switch_xml_locate_user_merged"]
    pub fn xml_locate_user_merged(key: *const c_char,
                                  user_name: *const c_char,
                                  domain_name: *const c_char,
                                  ip: *const c_char,
                                  user: *mut xml_t,
                                  params: *mut event)
                                  -> status;
    #[link_name="switch_xml_clear_user_cache"]
    pub fn xml_clear_user_cache(key: *const c_char,
                                user_name: *const c_char,
                                domain_name: *const c_char)
                                -> u32;
    #[link_name="switch_xml_merge_user"]
    pub fn xml_merge_user(user: xml_t, domain: xml_t, group: xml_t);
    #[link_name="switch_xml_dup"]
    pub fn xml_dup(xml: xml_t) -> xml_t;
    #[link_name="switch_xml_open_cfg"]
    pub fn xml_open_cfg(file_path: *const c_char, node: *mut xml_t, params: *mut event) -> xml_t;
    #[link_name="switch_xml_set_binding_sections"]
    pub fn xml_set_binding_sections(binding: *mut xml_binding, sections: xml_section);
    #[link_name="switch_xml_set_binding_user_data"]
    pub fn xml_set_binding_user_data(binding: *mut xml_binding, user_data: *mut c_void);
    #[link_name="switch_xml_get_binding_sections"]
    pub fn xml_get_binding_sections(binding: *mut xml_binding) -> xml_section;
    #[link_name="switch_xml_get_binding_user_data"]
    pub fn xml_get_binding_user_data(binding: *mut xml_binding) -> *mut c_void;
    #[link_name="switch_xml_bind_search_function_ret"]
    pub fn xml_bind_search_function_ret(function: xml_search_function,
                                        sections: xml_section,
                                        user_data: *mut c_void,
                                        ret_binding: *mut *mut xml_binding)
                                        -> status;
    #[link_name="switch_xml_unbind_search_function"]
    pub fn xml_unbind_search_function(binding: *mut *mut xml_binding) -> status;
    #[link_name="switch_xml_unbind_search_function_ptr"]
    pub fn xml_unbind_search_function_ptr(function: xml_search_function) -> status;
    #[link_name="switch_xml_parse_section_string"]
    pub fn xml_parse_section_string(str: *const c_char) -> xml_section;
    #[link_name="switch_xml_std_datetime_check"]
    pub fn xml_std_datetime_check(xcond: xml_t,
                                  offset: *mut c_int,
                                  tzname: *const c_char)
                                  -> c_int;
    #[link_name="switch_xml_locate_language"]
    pub fn xml_locate_language(root: *mut xml_t,
                               node: *mut xml_t,
                               params: *mut event,
                               language: *mut xml_t,
                               phrases: *mut xml_t,
                               macros: *mut xml_t,
                               str_language: *const c_char)
                               -> status;
    #[link_name="switch_config_perform_set_item"]
    pub fn config_perform_set_item(item: *mut xml_config_item,
                                   key: *const c_char,
                                   type_: xml_config_type,
                                   flags: c_int,
                                   ptr: *mut c_void,
                                   defaultvalue: *const c_void,
                                   data: *mut c_void,
                                   function: xml_config_callback,
                                   syntax: *const c_char,
                                   helptext: *const c_char);
    #[link_name="switch_xml_config_enum_str2int"]
    pub fn xml_config_enum_str2int(enum_options: *mut xml_config_enum_item,
                                   value: *const c_char,
                                   out: *mut c_int)
                                   -> status;
    #[link_name="switch_xml_config_enum_int2str"]
    pub fn xml_config_enum_int2str(enum_options: *mut xml_config_enum_item,
                                   value: c_int)
                                   -> *const c_char;
    #[link_name="switch_xml_config_item_print_doc"]
    pub fn xml_config_item_print_doc(level: c_int, item: *mut xml_config_item);
    #[link_name="switch_xml_config_parse"]
    pub fn xml_config_parse(xml: xml_t,
                            reload: switch_bool,
                            instructions: *mut xml_config_item)
                            -> status;
    #[link_name="switch_xml_config_parse_module_settings"]
    pub fn xml_config_parse_module_settings(file: *const c_char,
                                            reload: switch_bool,
                                            instructions: *mut xml_config_item)
                                            -> status;
    #[link_name="switch_xml_config_parse_event"]
    pub fn xml_config_parse_event(event: *mut event,
                                  count: c_int,
                                  reload: switch_bool,
                                  instructions: *mut xml_config_item)
                                  -> status;
    #[link_name="switch_event_import_xml"]
    pub fn event_import_xml(xml: xml_t,
                            keyname: *const c_char,
                            valuename: *const c_char,
                            event: *mut *mut event)
                            -> usize;
    #[link_name="switch_xml_config_cleanup"]
    pub fn xml_config_cleanup(instructions: *mut xml_config_item);
    #[link_name="switch_core_session_get_event_hooks"]
    pub fn core_session_get_event_hooks(session: *mut core_session) -> io_event_hooks;
    #[link_name="switch_core_event_hook_add_outgoing_channel"]
    pub fn core_event_hook_add_outgoing_channel(session: *mut core_session,
                                                outgoing_channel: outgoing_channel_hook)
                                                -> status;
    #[link_name="switch_core_event_hook_add_receive_message"]
    pub fn core_event_hook_add_receive_message(session: *mut core_session,
                                               receive_message: receive_message_hook)
                                               -> status;
    #[link_name="switch_core_event_hook_add_receive_event"]
    pub fn core_event_hook_add_receive_event(session: *mut core_session,
                                             receive_event: receive_event_hook)
                                             -> status;
    #[link_name="switch_core_event_hook_add_state_change"]
    pub fn core_event_hook_add_state_change(session: *mut core_session,
                                            state_change: state_change_hook)
                                            -> status;
    #[link_name="switch_core_event_hook_add_state_run"]
    pub fn core_event_hook_add_state_run(session: *mut core_session,
                                         state_run: state_run_hook)
                                         -> status;
    #[link_name="switch_core_event_hook_add_read_frame"]
    pub fn core_event_hook_add_read_frame(session: *mut core_session,
                                          read_frame: read_frame_hook)
                                          -> status;
    #[link_name="switch_core_event_hook_add_write_frame"]
    pub fn core_event_hook_add_write_frame(session: *mut core_session,
                                           write_frame: write_frame_hook)
                                           -> status;
    #[link_name="switch_core_event_hook_add_video_read_frame"]
    pub fn core_event_hook_add_video_read_frame(session: *mut core_session,
                                                video_read_frame: video_read_frame_hook)
                                                -> status;
    #[link_name="switch_core_event_hook_add_video_write_frame"]
    pub fn core_event_hook_add_video_write_frame(session: *mut core_session,
                                                 video_write_frame: video_write_frame_hook)
                                                 -> status;
    #[link_name="switch_core_event_hook_add_kill_channel"]
    pub fn core_event_hook_add_kill_channel(session: *mut core_session,
                                            kill_channel: kill_channel_hook)
                                            -> status;
    #[link_name="switch_core_event_hook_add_send_dtmf"]
    pub fn core_event_hook_add_send_dtmf(session: *mut core_session,
                                         send_dtmf: send_dtmf_hook)
                                         -> status;
    #[link_name="switch_core_event_hook_add_recv_dtmf"]
    pub fn core_event_hook_add_recv_dtmf(session: *mut core_session,
                                         recv_dtmf: recv_dtmf_hook)
                                         -> status;
    #[link_name="switch_core_event_hook_remove_outgoing_channel"]
    pub fn core_event_hook_remove_outgoing_channel(session: *mut core_session,
                                                   outgoing_channel: outgoing_channel_hook)
                                                   -> status;
    #[link_name="switch_core_event_hook_remove_receive_message"]
    pub fn core_event_hook_remove_receive_message(session: *mut core_session,
                                                  receive_message: receive_message_hook)
                                                  -> status;
    #[link_name="switch_core_event_hook_remove_receive_event"]
    pub fn core_event_hook_remove_receive_event(session: *mut core_session,
                                                receive_event: receive_event_hook)
                                                -> status;
    #[link_name="switch_core_event_hook_remove_state_change"]
    pub fn core_event_hook_remove_state_change(session: *mut core_session,
                                               state_change: state_change_hook)
                                               -> status;
    #[link_name="switch_core_event_hook_remove_state_run"]
    pub fn core_event_hook_remove_state_run(session: *mut core_session,
                                            state_run: state_run_hook)
                                            -> status;
    #[link_name="switch_core_event_hook_remove_read_frame"]
    pub fn core_event_hook_remove_read_frame(session: *mut core_session,
                                             read_frame: read_frame_hook)
                                             -> status;
    #[link_name="switch_core_event_hook_remove_write_frame"]
    pub fn core_event_hook_remove_write_frame(session: *mut core_session,
                                              write_frame: write_frame_hook)
                                              -> status;
    #[link_name="switch_core_event_hook_remove_video_read_frame"]
    pub fn core_event_hook_remove_video_read_frame(session: *mut core_session,
                                                   video_read_frame: video_read_frame_hook)
                                                   -> status;
    #[link_name="switch_core_event_hook_remove_video_write_frame"]
    pub fn core_event_hook_remove_video_write_frame(session: *mut core_session,
                                                    video_write_frame: video_write_frame_hook)
                                                    -> status;
    #[link_name="switch_core_event_hook_remove_kill_channel"]
    pub fn core_event_hook_remove_kill_channel(session: *mut core_session,
                                               kill_channel: kill_channel_hook)
                                               -> status;
    #[link_name="switch_core_event_hook_remove_send_dtmf"]
    pub fn core_event_hook_remove_send_dtmf(session: *mut core_session,
                                            send_dtmf: send_dtmf_hook)
                                            -> status;
    #[link_name="switch_core_event_hook_remove_recv_dtmf"]
    pub fn core_event_hook_remove_recv_dtmf(session: *mut core_session,
                                            recv_dtmf: recv_dtmf_hook)
                                            -> status;
    #[link_name="switch_scheduler_add_task"]
    pub fn scheduler_add_task(task_runtime: time_t,
                              func: scheduler_func,
                              desc: *const c_char,
                              group: *const c_char,
                              cmd_id: u32,
                              cmd_arg: *mut c_void,
                              flags: scheduler_flag)
                              -> u32;
    #[link_name="switch_scheduler_del_task_id"]
    pub fn scheduler_del_task_id(task_id: u32) -> u32;
    #[link_name="switch_scheduler_del_task_group"]
    pub fn scheduler_del_task_group(group: *const c_char) -> u32;
    #[link_name="switch_scheduler_task_thread_start"]
    pub fn scheduler_task_thread_start();
    #[link_name="switch_scheduler_task_thread_stop"]
    pub fn scheduler_task_thread_stop();
    #[link_name="switch_config_open_file"]
    pub fn config_open_file(cfg: *mut config, file_path: *mut c_char) -> c_int;
    #[link_name="switch_config_close_file"]
    pub fn config_close_file(cfg: *mut config);
    #[link_name="switch_config_next_pair"]
    pub fn config_next_pair(cfg: *mut config,
                            var: *mut *mut c_char,
                            val: *mut *mut c_char)
                            -> c_int;
    #[link_name="switch_nat_get_type"]
    pub fn nat_get_type() -> *const c_char;
    #[link_name="switch_nat_init"]
    pub fn nat_init(pool: *mut memory_pool, mapping: switch_bool);
    #[link_name="switch_nat_late_init"]
    pub fn nat_late_init();
    #[link_name="switch_nat_shutdown"]
    pub fn nat_shutdown();
    #[link_name="switch_nat_status"]
    pub fn nat_status() -> *mut c_char;
    #[link_name="switch_nat_republish"]
    pub fn nat_republish();
    #[link_name="switch_nat_reinit"]
    pub fn nat_reinit();
    #[link_name="switch_nat_set_mapping"]
    pub fn nat_set_mapping(mapping: switch_bool);
    #[link_name="switch_nat_add_mapping"]
    pub fn nat_add_mapping(port: port,
                           proto: nat_ip_proto,
                           external_port: *mut port,
                           sticky: switch_bool)
                           -> status;
    #[link_name="switch_nat_is_initialized"]
    pub fn nat_is_initialized() -> switch_bool;
    #[link_name="switch_nat_del_mapping"]
    pub fn nat_del_mapping(port: port, proto: nat_ip_proto) -> status;
    #[link_name="switch_odbc_handle_new"]
    pub fn odbc_handle_new(dsn: *const c_char,
                           username: *const c_char,
                           password: *const c_char)
                           -> *mut odbc_handle;
    #[link_name="switch_odbc_set_num_retries"]
    pub fn odbc_set_num_retries(handle: *mut odbc_handle, num_retries: c_int);
    #[link_name="switch_odbc_handle_disconnect"]
    pub fn odbc_handle_disconnect(handle: *mut odbc_handle) -> odbc_status;
    #[link_name="switch_odbc_handle_connect"]
    pub fn odbc_handle_connect(handle: *mut odbc_handle) -> odbc_status;
    #[link_name="switch_odbc_handle_destroy"]
    pub fn odbc_handle_destroy(handlep: *mut *mut odbc_handle);
    #[link_name="switch_odbc_handle_get_state"]
    pub fn odbc_handle_get_state(handle: *mut odbc_handle) -> odbc_state;
    #[link_name="switch_odbc_handle_exec"]
    pub fn odbc_handle_exec(handle: *mut odbc_handle,
                            sql: *const c_char,
                            rstmt: *mut odbc_statement_handle,
                            err: *mut *mut c_char)
                            -> odbc_status;
    #[link_name="switch_odbc_handle_exec_string"]
    pub fn odbc_handle_exec_string(handle: *mut odbc_handle,
                                   sql: *const c_char,
                                   resbuf: *mut c_char,
                                   len: usize,
                                   err: *mut *mut c_char)
                                   -> odbc_status;
    #[link_name="switch_odbc_available"]
    pub fn odbc_available() -> switch_bool;
    #[link_name="switch_odbc_SQLSetAutoCommitAttr"]
    pub fn odbc_SQLSetAutoCommitAttr(handle: *mut odbc_handle, on: switch_bool) -> odbc_status;
    #[link_name="switch_odbc_SQLEndTran"]
    pub fn odbc_SQLEndTran(handle: *mut odbc_handle, commit: switch_bool) -> odbc_status;
    #[link_name="switch_odbc_statement_handle_free"]
    pub fn odbc_statement_handle_free(stmt: *mut odbc_statement_handle) -> odbc_status;
    #[link_name="switch_odbc_handle_callback_exec_detailed"]
    pub fn odbc_handle_callback_exec_detailed(file: *const c_char,
                                              func: *const c_char,
                                              line: c_int,
                                              handle: *mut odbc_handle,
                                              sql: *const c_char,
                                              callback: core_db_callback_func,
                                              pdata: *mut c_void,
                                              err: *mut *mut c_char)
                                              -> odbc_status;
    #[link_name="switch_odbc_handle_get_error"]
    pub fn odbc_handle_get_error(handle: *mut odbc_handle,
                                 stmt: odbc_statement_handle)
                                 -> *mut c_char;
    #[link_name="switch_odbc_handle_affected_rows"]
    pub fn odbc_handle_affected_rows(handle: *mut odbc_handle) -> c_int;
    #[link_name="switch_pgsql_handle_new"]
    pub fn pgsql_handle_new(dsn: *const c_char) -> *mut pgsql_handle;
    #[link_name="switch_pgsql_set_num_retries"]
    pub fn pgsql_set_num_retries(handle: *mut pgsql_handle, num_retries: c_int);
    #[link_name="switch_pgsql_handle_disconnect"]
    pub fn pgsql_handle_disconnect(handle: *mut pgsql_handle) -> pgsql_status;
    #[link_name="switch_pgsql_handle_connect"]
    pub fn pgsql_handle_connect(handle: *mut pgsql_handle) -> pgsql_status;
    #[link_name="switch_pgsql_handle_destroy"]
    pub fn pgsql_handle_destroy(handlep: *mut *mut pgsql_handle);
    #[link_name="switch_pgsql_send_query"]
    pub fn pgsql_send_query(handle: *mut pgsql_handle, sql: *const c_char) -> pgsql_status;
    #[link_name="switch_pgsql_cancel_real"]
    pub fn pgsql_cancel_real(file: *const c_char,
                             func: *const c_char,
                             line: c_int,
                             handle: *mut pgsql_handle)
                             -> pgsql_status;
    #[link_name="switch_pgsql_next_result_timed"]
    pub fn pgsql_next_result_timed(handle: *mut pgsql_handle,
                                   result_out: *mut *mut pgsql_result,
                                   seconds: c_int)
                                   -> pgsql_status;
    #[link_name="switch_pgsql_free_result"]
    pub fn pgsql_free_result(result: *mut *mut pgsql_result);
    #[link_name="switch_pgsql_finish_results_real"]
    pub fn pgsql_finish_results_real(file: *const c_char,
                                     func: *const c_char,
                                     line: c_int,
                                     handle: *mut pgsql_handle)
                                     -> pgsql_status;
    #[link_name="switch_pgsql_handle_exec_base_detailed"]
    pub fn pgsql_handle_exec_base_detailed(file: *const c_char,
                                           func: *const c_char,
                                           line: c_int,
                                           handle: *mut pgsql_handle,
                                           sql: *const c_char,
                                           err: *mut *mut c_char)
                                           -> pgsql_status;
    #[link_name="switch_pgsql_handle_get_state"]
    pub fn pgsql_handle_get_state(handle: *mut pgsql_handle) -> pgsql_state;
    #[link_name="switch_pgsql_handle_exec_detailed"]
    pub fn pgsql_handle_exec_detailed(file: *const c_char,
                                      func: *const c_char,
                                      line: c_int,
                                      handle: *mut pgsql_handle,
                                      sql: *const c_char,
                                      err: *mut *mut c_char)
                                      -> pgsql_status;
    #[link_name="switch_pgsql_handle_exec_string_detailed"]
    pub fn pgsql_handle_exec_string_detailed(file: *const c_char,
                                             func: *const c_char,
                                             line: c_int,
                                             handle: *mut pgsql_handle,
                                             sql: *const c_char,
                                             resbuf: *mut c_char,
                                             len: usize,
                                             err: *mut *mut c_char)
                                             -> pgsql_status;
    #[link_name="switch_pgsql_available"]
    pub fn pgsql_available() -> switch_bool;
    #[link_name="switch_pgsql_SQLSetAutoCommitAttr"]
    pub fn pgsql_SQLSetAutoCommitAttr(handle: *mut pgsql_handle, on: switch_bool) -> pgsql_status;
    #[link_name="switch_pgsql_SQLEndTran"]
    pub fn pgsql_SQLEndTran(handle: *mut pgsql_handle, commit: switch_bool) -> pgsql_status;
    #[link_name="switch_pgsql_handle_callback_exec_detailed"]
    pub fn pgsql_handle_callback_exec_detailed(file: *const c_char,
                                               func: *const c_char,
                                               line: c_int,
                                               handle: *mut pgsql_handle,
                                               sql: *const c_char,
                                               callback: core_db_callback_func,
                                               pdata: *mut c_void,
                                               err: *mut *mut c_char)
                                               -> pgsql_status;
    #[link_name="switch_pgsql_handle_get_error"]
    pub fn pgsql_handle_get_error(handle: *mut pgsql_handle) -> *mut c_char;
    #[link_name="switch_pgsql_handle_affected_rows"]
    pub fn pgsql_handle_affected_rows(handle: *mut pgsql_handle) -> c_int;
    #[link_name="switch_pgsql_flush"]
    pub fn pgsql_flush(handle: *mut pgsql_handle) -> pgsql_status;
    #[link_name="switch_limit_init"]
    pub fn limit_init(pool: *mut memory_pool);
    #[link_name="switch_limit_incr"]
    pub fn limit_incr(backend: *const c_char,
                      session: *mut core_session,
                      realm: *const c_char,
                      resource: *const c_char,
                      max: c_int,
                      interval: c_int)
                      -> status;
    #[link_name="switch_limit_release"]
    pub fn limit_release(backend: *const c_char,
                         session: *mut core_session,
                         realm: *const c_char,
                         resource: *const c_char)
                         -> status;
    #[link_name="switch_limit_usage"]
    pub fn limit_usage(backend: *const c_char,
                       realm: *const c_char,
                       resource: *const c_char,
                       rcount: *mut u32)
                       -> c_int;
    #[link_name="switch_limit_interval_reset"]
    pub fn limit_interval_reset(backend: *const c_char,
                                realm: *const c_char,
                                resource: *const c_char)
                                -> status;
    #[link_name="switch_limit_reset"]
    pub fn limit_reset(backend: *const c_char) -> status;
    #[link_name="switch_limit_fire_event"]
    pub fn limit_fire_event(backend: *const c_char,
                            realm: *const c_char,
                            resource: *const c_char,
                            usage: u32,
                            rate: u32,
                            max: u32,
                            ratemax: u32);
    #[link_name="switch_limit_status"]
    pub fn limit_status(backend: *const c_char) -> *mut c_char;
    #[link_name="switch_media_handle_create"]
    pub fn media_handle_create(smhp: *mut *mut media_handle,
                               session: *mut core_session,
                               params: *mut core_media_params)
                               -> status;
    #[link_name="switch_media_handle_destroy"]
    pub fn media_handle_destroy(session: *mut core_session);
    #[link_name="switch_core_session_get_media_handle"]
    pub fn core_session_get_media_handle(session: *mut core_session) -> *mut media_handle;
    #[link_name="switch_core_session_clear_media_handle"]
    pub fn core_session_clear_media_handle(session: *mut core_session) -> status;
    #[link_name="switch_core_session_media_handle_ready"]
    pub fn core_session_media_handle_ready(session: *mut core_session) -> status;
    #[link_name="switch_media_handle_set_media_flag"]
    pub fn media_handle_set_media_flag(smh: *mut media_handle, flag: core_media_flag);
    #[link_name="switch_media_handle_clear_media_flag"]
    pub fn media_handle_clear_media_flag(smh: *mut media_handle, flag: core_media_flag);
    #[link_name="switch_media_handle_test_media_flag"]
    pub fn media_handle_test_media_flag(smh: *mut media_handle, flag: core_media_flag) -> i32;
    #[link_name="switch_media_handle_set_media_flags"]
    pub fn media_handle_set_media_flags(smh: *mut media_handle, flags: *mut core_media_flag);
    #[link_name="switch_core_session_check_outgoing_crypto"]
    pub fn core_session_check_outgoing_crypto(session: *mut core_session);
    #[link_name="switch_core_session_local_crypto_key"]
    pub fn core_session_local_crypto_key(session: *mut core_session,
                                         type_: media_type)
                                         -> *const c_char;
    #[link_name="switch_core_session_check_incoming_crypto"]
    pub fn core_session_check_incoming_crypto(session: *mut core_session,
                                              varname: *const c_char,
                                              type_: media_type,
                                              crypto: *const c_char,
                                              crypto_tag: c_int,
                                              sdp_type: sdp_type)
                                              -> c_int;
    #[link_name="switch_core_media_get_video_fps"]
    pub fn core_media_get_video_fps(session: *mut core_session) -> u32;
    #[link_name="switch_core_media_set_rtp_session"]
    pub fn core_media_set_rtp_session(session: *mut core_session,
                                      type_: media_type,
                                      rtp_session: *mut rtp);
    #[link_name="switch_core_media_get_codec_string"]
    pub fn core_media_get_codec_string(session: *mut core_session) -> *const c_char;
    #[link_name="switch_core_media_parse_rtp_bugs"]
    pub fn core_media_parse_rtp_bugs(flag_pole: *mut rtp_bug_flag, str: *const c_char);
    #[link_name="switch_core_media_extract_t38_options"]
    pub fn core_media_extract_t38_options(session: *mut core_session,
                                          r_sdp: *const c_char)
                                          -> *mut t38_options;
    #[link_name="switch_core_media_pass_zrtp_hash"]
    pub fn core_media_pass_zrtp_hash(session: *mut core_session);
    #[link_name="switch_core_media_get_zrtp_hash"]
    pub fn core_media_get_zrtp_hash(session: *mut core_session,
                                    type_: media_type,
                                    local: switch_bool)
                                    -> *const c_char;
    #[link_name="switch_core_media_pass_zrtp_hash2"]
    pub fn core_media_pass_zrtp_hash2(aleg_session: *mut core_session,
                                      bleg_session: *mut core_session);
    #[link_name="switch_core_media_toggle_hold"]
    pub fn core_media_toggle_hold(session: *mut core_session, sendonly: c_int) -> c_int;
    #[link_name="switch_core_media_copy_t38_options"]
    pub fn core_media_copy_t38_options(t38_options: *mut t38_options, session: *mut core_session);
    #[link_name="switch_core_media_negotiate_sdp"]
    pub fn core_media_negotiate_sdp(session: *mut core_session,
                                    r_sdp: *const c_char,
                                    proceed: *mut u8,
                                    sdp_type: sdp_type)
                                    -> u8;
    #[link_name="switch_core_media_set_video_codec"]
    pub fn core_media_set_video_codec(session: *mut core_session, force: c_int) -> status;
    #[link_name="switch_core_media_set_codec"]
    pub fn core_media_set_codec(session: *mut core_session,
                                force: c_int,
                                codec_flags: u32)
                                -> status;
    #[link_name="switch_core_media_check_video_codecs"]
    pub fn core_media_check_video_codecs(session: *mut core_session);
    #[link_name="switch_core_media_read_frame"]
    pub fn core_media_read_frame(session: *mut core_session,
                                 frame: *mut *mut frame,
                                 flags: io_flag,
                                 stream_id: c_int,
                                 type_: media_type)
                                 -> status;
    #[link_name="switch_core_media_write_frame"]
    pub fn core_media_write_frame(session: *mut core_session,
                                  frame: *mut frame,
                                  flags: io_flag,
                                  stream_id: c_int,
                                  type_: media_type)
                                  -> status;
    #[link_name="switch_core_media_check_nat"]
    pub fn core_media_check_nat(smh: *mut media_handle, network_ip: *const c_char) -> c_int;
    #[link_name="switch_core_media_choose_port"]
    pub fn core_media_choose_port(session: *mut core_session,
                                  type_: media_type,
                                  force: c_int)
                                  -> status;
    #[link_name="switch_core_media_choose_ports"]
    pub fn core_media_choose_ports(session: *mut core_session,
                                   audio: switch_bool,
                                   video: switch_bool)
                                   -> status;
    #[link_name="switch_core_media_check_dtmf_type"]
    pub fn core_media_check_dtmf_type(session: *mut core_session);
    #[link_name="switch_core_media_absorb_sdp"]
    pub fn core_media_absorb_sdp(session: *mut core_session);
    #[link_name="switch_core_media_proxy_remote_addr"]
    pub fn core_media_proxy_remote_addr(session: *mut core_session,
                                        sdp_str: *const c_char)
                                        -> status;
    #[link_name="switch_core_media_deactivate_rtp"]
    pub fn core_media_deactivate_rtp(session: *mut core_session);
    #[link_name="switch_core_media_activate_rtp"]
    pub fn core_media_activate_rtp(session: *mut core_session) -> status;
    #[link_name="switch_core_media_ext_address_lookup"]
    pub fn core_media_ext_address_lookup(session: *mut core_session,
                                         ip: *mut *mut c_char,
                                         port: *mut port,
                                         sourceip: *const c_char)
                                         -> status;
    #[link_name="switch_core_media_process_t38_passthru"]
    pub fn core_media_process_t38_passthru(session: *mut core_session,
                                           other_session: *mut core_session,
                                           t38_options: *mut t38_options)
                                           -> status;
    #[link_name="switch_core_media_gen_local_sdp"]
    pub fn core_media_gen_local_sdp(session: *mut core_session,
                                    sdp_type: sdp_type,
                                    ip: *const c_char,
                                    port: port,
                                    sr: *const c_char,
                                    force: c_int);
    #[link_name="switch_core_media_set_local_sdp"]
    pub fn core_media_set_local_sdp(session: *mut core_session,
                                    sdp_str: *const c_char,
                                    dup: switch_bool);
    #[link_name="switch_core_media_patch_sdp"]
    pub fn core_media_patch_sdp(session: *mut core_session);
    #[link_name="switch_core_media_set_udptl_image_sdp"]
    pub fn core_media_set_udptl_image_sdp(session: *mut core_session,
                                          t38_options: *mut t38_options,
                                          insist: c_int);
    #[link_name="switch_core_media_get_mparams"]
    pub fn core_media_get_mparams(smh: *mut media_handle) -> *mut core_media_params;
    #[link_name="switch_core_media_prepare_codecs"]
    pub fn core_media_prepare_codecs(session: *mut core_session, force: switch_bool);
    #[link_name="switch_core_media_start_udptl"]
    pub fn core_media_start_udptl(session: *mut core_session, t38_options: *mut t38_options);
    #[link_name="switch_core_media_hard_mute"]
    pub fn core_media_hard_mute(session: *mut core_session, on: switch_bool);
    #[link_name="switch_core_media_receive_message"]
    pub fn core_media_receive_message(session: *mut core_session,
                                      msg: *mut core_session_message)
                                      -> status;
    #[link_name="switch_core_media_break"]
    pub fn core_media_break(session: *mut core_session, type_: media_type);
    #[link_name="switch_core_media_kill_socket"]
    pub fn core_media_kill_socket(session: *mut core_session, type_: media_type);
    #[link_name="switch_core_media_queue_rfc2833"]
    pub fn core_media_queue_rfc2833(session: *mut core_session,
                                    type_: media_type,
                                    dtmf: *const dtmf)
                                    -> status;
    #[link_name="switch_core_media_queue_rfc2833_in"]
    pub fn core_media_queue_rfc2833_in(session: *mut core_session,
                                       type_: media_type,
                                       dtmf: *const dtmf)
                                       -> status;
    #[link_name="switch_core_media_ready"]
    pub fn core_media_ready(session: *mut core_session, type_: media_type) -> u8;
    #[link_name="switch_core_media_set_telephony_event"]
    pub fn core_media_set_telephony_event(session: *mut core_session,
                                          type_: media_type,
                                          te: payload);
    #[link_name="switch_core_media_set_telephony_recv_event"]
    pub fn core_media_set_telephony_recv_event(session: *mut core_session,
                                               type_: media_type,
                                               te: payload);
    #[link_name="switch_core_media_stats"]
    pub fn core_media_stats(session: *mut core_session,
                            type_: media_type,
                            pool: *mut memory_pool)
                            -> *mut rtp_stats;
    #[link_name="switch_core_media_udptl_mode"]
    pub fn core_media_udptl_mode(session: *mut core_session, type_: media_type) -> status;
    #[link_name="switch_core_media_check_udptl_mode"]
    pub fn core_media_check_udptl_mode(session: *mut core_session,
                                       type_: media_type)
                                       -> switch_bool;
    #[link_name="switch_core_media_set_rtp_flag"]
    pub fn core_media_set_rtp_flag(session: *mut core_session, type_: media_type, flag: rtp_flag);
    #[link_name="switch_core_media_clear_rtp_flag"]
    pub fn core_media_clear_rtp_flag(session: *mut core_session,
                                     type_: media_type,
                                     flag: rtp_flag);
    #[link_name="switch_core_media_get_jb"]
    pub fn core_media_get_jb(session: *mut core_session, type_: media_type) -> *mut jb;
    #[link_name="switch_core_media_get_stats"]
    pub fn core_media_get_stats(session: *mut core_session,
                                type_: media_type,
                                pool: *mut memory_pool)
                                -> *mut rtp_stats;
    #[link_name="switch_core_media_set_sdp_codec_string"]
    pub fn core_media_set_sdp_codec_string(session: *mut core_session,
                                           r_sdp: *const c_char,
                                           sdp_type: sdp_type);
    #[link_name="switch_core_media_reset_autofix"]
    pub fn core_media_reset_autofix(session: *mut core_session, type_: media_type);
    #[link_name="switch_core_media_check_outgoing_proxy"]
    pub fn core_media_check_outgoing_proxy(session: *mut core_session,
                                           o_session: *mut core_session);
    #[link_name="switch_core_media_codec_chosen"]
    pub fn core_media_codec_chosen(session: *mut core_session, media: media_type) -> status;
    #[link_name="switch_core_media_recover_session"]
    pub fn core_media_recover_session(session: *mut core_session);
    #[link_name="switch_core_media_add_ice_acl"]
    pub fn core_media_add_ice_acl(session: *mut core_session,
                                  type_: media_type,
                                  acl_name: *const c_char)
                                  -> status;
    #[link_name="switch_core_session_set_ice"]
    pub fn core_session_set_ice(session: *mut core_session);
    #[link_name="switch_core_media_clear_ice"]
    pub fn core_media_clear_ice(session: *mut core_session);
    #[link_name="switch_core_media_pause"]
    pub fn core_media_pause(session: *mut core_session);
    #[link_name="switch_core_media_resume"]
    pub fn core_media_resume(session: *mut core_session);
    #[link_name="switch_core_media_init"]
    pub fn core_media_init();
    #[link_name="switch_core_media_deinit"]
    pub fn core_media_deinit();
    #[link_name="switch_core_media_set_stats"]
    pub fn core_media_set_stats(session: *mut core_session);
    #[link_name="switch_core_media_sync_stats"]
    pub fn core_media_sync_stats(session: *mut core_session);
    #[link_name="switch_core_session_wake_video_thread"]
    pub fn core_session_wake_video_thread(session: *mut core_session);
    #[link_name="switch_core_session_clear_crypto"]
    pub fn core_session_clear_crypto(session: *mut core_session);
    #[link_name="switch_core_session_get_payload_code"]
    pub fn core_session_get_payload_code(session: *mut core_session,
                                         type_: media_type,
                                         iananame: *const c_char,
                                         rate: u32,
                                         fmtp_in: *const c_char,
                                         ptP: *mut payload,
                                         recv_ptP: *mut payload,
                                         fmtpP: *mut *mut c_char)
                                         -> status;
    #[link_name="switch_core_media_add_payload_map"]
    pub fn core_media_add_payload_map(session: *mut core_session,
                                      type_: media_type,
                                      name: *const c_char,
                                      modname: *const c_char,
                                      fmtp: *const c_char,
                                      sdp_type: sdp_type,
                                      pt: u32,
                                      rate: u32,
                                      ptime: u32,
                                      channels: u32,
                                      negotiated: u8)
                                      -> *mut payload_map;
    #[link_name="switch_core_media_check_autoadj"]
    pub fn core_media_check_autoadj(session: *mut core_session) -> status;
    #[link_name="switch_core_media_crypto_str2type"]
    pub fn core_media_crypto_str2type(str: *const c_char) -> rtp_crypto_key_type;
    #[link_name="switch_core_media_crypto_type2str"]
    pub fn core_media_crypto_type2str(type_: rtp_crypto_key_type) -> *const c_char;
    #[link_name="switch_core_media_crypto_keylen"]
    pub fn core_media_crypto_keylen(type_: rtp_crypto_key_type) -> c_int;
    #[link_name="switch_core_media_filter_sdp"]
    pub fn core_media_filter_sdp(sdp: *const c_char,
                                 cmd: *const c_char,
                                 arg: *const c_char)
                                 -> *mut c_char;
    #[link_name="switch_core_media_process_sdp_filter"]
    pub fn core_media_process_sdp_filter(sdp: *const c_char,
                                         cmd_buf: *const c_char,
                                         session: *mut core_session)
                                         -> *mut c_char;
    #[link_name="switch_core_media_codec_control"]
    pub fn core_media_codec_control(session: *mut core_session,
                                    mtype: media_type,
                                    iotype: io_type,
                                    cmd: codec_control_command,
                                    ctype: codec_control_type,
                                    cmd_data: *mut c_void,
                                    atype: codec_control_type,
                                    cmd_arg: *mut c_void,
                                    rtype: *mut codec_control_type,
                                    ret_data: *mut *mut c_void)
                                    -> status;
    #[link_name="switch_core_media_get_timer"]
    pub fn core_media_get_timer(session: *mut core_session, mtype: media_type) -> *mut timer;
    #[link_name="switch_core_media_start_video_function"]
    pub fn core_media_start_video_function(session: *mut core_session,
                                           video_function: video_function,
                                           user_data: *mut c_void);
    #[link_name="switch_core_media_end_video_function"]
    pub fn core_media_end_video_function(session: *mut core_session);
    #[link_name="switch_core_session_start_video_thread"]
    pub fn core_session_start_video_thread(session: *mut core_session) -> status;
    #[link_name="switch_core_media_check_video_function"]
    pub fn core_media_check_video_function(session: *mut core_session) -> c_int;
    #[link_name="switch_core_session_video_reinit"]
    pub fn core_session_video_reinit(session: *mut core_session);
    #[link_name="switch_core_media_read_lock_unlock"]
    pub fn core_media_read_lock_unlock(session: *mut core_session,
                                       type_: media_type,
                                       lock: switch_bool)
                                       -> status;
    #[link_name="switch_core_session_stop_media"]
    pub fn core_session_stop_media(session: *mut core_session);
    #[link_name="switch_core_session_media_flow"]
    pub fn core_session_media_flow(session: *mut core_session, type_: media_type) -> media_flow;
    #[link_name="switch_core_media_get_vid_params"]
    pub fn core_media_get_vid_params(session: *mut core_session,
                                     vid_params: *mut vid_params)
                                     -> status;
    #[link_name="switch_core_media_lock_video_file"]
    pub fn core_media_lock_video_file(session: *mut core_session, rw: rw) -> status;
    #[link_name="switch_core_media_unlock_video_file"]
    pub fn core_media_unlock_video_file(session: *mut core_session, rw: rw) -> status;
    #[link_name="switch_core_media_set_video_file"]
    pub fn core_media_set_video_file(session: *mut core_session,
                                     fh: *mut file_handle,
                                     rw: rw)
                                     -> status;
    #[link_name="switch_core_media_get_video_file"]
    pub fn core_media_get_video_file(session: *mut core_session, rw: rw) -> *mut file_handle;
    #[link_name="switch_core_session_in_video_thread"]
    pub fn core_session_in_video_thread(session: *mut core_session) -> switch_bool;
    #[link_name="switch_core_media_check_dtls"]
    pub fn core_media_check_dtls(session: *mut core_session, type_: media_type) -> switch_bool;
    #[link_name="switch_core_media_set_outgoing_bitrate"]
    pub fn core_media_set_outgoing_bitrate(session: *mut core_session,
                                           type_: media_type,
                                           bitrate: u32)
                                           -> status;
    #[link_name="switch_core_media_reset_jb"]
    pub fn core_media_reset_jb(session: *mut core_session, type_: media_type) -> status;
    #[link_name="switch_core_session_wait_for_video_input_params"]
    pub fn core_session_wait_for_video_input_params(session: *mut core_session,
                                                    timeout_ms: u32)
                                                    -> status;
    #[link_name="switch_jb_create"]
    pub fn jb_create(jbp: *mut *mut jb,
                     type_: jb_type,
                     min_frame_len: u32,
                     max_frame_len: u32,
                     pool: *mut memory_pool)
                     -> status;
    #[link_name="switch_jb_set_frames"]
    pub fn jb_set_frames(jb: *mut jb, min_frame_len: u32, max_frame_len: u32) -> status;
    #[link_name="switch_jb_peek_frame"]
    pub fn jb_peek_frame(jb: *mut jb, ts: u32, seq: u16, peek: c_int, frame: *mut frame) -> status;
    #[link_name="switch_jb_get_frames"]
    pub fn jb_get_frames(jb: *mut jb,
                         min_frame_len: *mut u32,
                         max_frame_len: *mut u32,
                         cur_frame_len: *mut u32,
                         highest_frame_len: *mut u32)
                         -> status;
    #[link_name="switch_jb_destroy"]
    pub fn jb_destroy(jbp: *mut *mut jb) -> status;
    #[link_name="switch_jb_reset"]
    pub fn jb_reset(jb: *mut jb);
    #[link_name="switch_jb_debug_level"]
    pub fn jb_debug_level(jb: *mut jb, level: u8);
    #[link_name="switch_jb_frame_count"]
    pub fn jb_frame_count(jb: *mut jb) -> c_int;
    #[link_name="switch_jb_poll"]
    pub fn jb_poll(jb: *mut jb) -> c_int;
    #[link_name="switch_jb_put_packet"]
    pub fn jb_put_packet(jb: *mut jb, packet: *mut rtp_packet, len: usize) -> status;
    #[link_name="switch_jb_get_last_read_len"]
    pub fn jb_get_last_read_len(jb: *mut jb) -> usize;
    #[link_name="switch_jb_get_packet"]
    pub fn jb_get_packet(jb: *mut jb, packet: *mut rtp_packet, len: *mut usize) -> status;
    #[link_name="switch_jb_pop_nack"]
    pub fn jb_pop_nack(jb: *mut jb) -> u32;
    #[link_name="switch_jb_get_packet_by_seq"]
    pub fn jb_get_packet_by_seq(jb: *mut jb,
                                seq: u16,
                                packet: *mut rtp_packet,
                                len: *mut usize)
                                -> status;
    #[link_name="switch_jb_set_session"]
    pub fn jb_set_session(jb: *mut jb, session: *mut core_session);
    #[link_name="switch_jb_ts_mode"]
    pub fn jb_ts_mode(jb: *mut jb, samples_per_frame: u32, samples_per_second: u32);
    #[link_name="switch_jb_set_flag"]
    pub fn jb_set_flag(jb: *mut jb, flag: jb_flag);
    #[link_name="switch_jb_clear_flag"]
    pub fn jb_clear_flag(jb: *mut jb, flag: jb_flag);
    pub fn teletone_set_tone(ts: *mut teletone_generation_session, index: c_int, ...) -> c_int;
    pub fn teletone_set_map(map: *mut teletone_tone_map, ...) -> c_int;
    pub fn teletone_init_session(ts: *mut teletone_generation_session,
                                 buflen: c_int,
                                 handler: tone_handler,
                                 user_data: *mut c_void)
                                 -> c_int;
    pub fn teletone_destroy_session(ts: *mut teletone_generation_session) -> c_int;
    pub fn teletone_mux_tones(ts: *mut teletone_generation_session,
                              map: *mut teletone_tone_map)
                              -> c_int;
    pub fn teletone_run(ts: *mut teletone_generation_session, cmd: *const c_char) -> c_int;
    pub fn teletone_multi_tone_init(mt: *mut teletone_multi_tone, map: *mut teletone_tone_map);
    pub fn teletone_multi_tone_detect(mt: *mut teletone_multi_tone,
                                      sample_buffer: *mut i16,
                                      samples: c_int)
                                      -> c_int;
    pub fn teletone_dtmf_detect_init(dtmf_detect_state: *mut teletone_dtmf_detect_state,
                                     sample_rate: c_int);
    pub fn teletone_dtmf_detect(dtmf_detect_state: *mut teletone_dtmf_detect_state,
                                sample_buffer: *mut i16,
                                samples: c_int)
                                -> teletone_hit_type;
    pub fn teletone_dtmf_get(dtmf_detect_state: *mut teletone_dtmf_detect_state,
                             buf: *mut c_char,
                             dur: *mut c_uint)
                             -> c_int;
    pub fn teletone_goertzel_update(goertzel_state: *mut teletone_goertzel_state,
                                    sample_buffer: *mut i16,
                                    samples: c_int);
}
