import lib from './mod';

function promise_0(){
    return new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_point_values());
        }, 100);
    });
};

function promise_1() {
    return new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_password(10));
        }, 300);
    });
};

function promise_2() {
    return new Promise(resolve => {
        setTimeout(() => {
            resolve(lib.generate_number_range_int(0, 10));
        }, 400);
    });
};

async function start(){
  await Promise.all([
      promise_0(),
      promise_1(),
      promise_2()
  ]);
};

start();