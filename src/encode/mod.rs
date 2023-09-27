use js_sys;
use wasm_bindgen::prelude::*;

use crate::shared::tokens::TOKENS;

fn is_date(val: &JsValue) -> bool {
    let date: js_sys::Date = val.to_owned().into();
    let ctx: js_sys::Function = js_sys::Reflect::get(&js_sys::Object::get_prototype_of(&js_sys::Object::new()), &js_sys::JsString::from("toString")).unwrap().into();
    let is_date =  js_sys::Function::call0(&ctx, &date).unwrap() == "[object Date]";

    let is_invalid_date_string = &date.to_string() == "Invalid Date";
    let is_nan = &date.value_of().is_nan();

    is_date && !is_invalid_date_string && !is_nan
}


#[wasm_bindgen]
pub fn encode(source: JsValue) -> js_sys::JsString {
    return match &source {
       source if JsValue::is_null(&source) => {
          let token = TOKENS.get("null").unwrap();
          js_sys::JsString::from(token.as_str())
       },
       source if JsValue::is_string(&source) => {
            let token: String = TOKENS.get("string").unwrap().to_owned();
            let val: String = js_sys::JSON::stringify(&source).unwrap().into();
            js_sys::JsString::from(token + val.as_str())
       },
       source if JsValue::is_bigint(&source) => {
            let token: String = TOKENS.get("bigint").unwrap().to_owned();
            let val = source.unchecked_into_f64().to_string();
        
            js_sys::JsString::from(token + val.as_str())
       },
       source if JsValue::is_undefined(&source) => {
            let token = TOKENS.get("undefined").unwrap();

            js_sys::JsString::from(token.as_str())
       },
       source if is_date(source) => {
            let token = TOKENS.get("date").unwrap();
            let val: js_sys::Date = source.clone().unchecked_into();

            js_sys::JsString::from(token.to_owned() + val.to_iso_string().as_string().unwrap().as_str())
       },
       source if js_sys::Number::is_nan(&source) => {
          let token = TOKENS.get("nan").unwrap();

          js_sys::JsString::from(token.as_str())
       },
       source if JsValue::js_typeof(&source) == "boolean" => {
            let token = TOKENS.get("boolean").unwrap();
            let val: i32 = if *source == JsValue::TRUE {1} else {0};

            js_sys::JsString::from(token.to_owned() + val.to_string().as_str())
       },
       source if JsValue::js_typeof(&source) == "function" => {
            let token = TOKENS.get("function").unwrap();
            let js_fn: &js_sys::Function = source.as_ref().unchecked_ref();
            let fn_name: String = js_fn.name().into();
            let fn_string: String = js_fn.to_string().into();

            js_sys::JsString::from(token.to_owned() + "[name=" + fn_name.as_str() + "]" + fn_string.as_str() + "$")
       },
       source if JsValue::js_typeof(&source) == "number" && !js_sys::is_finite(&source) => {
            let token = TOKENS.get("infinity").unwrap();
            let val: i32 = if source.unchecked_into_f64() > 0.0 {1} else {-1};
            
            js_sys::JsString::from(token.to_owned() + val.to_string().as_str())
       },
       source if JsValue::js_typeof(&source) == "number" => {
            let token = TOKENS.get("number").unwrap();
            let val = source.unchecked_into_f64().to_string();
        
            js_sys::JsString::from(token.to_owned() + val.as_str())
       },
        source if JsValue::is_array(&source) => {
                let token = TOKENS.get("arraystart").unwrap().to_owned();
                let js_array: Vec<JsValue> = js_sys::Array::from(source).to_vec();
                let mut result: String = String::from("");

                for val in js_array.iter() {
                    result += &encode(val.clone()).as_string().unwrap();
                    result += ",";
                }

                js_sys::JsString::from(token.to_owned() + "[" + result.as_str() + "]")
        },
        source if JsValue::is_object(&source) => {
            let token = TOKENS.get("objectstart").unwrap().to_owned();

            let js_obj: js_sys::Object = source.as_ref().clone().unchecked_into();
            let js_obj_keys: Vec<JsValue> = js_sys::Object::keys(&js_obj).to_vec();
            let mut result: Vec<String> = vec![];

            for key in js_obj_keys.iter() {
                let obj_val = js_sys::Reflect::get(&js_obj, key).unwrap();

                let value: String = encode(key.clone()).as_string().unwrap().to_owned() + ":" + encode(obj_val).as_string().unwrap().to_owned().as_str();
                
                result.push(value);
            }

            let val: String = result.join(",");
    
            js_sys::JsString::from(token + "{" + &val + "}")
        },
       _ => {
          
            js_sys::JSON::stringify(&source).unwrap()
       }
    }
}