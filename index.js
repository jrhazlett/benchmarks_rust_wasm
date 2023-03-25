import * as wasm from "rust-temp-bg";

console.log( "RAN" )

const get_array_of_strings_combined_wo_arg = () => {
    let array_of_strings = []
    let item_index = -1
    let int_length = 1000
    while ( ++item_index < int_length ) { array_of_strings.push( `${item_index}_TEST` ) }
    return array_of_strings
}

const get_string_unmodified = ( arg_string ) => { return arg_string }

const get_string_combined = ( arg_string ) => { return `${arg_string}_TEST_SUCCESSFUL` }

const calculation_without_return = () => {
    let count = 0
    let item_index = -1
    while ( ++item_index < 100000 ) { count += item_index; }
}

const get_array_of_strings_combined_wo_arg_serde = () => {
    let array_of_strings = []
    let item_index = -1
    let int_length = 1000
    while ( ++item_index < int_length ) { array_of_strings.push( `${item_index}_TEST` ) }
    return array_of_strings
}

let start_js = performance.now()
let result_js = get_array_of_strings_combined_wo_arg( "test" );
console.log( `duration (js) = ${(performance.now() - start_js) * 1000}` )

let start_wasm = performance.now()
let result_wasm = wasm.get_array_of_strings_combined_wo_arg_serde( "test" );
console.log( `duration (wasm) = ${(performance.now() - start_wasm) * 1000}` )












































