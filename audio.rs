use core::libc::{c_void, c_int, c_char, c_ulong, c_float,  uint32_t, size_t };
use core::str::raw::{from_c_str};

use core::hashmap::linear;

type JackClient               = c_void;
type JackPort                 = c_void;
type CStr                    = *c_char;
type CStrArray              = *CStr;
type JackNFrames              = uint32_t;
type JackProcessCallback      = *u8;
type JackDefaultAudioSample   = c_float;  


struct BoxedJackStatus {
  pub val   : c_int,
  pub errors: ~[JackStatus]
}

struct JackRingBuffer {
  buf       : *c_char,
  write_ptr : size_t, 
  read_ptr  : size_t,
  size      : size_t,
  size_mask : size_t,
  mlocked   : c_int
}

struct ProcessArgs {
  pub out_port_ptr: *JackPort,
  pub rb_ptr: *JackRingBuffer,
  pub chan : Chan<~str>
}

fn write_cstr(c: *c_char) -> () {
  unsafe { 
    use core::libc::{puts};
    puts(c);
  }
}


//TODO:
//Callbacks:
//void jack_on_shutdown ( jack_client_t ∗ client, JackShutdownCallback function, void ∗ arg )
//int jack_set_buffer_size_callback ( jack_client_t ∗ client, JackBufferSizeCallback bufsize_callback, void ∗ arg )
//int jack_set_port_connect_callback ( jack_client_t ∗ , JackPortConnectCallback connect_callback, void ∗ arg )
//float jack_cpu_load ( jack_client_t ∗ client )
//jack_nframes_t jack_get_buffer_size ( jack_client_t ∗ )
//jack_nframes_t jack_get_sample_rate ( jack_client_t ∗ )

//Ports:
//int jack_connect ( jack_client_t ∗ , const char ∗ source_port, const char ∗ destination_port )
//int jack_disconnect ( jack_client_t ∗ , const char ∗ source_port, const char ∗ destination_port )


impl BoxedJackStatus {
  fn parse_jack_status(& mut self) -> () {
    let mut all_statuses = linear::LinearMap::new();
    let mut remaining = self.val;

    all_statuses.insert(0x01, JackFailure);
    all_statuses.insert(0x02, JackInvalidOption);
    all_statuses.insert(0x04, JackNameNotUnique);
    all_statuses.insert(0x08, JackServerStarted);
    all_statuses.insert(0x10, JackServerFailed);
    all_statuses.insert(0x20, JackServerError); 
    all_statuses.insert(0x40, JackNoSuchClient); 
    all_statuses.insert(0x80, JackLoadFailure); 
    all_statuses.insert(0x100, JackInitFailure); 
    all_statuses.insert(0x200, JackShmFailure); 
    all_statuses.insert(0x400, JackVersionError); 
    all_statuses.insert(0x800, JackBackendError); 
    all_statuses.insert(0x1000, JackClientZombie); 

    if (remaining == 0) {
      return; 
    }

    for uint::range_rev(6,0) |i| {
      let bit_val = float::pow_with_uint(2, i) as int;
      if remaining as int >= bit_val {
        self.errors.push(*all_statuses.get(&bit_val));
        remaining = remaining - bit_val as i32;
      }
      if (remaining == 0) {
        break;  
      }
    }
    return;
  }
}


enum JackStatus {
  JackFailure       = 0x01,
  JackInvalidOption = 0x02,
  JackNameNotUnique = 0x04,
  JackServerStarted = 0x08,
  JackServerFailed  = 0x10,
  JackServerError   = 0x20,
  JackNoSuchClient  = 0x40,
  JackLoadFailure   = 0x80,
  JackInitFailure   = 0x100,
  JackShmFailure    = 0x200,
  JackVersionError  = 0x400,
  JackBackendError  = 0x800,
  JackClientZombie  = 0x1000
}


enum JackPortFlags {
  JackPortIsInput    = 0x1,
  JackPortIsOutput   = 0x2,
  JackPortIsPhysical = 0x4,
  JackPortCanMonitor = 0x8,
  JackPortIsTerminal = 0x10
}


#[link_args = "-ljack"]
extern {
  fn jack_client_open( client_name : CStr,  options : c_int, status: *c_int) -> *JackClient;
  fn jack_client_close ( c: *JackClient ) -> c_int;

  fn jack_activate( client : *JackClient) -> c_int;
  fn jack_deactivate( client: *JackClient ) -> c_int;

  fn jack_port_register(client      : *JackClient,    port_name : CStr, 
      port_type   : CStr,          flags     : c_ulong, 
      buffer_size : c_ulong)     -> *JackPort;

  fn jack_get_ports(client            : *JackClient, port_name_pattern : CStr, 
      type_name_pattern : CStr,      flags              : c_ulong) -> CStrArray;

  fn jack_set_process_callback (client : *JackClient,  process_callback : JackProcessCallback, 
      arg    : *c_void)   -> c_int;

  fn jack_port_get_buffer (port : *JackPort, frames : JackNFrames) -> *JackDefaultAudioSample;

  fn jack_ringbuffer_create ( sz : size_t ) -> *JackRingBuffer;
  fn jack_ringbuffer_free   ( rb : *JackRingBuffer ) -> ();
  fn jack_ringbuffer_mlock  ( rb : *JackRingBuffer ) -> c_int;
  fn jack_ringbuffer_read_space ( rb :  *JackRingBuffer ) -> size_t;
  fn jack_ringbuffer_read   ( rb: *JackRingBuffer, dest : *c_char, cnt: size_t ) -> size_t;
  fn jack_ringbuffer_write_space ( rb: *JackRingBuffer ) -> size_t;
  fn jack_ringbuffer_write  ( rb: *JackRingBuffer, src : *char, cnt : size_t ) -> size_t;


}


unsafe fn from_CStrArray(str_array: CStrArray, results : & mut ~[~str]) -> () {
  let mut curr_ptr = str_array;

  while (!ptr::is_null(*curr_ptr)) {
    let next = from_c_str(*curr_ptr);
    results.push(next);
    curr_ptr = ptr::offset(curr_ptr, 1); 
  }
}


fn register_output_port(client : * JackPort) -> (*JackPort) {
  unsafe { 
    do str::as_c_str(~"32 bit float mono audio") |default_audio| {
      do str::as_c_str(~"out") |port_name| {
        let port_type : JackPortFlags = JackPortIsOutput;
        jack_port_register(client, port_name, default_audio, port_type as c_ulong, 0 as c_ulong)
      }
    }
  }
}

fn list_ports(client : *JackClient) -> () {
  unsafe { 
    do str::as_c_str(~"") |empty_string| {
      let ports : CStrArray = jack_get_ports(client, empty_string ,empty_string, 0 as c_ulong);
      let mut port_names :  ~[~str] = ~[];
      from_CStrArray(ports, & mut port_names);
      for port_names.each |port| {
        io::println(*port);
      }
    }
  }
}
