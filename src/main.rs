#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate neutron_star_constants;
extern crate neutron_star_rt;
extern crate neutron_star;
#[macro_use]
extern crate alloc;
pub mod trc20;

use neutron_star_rt::*;
use neutron_star_constants::*;
use neutron_star::testing::*;
use neutron_star::*;
use trc20::*;
use neutron_star::logging::*;
use neutron_star::storage::*;
use neutron_star::syscalls::*;

static mut foobar:u32 = 1234;

macro_rules! neutron_contract {
    ( $contract:ty ) => {
        #[no_mangle]
        extern "Rust" fn on_call() -> u32 {
            println!("Call!");
            match my_call(){
                Err(e) => {
                    println!("Contract signalled error! {:?}", e);
                    unsafe{
                        __revert_execution(0xFFFF_FFFF);
                    }
                },
                Ok(v) => {
                    unsafe{
                        __exit_execution(v); 
                    }
                }
            }

           /* #[cfg(test)]
            test_main();
            println!("Testing from println!");
            let mut contract = <$contract>::init();
            contract.try_exec();
            0
            */
            0
        }
        #[no_mangle]
        extern "Rust" fn on_create() -> u32 {
            println!("Deployment!");
            unsafe{
               // __exit_execution(0);
               0
            }
        }
    };
}

neutron_contract!(MyToken);

fn my_call() -> Result<u32, SystemError>{
    println!("Testing 101112");
    
    match pop_sccs(){
        Err(_) => {
            //no stack, so assume want to print the variable
            println!("Loading variable!");
            let v = load_state(&[0, 1, 2, 3]).unwrap();
            println!("variable: {:?}", v);
        },
        Ok(v) => {
            println!("storing...");
            println!("Storing: {:?}", v);
            store_state(&[0, 1, 2, 3], &v).unwrap();
            println!("Stored!");
        }
    }
    Ok(0)
}


pub struct MyToken{
}

impl MyToken{
    fn init() -> MyToken{
        MyToken{}
    }
    fn try_exec(&mut self){
        if (self as &mut dyn TRC20).try_exec(){
            return; 
        }
        //more contract interfaces can be implemented here
        //didn't find an interface that matches the call so panic
        panic!("No matching interface called!");
    }
}

impl TRC20 for MyToken{
    fn get_balance(&mut self, address: &NeutronShortAddress) -> u64{
        let mut x = vec![0u8, 1, 2];
        x.push(20);
        0
    }
    fn transfer(&mut self, to: &NeutronShortAddress, value: u64) -> bool{
        false
    }
}



#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(test)]

#[test_case]
fn test_test(){
    let mut t = MyToken{};
    println!("test case test_test");
    assert_eq!(t.get_balance_self(), 0);
}






