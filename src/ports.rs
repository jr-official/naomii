use port_check::free_local_port;

pub fn get_free_port() -> u16 {
    let starting_port = free_local_port().unwrap();

    return starting_port;
}
