use rcat::{display, get_args};

fn main() {
    let input = get_args();
    for file in &input.files {
        display(file, input.numbered_lines, input.numbered_nonblank_lines);
    }
}
