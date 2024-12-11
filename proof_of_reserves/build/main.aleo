import credits.aleo;
program proof_of_reserves_v0_1_0.aleo;

struct Key:
    user as address;
    hash as field;
    height as u32;

struct Parent:
    left as field;
    right as address;

mapping data:
    key as Key.public;
    value as u64.public;

function record_balances1:
    input r0 as address.private;
    input r1 as [address; 1u32].private;
    cast 0field r1[0u32] into r2 as Parent;
    hash.bhp512 r2 into r3 as field;
    async record_balances1 r3 self.caller r1 into r4;
    output r4 as proof_of_reserves_v0_1_0.aleo/record_balances1.future;

finalize record_balances1:
    input r0 as field.public;
    input r1 as address.public;
    input r2 as [address; 1u32].public;
    get.or_use credits.aleo/account[r2[0u32]] 0u64 into r3;
    add 0u64 r3 into r4;
    cast r1 r0 block.height into r5 as Key;
    set r4 into data[r5];

function record_balances2:
    input r0 as address.private;
    input r1 as [address; 2u32].private;
    cast 0field r1[0u32] into r2 as Parent;
    hash.bhp512 r2 into r3 as field;
    cast r3 r1[1u32] into r4 as Parent;
    hash.bhp512 r4 into r5 as field;
    async record_balances2 r5 self.caller r1 into r6;
    output r6 as proof_of_reserves_v0_1_0.aleo/record_balances2.future;

finalize record_balances2:
    input r0 as field.public;
    input r1 as address.public;
    input r2 as [address; 2u32].public;
    get.or_use credits.aleo/account[r2[0u32]] 0u64 into r3;
    add 0u64 r3 into r4;
    get.or_use credits.aleo/account[r2[1u32]] 0u64 into r5;
    add r4 r5 into r6;
    cast r1 r0 block.height into r7 as Key;
    set r6 into data[r7];
