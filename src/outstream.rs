use libsoundio_sys as raw;

struct StreamData {
    outstream: raw::SoundIoOutStream,
}
