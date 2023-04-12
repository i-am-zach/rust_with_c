// main.rs

mod bindings;

fn main() {
    let hello_result =  unsafe { bindings::hello() };
    println!("The result from the C function is: {}", hello_result);

    let a = 5;
    let b = 7;
    let sum_result = unsafe { bindings::sum(a, b) };
    println!("The result from the C function `sum` is: {}", sum_result);

    let mult_result = unsafe { bindings::mult(a, b) };
    println!("The result from the C function `mult` is: {}", mult_result);

    let seg_fault_result = unsafe { bindings::seg_fault() };
    println!("Seg fault = {}", seg_fault_result);
}

