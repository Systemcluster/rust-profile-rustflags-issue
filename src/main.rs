pub fn main() {
    let i = sys_info::mem_info().unwrap();
    println!("Total memory: {} KB", i.total);
}
