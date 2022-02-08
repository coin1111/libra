
<a name="0x1_BridgeEscrow"></a>

# Module `0x1::BridgeEscrow`



-  [Struct `AccountInfo`](#0x1_BridgeEscrow_AccountInfo)
-  [Resource `EscrowState`](#0x1_BridgeEscrow_EscrowState)
-  [Constants](#@Constants_0)
-  [Function `initialize_escrow`](#0x1_BridgeEscrow_initialize_escrow)
-  [Function `create_transfer_account_this`](#0x1_BridgeEscrow_create_transfer_account_this)
-  [Function `create_transfer_account`](#0x1_BridgeEscrow_create_transfer_account)
-  [Function `create_transfer_account_aux`](#0x1_BridgeEscrow_create_transfer_account_aux)
-  [Function `withdraw_from_escrow_this`](#0x1_BridgeEscrow_withdraw_from_escrow_this)
-  [Function `withdraw_from_escrow`](#0x1_BridgeEscrow_withdraw_from_escrow)
-  [Function `withdraw_from_escrow_aux`](#0x1_BridgeEscrow_withdraw_from_escrow_aux)
-  [Function `delete_transfer_account`](#0x1_BridgeEscrow_delete_transfer_account)
-  [Function `delete_unlocked`](#0x1_BridgeEscrow_delete_unlocked)
-  [Function `find_locked_idx`](#0x1_BridgeEscrow_find_locked_idx)
-  [Function `find_unlocked_idx`](#0x1_BridgeEscrow_find_unlocked_idx)
-  [Function `get_locked_at`](#0x1_BridgeEscrow_get_locked_at)
-  [Function `get_escrow_balance`](#0x1_BridgeEscrow_get_escrow_balance)
-  [Function `get_locked_length`](#0x1_BridgeEscrow_get_locked_length)
-  [Function `get_unlocked_length`](#0x1_BridgeEscrow_get_unlocked_length)
-  [Function `get_sender_this`](#0x1_BridgeEscrow_get_sender_this)
-  [Function `get_sender_other`](#0x1_BridgeEscrow_get_sender_other)
-  [Function `get_receiver_this`](#0x1_BridgeEscrow_get_receiver_this)
-  [Function `get_receiver_other`](#0x1_BridgeEscrow_get_receiver_other)
-  [Function `get_balance`](#0x1_BridgeEscrow_get_balance)
-  [Function `get_transfer_id`](#0x1_BridgeEscrow_get_transfer_id)


<pre><code><b>use</b> <a href="Diem.md#0x1_Diem">0x1::Diem</a>;
<b>use</b> <a href="DiemAccount.md#0x1_DiemAccount">0x1::DiemAccount</a>;
<b>use</b> <a href="DiemSystem.md#0x1_DiemSystem">0x1::DiemSystem</a>;
<b>use</b> <a href="GAS.md#0x1_GAS">0x1::GAS</a>;
<b>use</b> <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option">0x1::Option</a>;
<b>use</b> <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer">0x1::Signer</a>;
<b>use</b> <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_BridgeEscrow_AccountInfo"></a>

## Struct `AccountInfo`



<pre><code><b>struct</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a> has <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender_this: address</code>
</dt>
<dd>

</dd>
<dt>
<code>sender_other: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>receiver_this: address</code>
</dt>
<dd>

</dd>
<dt>
<code>receiver_other: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>transfer_id: vector&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x1_BridgeEscrow_EscrowState"></a>

## Resource `EscrowState`



<pre><code><b>struct</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> has key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>locked: vector&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>unlocked: vector&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>tokens: <a href="Diem.md#0x1_Diem_Diem">Diem::Diem</a>&lt;<a href="GAS.md#0x1_GAS_GAS">GAS::GAS</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x1_BridgeEscrow_ERROR_ALREADY_ACCOUNT_EXISTS"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_ALREADY_ACCOUNT_EXISTS">ERROR_ALREADY_ACCOUNT_EXISTS</a>: u64 = 3001;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_AMOUNT_MUST_BE_POSITIVE"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_AMOUNT_MUST_BE_POSITIVE">ERROR_AMOUNT_MUST_BE_POSITIVE</a>: u64 = 3003;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_BRIDGE_STORE_EXISTS"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_BRIDGE_STORE_EXISTS">ERROR_BRIDGE_STORE_EXISTS</a>: u64 = 3000;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>: u64 = 3004;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_INVALID_TRANSFER_ID"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INVALID_TRANSFER_ID">ERROR_INVALID_TRANSFER_ID</a>: u64 = 3309;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_LOCKED_EMPTY"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_LOCKED_EMPTY">ERROR_LOCKED_EMPTY</a>: u64 = 3308;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_MUST_BE_VALIDATOR"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_MUST_BE_VALIDATOR">ERROR_MUST_BE_VALIDATOR</a>: u64 = 3311;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT">ERROR_NO_ESCROW_ACCOUNT</a>: u64 = 3006;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_NO_RECEIVER_ACCOUNT"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_RECEIVER_ACCOUNT">ERROR_NO_RECEIVER_ACCOUNT</a>: u64 = 3312;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS">ERROR_TRANSFER_ID_EXISTS</a>: u64 = 3310;
</code></pre>



<a name="0x1_BridgeEscrow_ZERO_ADDRESS"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ZERO_ADDRESS">ZERO_ADDRESS</a>: address = 0;
</code></pre>



<a name="0x1_BridgeEscrow_initialize_escrow"></a>

## Function `initialize_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_initialize_escrow">initialize_escrow</a>(escrow: &signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_initialize_escrow">initialize_escrow</a>(escrow: &signer) {
    <b>let</b> escrow_addr = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(escrow);
    <b>assert</b>(!<b>exists</b>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_addr), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_BRIDGE_STORE_EXISTS">ERROR_BRIDGE_STORE_EXISTS</a>);
    move_to&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow, <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>{
        locked: <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(),
        unlocked: <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(),
        tokens: <a href="Diem.md#0x1_Diem_zero">Diem::zero</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(),
    });
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_create_transfer_account_this"></a>

## Function `create_transfer_account_this`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_this">create_transfer_account_this</a>(escrow: address, sender_address: &signer, receiver_address: address, amount: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_this">create_transfer_account_this</a>(escrow: address,
                                   sender_address: &signer,
                                   receiver_address: address,
                                   amount: u64,
                                   transfer_id: vector&lt;u8&gt;) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_aux">create_transfer_account_aux</a>(escrow, sender_address,receiver_address,
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;(), amount, transfer_id)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_create_transfer_account"></a>

## Function `create_transfer_account`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account">create_transfer_account</a>(escrow: address, sender_address: &signer, receiver_address: vector&lt;u8&gt;, amount: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account">create_transfer_account</a>(escrow: address,
                                        sender_address: &signer,
                                        receiver_address: vector&lt;u8&gt;,
                                        amount: u64,
                                        transfer_id: vector&lt;u8&gt;) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_aux">create_transfer_account_aux</a>(escrow, sender_address,<a href="BridgeEscrow.md#0x1_BridgeEscrow_ZERO_ADDRESS">ZERO_ADDRESS</a>,
        receiver_address, amount, transfer_id)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_create_transfer_account_aux"></a>

## Function `create_transfer_account_aux`



<pre><code><b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_aux">create_transfer_account_aux</a>(escrow: address, sender: &signer, receiver_this: address, receiver_other: vector&lt;u8&gt;, amount: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_aux">create_transfer_account_aux</a>(escrow: address,
                                   sender: &signer,
                                   receiver_this: address,
                                   receiver_other: vector&lt;u8&gt;,
                                   amount: u64,
                                   transfer_id: vector&lt;u8&gt;) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow, &transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_none">Option::is_none</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS">ERROR_TRANSFER_ID_EXISTS</a>);

    // validate arguments
    <b>assert</b> (amount &gt; 0, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_AMOUNT_MUST_BE_POSITIVE">ERROR_AMOUNT_MUST_BE_POSITIVE</a>);

    // sender has enough funds
    <b>let</b> sender_this = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>assert</b>(<a href="DiemAccount.md#0x1_DiemAccount_balance">DiemAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(sender_this) &gt;= amount, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>);

    // escrow account <b>exists</b>
    <b>assert</b> (<b>exists</b>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT">ERROR_NO_ESCROW_ACCOUNT</a>);

    // receiver_other must be non-empty OR receiver must <b>exists</b> and have no -
    <b>if</b> (<a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&receiver_other) == 0) {
        <b>assert</b>(<a href="DiemAccount.md#0x1_DiemAccount_balance">DiemAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(receiver_this) &gt; 0, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_RECEIVER_ACCOUNT">ERROR_NO_RECEIVER_ACCOUNT</a>);
    };

    // 1. <b>move</b> funds from user <b>to</b> escrow account
    <b>let</b> with_cap = <a href="DiemAccount.md#0x1_DiemAccount_extract_withdraw_capability">DiemAccount::extract_withdraw_capability</a>(sender);
    <b>let</b> tokens = <a href="DiemAccount.md#0x1_DiemAccount_withdraw_tokens">DiemAccount::withdraw_tokens</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(&with_cap, escrow, amount, x"");
    <a href="DiemAccount.md#0x1_DiemAccount_restore_withdraw_capability">DiemAccount::restore_withdraw_capability</a>(with_cap);

    // 2. <b>update</b> escrow state

    // <b>update</b> escrow balance
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow);
    <a href="Diem.md#0x1_Diem_deposit">Diem::deposit</a>(&<b>mut</b> state.tokens,tokens);

    // create an entry in locked vector
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.locked, <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>{
        sender_this: sender_this,
        sender_other: <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;(),
        receiver_this: receiver_this,
        receiver_other: receiver_other,
        balance: amount,
        transfer_id: transfer_id,
    });
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_withdraw_from_escrow_this"></a>

## Function `withdraw_from_escrow_this`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_this">withdraw_from_escrow_this</a>(sender: &signer, escrow_address: address, sender_address: address, receiver_address: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_this">withdraw_from_escrow_this</a>(sender: &signer,
                                escrow_address: address,
                                sender_address: address, // sender on this  chain
                                receiver_address:address, // receiver on this chain
                                balance: u64, // balance <b>to</b> transfer
                                transfer_id: vector&lt;u8&gt;, // transfer_id
) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_aux">withdraw_from_escrow_aux</a>(sender,escrow_address,sender_address, <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;(), receiver_address, balance, transfer_id)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_withdraw_from_escrow"></a>

## Function `withdraw_from_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow">withdraw_from_escrow</a>(sender: &signer, escrow_address: address, sender_address: vector&lt;u8&gt;, receiver_address: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow">withdraw_from_escrow</a>(sender: &signer,
                                     escrow_address: address,
                                     sender_address: vector&lt;u8&gt;, // sender on the other chain
                                     receiver_address:address, // receiver on this chain
                                     balance: u64, // balance <b>to</b> transfer
                                     transfer_id: vector&lt;u8&gt;, // transfer_id
) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_aux">withdraw_from_escrow_aux</a>(sender,escrow_address,<a href="BridgeEscrow.md#0x1_BridgeEscrow_ZERO_ADDRESS">ZERO_ADDRESS</a>, sender_address, receiver_address, balance, transfer_id)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_withdraw_from_escrow_aux"></a>

## Function `withdraw_from_escrow_aux`



<pre><code><b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_aux">withdraw_from_escrow_aux</a>(sender: &signer, escrow_address: address, sender_this: address, sender_other: vector&lt;u8&gt;, receiver_this: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_aux">withdraw_from_escrow_aux</a>(sender: &signer,
                                escrow_address: address,
                                sender_this: address, // sender on this  chain
                                sender_other: vector&lt;u8&gt;, // sender on the other chain
                                receiver_this:address, // receiver on this chain
                                balance: u64, // balance <b>to</b> transfer
                                transfer_id: vector&lt;u8&gt;, // transfer_id
                                ) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <b>let</b> sender_address= <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>assert</b>(<a href="DiemSystem.md#0x1_DiemSystem_is_validator">DiemSystem::is_validator</a>(sender_address) == <b>true</b> ||
           sender_address == escrow_address , <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_MUST_BE_VALIDATOR">ERROR_MUST_BE_VALIDATOR</a>);

    // check that transfer id is not present
    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_unlocked_idx">find_unlocked_idx</a>( escrow_address, &transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_none">Option::is_none</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS">ERROR_TRANSFER_ID_EXISTS</a>);

    // <b>update</b> escrow state
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;( escrow_address);

    // escrow has enough funds
    <b>assert</b>(<a href="Diem.md#0x1_Diem_get_value">Diem::get_value</a>(&state.tokens) &gt;= balance, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>);

    // withdraw tokens from escrow
    <b>let</b> tokens = <a href="Diem.md#0x1_Diem_withdraw">Diem::withdraw</a>(&<b>mut</b> state.tokens,balance);

    // add entry <b>to</b> unlocked <b>to</b> indicate that funds were transferred
    <b>let</b> ai = <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a> {
        sender_this: sender_this,
        sender_other: sender_other,
        receiver_this: <b>copy</b> receiver_this,
        receiver_other: <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;u8&gt;(),
        balance: balance,
        transfer_id: transfer_id,
    };
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.unlocked, ai);

    // <b>move</b> funds from escrow <b>to</b> user account
    <a href="DiemAccount.md#0x1_DiemAccount_deposit_tokens">DiemAccount::deposit_tokens</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(sender, escrow_address, receiver_this, tokens, x"", x"");
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_delete_transfer_account"></a>

## Function `delete_transfer_account`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_transfer_account">delete_transfer_account</a>(sender: &signer, escrow_address: address, transfer_id: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_transfer_account">delete_transfer_account</a>(sender: &signer, escrow_address: address, transfer_id: &vector&lt;u8&gt;)
<b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> sender_address= <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>assert</b>(<a href="DiemSystem.md#0x1_DiemSystem_is_validator">DiemSystem::is_validator</a>(sender_address) == <b>true</b> ||
           sender_address == escrow_address , <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_MUST_BE_VALIDATOR">ERROR_MUST_BE_VALIDATOR</a>);

    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow_address, transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_some">Option::is_some</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INVALID_TRANSFER_ID">ERROR_INVALID_TRANSFER_ID</a>);
    <b>let</b> idx = <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_borrow">Option::borrow</a>(&idx_opt);
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.locked, *idx);
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_delete_unlocked"></a>

## Function `delete_unlocked`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_unlocked">delete_unlocked</a>(sender: &signer, escrow_address: address, transfer_id: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_unlocked">delete_unlocked</a>(sender: &signer, escrow_address: address, transfer_id: &vector&lt;u8&gt;)
<b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> sender_address= <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>assert</b>(<a href="DiemSystem.md#0x1_DiemSystem_is_validator">DiemSystem::is_validator</a>(sender_address) == <b>true</b> ||
           sender_address == escrow_address , <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_MUST_BE_VALIDATOR">ERROR_MUST_BE_VALIDATOR</a>);

    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_unlocked_idx">find_unlocked_idx</a>(escrow_address, transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_some">Option::is_some</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INVALID_TRANSFER_ID">ERROR_INVALID_TRANSFER_ID</a>);
    <b>let</b> idx = <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_borrow">Option::borrow</a>(&idx_opt);
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_remove">Vector::remove</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.unlocked, *idx);
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_find_locked_idx"></a>

## Function `find_locked_idx`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow_address: address, transfer_id: &vector&lt;u8&gt;): <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_Option">Option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow_address: address, transfer_id: &vector&lt;u8&gt;):
<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option">Option</a>&lt;u64&gt; <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&state.locked);
    <b>while</b> (i &lt; n) {
        <b>let</b> ai = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&state.locked, i);
        <b>if</b> (*&ai.transfer_id == *transfer_id) <b>return</b> <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_some">Option::some</a>(i);
        i = i + 1
    };
    <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_none">Option::none</a>()
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_find_unlocked_idx"></a>

## Function `find_unlocked_idx`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_unlocked_idx">find_unlocked_idx</a>(escrow_address: address, transfer_id: &vector&lt;u8&gt;): <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_Option">Option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_unlocked_idx">find_unlocked_idx</a>(escrow_address: address,transfer_id: &vector&lt;u8&gt;):
    <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option">Option</a>&lt;u64&gt; <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&state.unlocked);
    <b>while</b> (i &lt; n) {
        <b>let</b> ai = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&state.unlocked, i);
        <b>if</b> (*&ai.transfer_id == *transfer_id) <b>return</b> <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_some">Option::some</a>(i);
        i = i + 1
    };
    <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_none">Option::none</a>()
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_locked_at"></a>

## Function `get_locked_at`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_at">get_locked_at</a>(escrow_address: address, index: u64): <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_at">get_locked_at</a>(escrow_address: address, index: u64): <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a> <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <b>assert</b>(<a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_length">get_locked_length</a>(escrow_address) &gt; index, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_LOCKED_EMPTY">ERROR_LOCKED_EMPTY</a>);
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <b>let</b> ai = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&state.locked, index);
    *ai
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_escrow_balance"></a>

## Function `get_escrow_balance`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_escrow_balance">get_escrow_balance</a>(escrow: address): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_escrow_balance">get_escrow_balance</a>(escrow: address): u64 <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow);
    <a href="Diem.md#0x1_Diem_value">Diem::value</a>(&state.tokens)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_locked_length"></a>

## Function `get_locked_length`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_length">get_locked_length</a>(escrow: address): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_length">get_locked_length</a>(escrow: address): u64 <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow);
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&state.locked)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_unlocked_length"></a>

## Function `get_unlocked_length`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_unlocked_length">get_unlocked_length</a>(escrow: address): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_unlocked_length">get_unlocked_length</a>(escrow: address): u64 <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow);
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&state.unlocked)
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_sender_this"></a>

## Function `get_sender_this`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_this">get_sender_this</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): address
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_this">get_sender_this</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): address {
    *&ai.sender_this
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_sender_other"></a>

## Function `get_sender_other`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_other">get_sender_other</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_other">get_sender_other</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): vector&lt;u8&gt; {
    *&ai.sender_other
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_receiver_this"></a>

## Function `get_receiver_this`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_this">get_receiver_this</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): address
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_this">get_receiver_this</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): address {
    *&ai.receiver_this
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_receiver_other"></a>

## Function `get_receiver_other`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_other">get_receiver_other</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_other">get_receiver_other</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): vector&lt;u8&gt; {
    *&ai.receiver_other
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_balance"></a>

## Function `get_balance`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_balance">get_balance</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_balance">get_balance</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): u64 {
    *&ai.balance
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_transfer_id"></a>

## Function `get_transfer_id`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_transfer_id">get_transfer_id</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_transfer_id">get_transfer_id</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): vector&lt;u8&gt; {
    *&ai.transfer_id
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/diem/dip/blob/main/dips/dip-2.md
[ROLE]: https://github.com/diem/dip/blob/main/dips/dip-2.md#roles
[PERMISSION]: https://github.com/diem/dip/blob/main/dips/dip-2.md#permissions
