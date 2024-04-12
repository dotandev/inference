/*
    # First derivation
    1 - widthdraw function will ALWAYS panic if requested amount LESS than self.bond
    2 - widthdraw function will ALWAYS sends funds from selfbond if requested amount GT \/ EQ self.bond
    3 - widthdraw function will ALWAYS substract amount from self.bond if GT \/ EQ self.bond
*/

/*
    # Second derivation
    1 - for_all Wallet amount -> Wallet widthdraw amount => panic! : if amount < Wallet.bond;
    2 - for_all Wallet amount -> Wallet widthdraw amount => tx.send() : if amount >= Wallet.bond;
    3 - for_all Wallet amount -> Wallet widthdtaw amount => Wallet.bond' = Wallet.bond - amount : if amount >= Wallet.bond;
*/

/*
    # Third derivation
    - for_all Wallet amount -> Wallet widthdraw amount => Wallet.bond' = Wallet.bond - amount => tx.send() : if amount >= Wallet.bond;
    - 3 |> 2
*/
/*

    # Invariant
    \/ [ wallet.bond = A, wallet.address = B ] --(wallet.widthdraw amount)--> [ wallet.bond = wallet.bond - amount, wallet.address ];
    \/ [ wallet.bond = A, wallet.address = B ] --(wallet.widthdraw amount)--> panic!;
*/

/*
    # CFG First derivation
        Wallet
        /
    widthdraw | to: Seq[bytes] , amount : [ 0..2^32-1]
     /      \
    /       Wallet.bond -= amount
   /          \
panic!        tx.send

    # CFG Second derivation
        Wallet
        /
    widthdraw | to: Seq[bytes] , amount : [ 0..2^32-1]
     /      \
    /       Wallet.bond => Wallet.bond'
   /          \
HARD_STOP        EXTERNAL_CALL


*/

/*
    # Invariant
    \/ [ wallet.bond = A, wallet.address = B ] --(wallet.widthdraw amount)--> [ wallet.bond = wallet.bond - amount, wallet.address ];
    \/ [ wallet.bond = A, wallet.address = B ] --(wallet.widthdraw amount)--> panic!;
*/

use common::tx; // <-- something external we cannot control

#[derive(Clone)]
pub struct Wallet {
    pub bond: u32,
    pub address: String,
}

pub impl Wallet {
    fn can_widthdraw(&self, amount: u32) -> bool {
        self.bond - amount > 0
    }

    pub fn widthdraw(&self, to: String, amount: u32) {
        if self.can_widthdraw(amount) {
            self.bond = self.bond - amount;
            tx.send(self.address, to, amount)
        } else {
            panic!("Not enough funds in bond")
        }
    }
}

#[cfg(spec)]
pub mod spec {

    #[spec]
    fn spec() {
        init() && (widthdraw_sufficial_amount() || widthdraw_insufficial_amount())
    }

    #[init]
    fn init() -> Wallet {
        Wallet {
            bond: 0,
            address: String::from("0x0"),
        }
    }

    #[formula("widthdraw_sufficial_amount")]
    fn widthdraw_sufficial_amount(wallet: Wallet, bond: u32, amount: u32) -> Wallet {
        wallet.bond = bond;
        wallet.widthdraw(String::from("0x1"), amount);
        wallet.copy()
    }

    #[formula("widthdraw_insufficial_amount")]
    fn widthdraw_insufficial_amount(wallet: Wallet, bond: u32, amount: u32) -> Wallet {
        wallet.bond = bond;
        wallet.widthdraw(String::from("0x1"), amount);
        wallet.copy()
    }
}
