import lib from './mod';

async function start(){
  await Promise.all([
    new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_point_values());
        }, 100);
    }),
    new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_password(10));
        }, 300);
    }),
    new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_number_range_int(0, 10));
        }, 400);
    })
  ]);
};

start();