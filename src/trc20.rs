extern crate neutron_star;
extern crate neutron_star_constants;

use neutron_star::syscalls::*;
use neutron_star_constants::*;


pub trait TRC20{
    fn try_exec_internal(&mut self) -> Result<bool, SystemError>{
        let functionid = pop_sccs_u64()?;
        match functionid{
            1 => {
                //get balance self
                let r = self.get_balance_self();
                push_sccs_u64(r)?;
            },
            2 => {
                //get balance of address
                let a = pop_sccs_address()?;
                let r = self.get_balance(&a);
                push_sccs_u64(r)?;
            },
            3 => {
                //transfer tokens
                let to = pop_sccs_address()?;
                let value = pop_sccs_u64()?;
                let r = self.transfer(&to, value);
                if r {
                    push_sccs_u8(1)?;
                }else{
                    push_sccs_u8(0)?;
                }
            },
            _ => {
                push_sccs_u64(functionid)?;
                return Ok(false);
            }
        }
        Ok(true)
    }
    fn try_exec(&mut self) -> bool{
        self.try_exec_internal().unwrap()
    }
    fn get_balance_self(&mut self) -> u64{
        let a = get_self_address();
        return self.get_balance(&a);
    }
    fn get_balance(&mut self, address: &NeutronShortAddress) -> u64;
    fn transfer(&mut self, to: &NeutronShortAddress, value: u64) -> bool;
    
}

