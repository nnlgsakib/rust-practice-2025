mod compound_types;
mod enum_l;
mod print;
mod scaler_types;
mod struct_l;
mod struct_methods;
mod unc_oparators;
mod var;
mod conditional_statements;
mod loop_l;
mod match_st;
mod if_let;

fn main() {
    print::print_func();
    struct_l::student_printer();
    var::var_practice();
    scaler_types::scaler_types();
    compound_types::cmp_type();
    enum_l::col_def();
    struct_methods::all_caller();
    unc_oparators::un_c();
    conditional_statements::con_st();
    loop_l::loop_l();
    match_st::match_st();
    if_let::if_let()
}
