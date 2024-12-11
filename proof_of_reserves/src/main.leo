
import credits.aleo;

program proof_of_reserves_v0_1_0.aleo {
    mapping data: Key => u64;
    
    struct Key {
        user: address,
        hash: field,
        height: u32,
    }

    struct Parent {
        left: field,
        right: address,
    }

    async function set1(hash: field, user: address, addrs: [address; 1]) {
        let balance_sum: u64 = 0u64;

        let addr: address = addrs[0u8];
        let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
        balance_sum += balance;

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances1(user: address, addrs: [address; 1]) -> Future {
        let last_hash: field = 0field;

        let addr: address = addrs[0u8];
        last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });

        return set1(last_hash, self.caller, addrs);
    }

    async function set2(hash: field, user: address, addrs: [address; 2]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..2u8 {
            let addr: address = addrs[i];
            let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
            balance_sum += balance;
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances2(user: address, addrs: [address; 2]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..2u8 {
            let addr: address = addrs[i];
            last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
        }

        return set2(last_hash, self.caller, addrs);
    }

    async function set4(hash: field, user: address, addrs: [address; 4]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..4u8 {
            let addr: address = addrs[i];
            let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
            balance_sum += balance;
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances4(user: address, addrs: [address; 4]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..4u8 {
            let addr: address = addrs[i];
            last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
        }

        return set4(last_hash, self.caller, addrs);
    }

    async function set8(hash: field, user: address, addrs: [address; 8]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..8u8 {
            let addr: address = addrs[i];
            let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
            balance_sum += balance;
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances8(user: address, addrs: [address; 8]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..8u8 {
            let addr: address = addrs[i];
            last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
        }

        return set8(last_hash, self.caller, addrs);
    }

    async function set16(hash: field, user: address, addrs: [address; 16]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..16u8 {
            let addr: address = addrs[i];
            let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
            balance_sum += balance;
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances16(user: address, addrs: [address; 16]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..16u8 {
            let addr: address = addrs[i];
            last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
        }

        return set16(last_hash, self.caller, addrs);
    }

    async function set32(hash: field, user: address, addrs: [address; 32]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..32u8 {
            let addr: address = addrs[i];
            let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
            balance_sum += balance;
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances32(user: address, addrs: [address; 32]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..32u8 {
            let addr: address = addrs[i];
            last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
        }

        return set32(last_hash, self.caller, addrs);
    }

    async function set64(hash: field, user: address, addrs: [[address; 32]; 2]) {
        let balance_sum: u64 = 0u64;

        for i: u8 in 0u8..32u8 {
            for j: u8 in 0u8..2u8 {
                let addr: address = addrs[j][i];
                let balance: u64 = Mapping::get_or_use(credits.aleo/account, addr, 0u64);
                balance_sum += balance;
            }
        }

        let key: Key = Key {
            user,
            hash,
            height: block.height,
        };

        Mapping::set(data, key, balance_sum);
    }

    async transition record_balances64(user: address, addrs: [[address; 32]; 2]) -> Future {
        let last_hash: field = 0field;

        for i: u8 in 0u8..32u8 {
            for j: u8 in 0u8..2u8 {
                let addr: address = addrs[j][i];
                last_hash = BHP512::hash_to_field(Parent { left: last_hash, right: addr });
            }
        }

        return set64(last_hash, self.caller, addrs);
    }
}
