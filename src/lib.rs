//
// Compiler settings
//
#![allow(dead_code)]
//
// Mods
//
mod utils;
//
// Libraries
//
use wasm_bindgen::prelude::*;
use js_sys::Array;
use serde_wasm_bindgen::{from_value, to_value};
//
// Public
//
#[wasm_bindgen]
pub fn get_array_of_strings_combined_wo_arg_serde() -> JsValue {
    let mut array = Array::new();
    for item_index in (0..1000) {
        let item_string = format!( "{}_TEST", item_index );

        let item_value = match to_value( &item_string ) {
            Ok( result ) => result,
            Err( err ) => return JsValue::from_str( "Error" )
        };

        array.push( &item_value );
    }
    array.into()
}

#[wasm_bindgen]
pub fn get_string_unmodified( arg_string: &JsValue ) -> JsValue {
    match arg_string.as_string() {
        Some( string ) => JsValue::from( string ),
        None => JsValue::from_str( "Failed" )
    }
}

#[wasm_bindgen]
pub fn get_array_of_strings_combined_wo_arg() -> JsValue {
    let mut array = Array::new();
    for item_index in (0..1000) { array.push( &JsValue::from( format!( "{}_TEST", item_index ) ) ); }
    array.into()
}



#[wasm_bindgen]
pub fn calculation_without_return() {
    let mut count = 0;
    for item_index in 0..100000 { count += item_index; }
}

#[wasm_bindgen]
pub fn get_string_combined( arg_value: &JsValue ) -> Result< JsValue, JsValue > {
    if let Some( string_value ) = arg_value.as_string() {
        Ok( JsValue::from_str( [ string_value.as_str(), "_TEST_SUCCESSFUL" ].join( "" ).as_str() ) )
    } else { Err( JsValue::from_str( "Error: Failed to process string." ) ) }
}











































