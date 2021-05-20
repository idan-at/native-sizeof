use neon::prelude::*;

enum Sizes {
    Character = 2,
    Boolean = 4,
    Number = 8,
}

fn sizeof_value<'a>(
    cx: &mut FunctionContext<'a>,
    handle: Handle<'a, JsValue>,
    seen: &mut Vec<Handle<'a, JsValue>>,
) -> u32 {
    if handle.is_a::<JsBoolean, _>(cx) {
        Sizes::Boolean as u32
    } else if handle.is_a::<JsNumber, _>(cx) {
        Sizes::Number as u32
    } else if handle.is_a::<JsString, _>(cx) {
        handle
            .downcast_or_throw::<JsString, _>(cx)
            .unwrap()
            .size(cx) as u32
            * Sizes::Character as u32
    } else if handle.is_a::<JsBuffer, _>(cx) {
        let buffer = handle.downcast_or_throw::<JsBuffer, _>(cx).unwrap();

        cx.borrow(&buffer, |buf| buf.as_slice::<u8>().len()) as u32
    } else if handle.is_a::<JsArray, _>(cx) {
        let vec = handle
            .downcast_or_throw::<JsArray, _>(cx)
            .unwrap()
            .to_vec(cx)
            .unwrap();

        vec.into_iter()
            .fold(0, |acc, value| acc + sizeof_value(cx, value, seen))
    } else if handle.is_a::<JsObject, _>(cx) {
        seen.push(handle.clone());

        let object = handle.downcast_or_throw::<JsObject, _>(cx).unwrap();

        let keys_handles = object
            .get_own_property_names(cx)
            .unwrap()
            .to_vec(cx)
            .unwrap();

        keys_handles.into_iter().fold(0, |acc, key_handle| {
            let value_handle = object.get(cx, key_handle).unwrap();

            let key_size = sizeof_value(cx, key_handle, seen);

            if seen.iter().any(|v| value_handle.strict_equals(cx, *v)) {
                return acc + key_size;
            }

            let value_size = sizeof_value(cx, value_handle, seen);

            acc + key_size + value_size
        })
    } else {
        0
    }
}

fn sizeof(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let argument = cx
        .argument_opt(0)
        .unwrap_or(cx.undefined().as_value(&mut cx));

    let mut seen: Vec<Handle<JsValue>> = Vec::new();
    let size = sizeof_value(&mut cx, argument, &mut seen);

    Ok(cx.number(size))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("sizeof", sizeof)?;
    Ok(())
}
