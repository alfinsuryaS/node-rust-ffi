
/* For types check: 
  https://devstore.io/js/node-ffi 

  It's mean, double_input type is int -> return int
  'double_input': [ 'int', [ 'int' ] ] i

*/

import * as ffi from 'ffi';

export default ffi.Library('./native/target/release/libembed', {
  'double_input': [ 'int', [ 'int' ] ]
});
