mod compound_types;
mod print;
mod scaler_types;
mod struct_l;
mod var;
mod enum_l;

fn main() {
    print::print_func();
    struct_l::student_printer();
    var::var_practice();
    scaler_types::scaler_types();
    compound_types::cmp_type();
    enum_l::col_def()
}
