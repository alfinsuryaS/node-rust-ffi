
/*
  Doc: https://devstore.io/js/node-ffi

  For types check: 
  https://devstore.io/js/node-ffi 

  It's mean, double_input type is int -> return int
  -> 'double_input': [ 'int', [ 'int' ] ]
  -> if hasn't parameter type: ['int', []]
*/

import * as ffi from 'ffi';

export default ffi.Library('./native/target/release/libembed', {
  'generate_rand_number': ['int', []]
});
