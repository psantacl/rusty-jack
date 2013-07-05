## Rusty Jack 
A Rust crate which providers interop to the Jack Audio Connection Kit(http://jackaudio.org/)

## Build
1. Install the Jack Audio Connection Kit server: http://jackaudio.org/download
2. `make`

## Example
Please see Cog(https://github.com/psantacl/cog) for an example on using this crate.

## List of Jack functions currently supported
This list is always growing as new functions are needed.  If you need something not on here, just message me and I'll add it.  I was planning on tackling the MIDI API next.

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

For a complete list of all Jack API functions, please see the [docs](http://jackaudio.org/files/docs/html/index.html)
