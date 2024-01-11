use rcat::{display, get_args};

fn main() {
    let args = get_args();
    for file in &args.files {
        display(file, &args);
    }
}
