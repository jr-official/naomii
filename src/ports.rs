use port_check::free_local_port;

pub fn get_free_port() -> u32 {
    let starting_port: u32 = free_local_port().unwrap().into();

    return starting_port;
}
