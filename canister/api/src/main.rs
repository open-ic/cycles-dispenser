use candid_gen::generate_candid_method;

fn main() {
    generate_candid_method!(cycles_dispenser, c2c_request_cycles, update);

    candid::export_service!();
    std::print!("{}", __export_service());
}