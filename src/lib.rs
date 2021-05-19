use neon::prelude::*;

enum Sizes {
    Character = 2,
    Boolean = 4,
    Number = 8,
}

fn sizeof_value(cx: &mut FunctionContext, handle: Handle<JsValue>) -> u32 {
    if handle.is_a::<JsBoolean, _>(cx) {
        Sizes::Boolean as u32
    } else if handle.is_a::<JsNumber, _>(cx) {
        Sizes::Number as u32
    } else if handle.is_a::<JsString, _>(cx) {
        handle.downcast_or_throw::<JsString, _>(cx).unwrap().size(cx) as u32 * Sizes::Character as u32
    } else if handle.is_a::<JsBuffer, _>(cx) {
        let buffer = handle.downcast_or_throw::<JsBuffer, _>(cx).unwrap();

        cx.borrow(&buffer, |buf| buf.as_slice::<u8>().len()) as u32
    } else if handle.is_a::<JsArray, _>(cx) {
        let vec = handle.downcast_or_throw::<JsArray, _>(cx).unwrap().to_vec(cx).unwrap();

        vec.into_iter().fold(0, |acc, value| acc + sizeof_value(cx, value))
    } else if handle.is_a::<JsObject, _>(cx) {
        let object = handle.downcast_or_throw::<JsObject, _>(cx).unwrap();

        let keys = object.get_own_property_names(cx).unwrap().to_vec(cx).unwrap();

        keys.into_iter().fold(0, |acc, key| {
            let value = object.get(cx, key).unwrap();

            let key_size = sizeof_value(cx, key);
            let value_size = sizeof_value(cx, value);

            acc + key_size + value_size
        })
    }
    else {
        0
    }
}

fn sizeof(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let argument = cx.argument_opt(0).unwrap_or(cx.undefined().as_value(&mut cx));

    let size = sizeof_value(&mut cx, argument);

    Ok(cx.number(size))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sizeof", sizeof)?;
    Ok(())
}
