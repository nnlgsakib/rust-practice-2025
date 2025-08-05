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
mod func_dec;
mod module;
mod mth;
mod error_l;
mod unwrap_n_expect_l;

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
    if_let::if_let();
    func_dec::caller();
    module::caller();
    error_l::error_handling();
    unwrap_n_expect_l::u_e();
}
