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

fn multi_thread(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..3 {
                println!("thread 1: {i}");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
            println!("thread id: {:?}", std::thread::current().id());
        });

        s.spawn(|| {
            for i in 0..3 {
                println!("thread 2: {i}");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
            println!("thread id: {:?}", std::thread::current().id());
        });

        s.spawn(|| {
            for i in 0..3 {
                println!("thread 3: {i}");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
            println!("thread id: {:?}", std::thread::current().id());
        });
    });

    Ok(cx.undefined())
}

fn js_functions(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let parse_int: Handle<JsFunction> = cx.global().get(&mut cx, "parseInt")?;
    let result: Handle<JsNumber> = parse_int
        .call_with(&mut cx)
        .arg(cx.string("42sss"))
        .apply(&mut cx)?;

    Ok(result)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("fibonacci", fibonacci)?;
    cx.export_function("cpu_threads", cpu_threads)?;
    cx.export_function("multi_thread", multi_thread)?;
    cx.export_function("js_functions", js_functions)?;

    Ok(())
}
