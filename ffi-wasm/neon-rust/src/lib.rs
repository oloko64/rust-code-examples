use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn fibonacci(mut cx: FunctionContext) -> JsResult<JsObject> {
    let num = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let result = fibonacci_memo(num, &mut vec![0_f64; (num + 1_f64) as usize]);
    let result_js = cx.number(result);

    let obj = cx.empty_object();
    obj.set(&mut cx, "result", result_js)?;

    Ok(obj)
}

fn fibonacci_memo(n: f64, memo: &mut Vec<f64>) -> f64 {
    if n <= 1.0 {
        return n;
    }

    if memo[n as usize] != 0.0 {
        return memo[n as usize];
    }

    let result = fibonacci_memo(n - 1.0, memo) + fibonacci_memo(n - 2.0, memo);
    memo[n as usize] = result;
    result
}

fn cpu_threads(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let num = num_cpus::get();
    let result = cx.number(num as f64);

    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("fibonacci", fibonacci)?;
    cx.export_function("cpu_threads", cpu_threads)?;

    Ok(())
}
