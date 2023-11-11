mod module1;
mod module2;

use module1::print_a_Z;
use module2::sub_module::print_A_z;

fn main() {
    print_a_Z();
    print_A_z();
}