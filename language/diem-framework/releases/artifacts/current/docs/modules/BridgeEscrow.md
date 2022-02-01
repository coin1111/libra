
<a name="0x1_BridgeEscrow"></a>

# Module `0x1::BridgeEscrow`



-  [Struct `AccountInfo`](#0x1_BridgeEscrow_AccountInfo)
-  [Resource `EscrowState`](#0x1_BridgeEscrow_EscrowState)
-  [Constants](#@Constants_0)
-  [Function `initialize_escrow`](#0x1_BridgeEscrow_initialize_escrow)
-  [Function `create_transfer_account`](#0x1_BridgeEscrow_create_transfer_account)
-  [Function `withdraw_from_escrow`](#0x1_BridgeEscrow_withdraw_from_escrow)
-  [Function `delete_transfer_account`](#0x1_BridgeEscrow_delete_transfer_account)
-  [Function `delete_unlocked`](#0x1_BridgeEscrow_delete_unlocked)
-  [Function `find_locked_idx`](#0x1_BridgeEscrow_find_locked_idx)
-  [Function `find_unlocked_idx`](#0x1_BridgeEscrow_find_unlocked_idx)
-  [Function `get_locked_at`](#0x1_BridgeEscrow_get_locked_at)
-  [Function `get_unlocked_at`](#0x1_BridgeEscrow_get_unlocked_at)
-  [Function `get_escrow_balance`](#0x1_BridgeEscrow_get_escrow_balance)
-  [Function `get_locked_length`](#0x1_BridgeEscrow_get_locked_length)
-  [Function `get_unlocked_length`](#0x1_BridgeEscrow_get_unlocked_length)
-  [Function `get_sender_from_ai`](#0x1_BridgeEscrow_get_sender_from_ai)
-  [Function `get_receiver_from_ai`](#0x1_BridgeEscrow_get_receiver_from_ai)
-  [Function `get_balance_from_ai`](#0x1_BridgeEscrow_get_balance_from_ai)
-  [Function `get_transfer_id_from_ai`](#0x1_BridgeEscrow_get_transfer_id_from_ai)


<pre><code><b>use</b> <a href="DiemAccount.md#0x1_DiemAccount">0x1::DiemAccount</a>;
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
<code>sender: address</code>
</dt>
<dd>

</dd>
<dt>
<code>receiver: address</code>
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
<code>balance: u64</code>
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



<a name="0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT">ERROR_NO_ESCROW_ACCOUNT</a>: u64 = 3006;
</code></pre>



<a name="0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS"></a>



<pre><code><b>const</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS">ERROR_TRANSFER_ID_EXISTS</a>: u64 = 3310;
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
        balance: 0,
    });
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_create_transfer_account"></a>

## Function `create_transfer_account`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account">create_transfer_account</a>(escrow: address, sender: &signer, receiver: address, amount: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account">create_transfer_account</a>(escrow: address,
                                   sender: &signer,
                                   receiver: address,
                                   amount: u64,
                                   transfer_id: vector&lt;u8&gt;) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow, &transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_none">Option::is_none</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_TRANSFER_ID_EXISTS">ERROR_TRANSFER_ID_EXISTS</a>);

    // validate arguments
    <b>assert</b> (amount &gt; 0, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_AMOUNT_MUST_BE_POSITIVE">ERROR_AMOUNT_MUST_BE_POSITIVE</a>);

    // sender has enough funds
    <b>let</b> address = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender);
    <b>assert</b>(<a href="DiemAccount.md#0x1_DiemAccount_balance">DiemAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(address) &gt;= amount, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>);

    // escrow account <b>exists</b>
    <b>assert</b> (<b>exists</b>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_NO_ESCROW_ACCOUNT">ERROR_NO_ESCROW_ACCOUNT</a>);

    // 1. <b>move</b> funds from user <b>to</b> escrow account
    <b>let</b> with_cap = <a href="DiemAccount.md#0x1_DiemAccount_extract_withdraw_capability">DiemAccount::extract_withdraw_capability</a>(sender);
    <a href="DiemAccount.md#0x1_DiemAccount_pay_from">DiemAccount::pay_from</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(&with_cap, escrow, amount, x"", x"");
    <a href="DiemAccount.md#0x1_DiemAccount_restore_withdraw_capability">DiemAccount::restore_withdraw_capability</a>(with_cap);

    // 2. <b>update</b> escrow state

    // <b>update</b> escrow balance
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow);
    *&<b>mut</b> state.balance = *&<b>mut</b> state.balance + amount;

    // create an entry in locked vector
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.locked, <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>{
        sender: <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(sender),
        receiver: receiver,
        balance: amount,
        transfer_id: transfer_id,
    });
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_withdraw_from_escrow"></a>

## Function `withdraw_from_escrow`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow">withdraw_from_escrow</a>(escrow: &signer, transfer_id: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow">withdraw_from_escrow</a>(escrow: &signer, transfer_id:&vector&lt;u8&gt;) <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <b>let</b> escrow_address = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(escrow);

    <b>let</b> idx_opt = <a href="BridgeEscrow.md#0x1_BridgeEscrow_find_locked_idx">find_locked_idx</a>(escrow_address,transfer_id);
    <b>assert</b>(<a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_is_some">Option::is_some</a>(&idx_opt), <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INVALID_TRANSFER_ID">ERROR_INVALID_TRANSFER_ID</a>);
    <b>let</b> idx = <a href="../../../../../../move-stdlib/docs/Option.md#0x1_Option_borrow">Option::borrow</a>(&idx_opt);

    <b>let</b> ai = <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_locked_at">get_locked_at</a>(escrow_address, *idx);

    // escrow has enough funds
    <b>assert</b>(<a href="DiemAccount.md#0x1_DiemAccount_balance">DiemAccount::balance</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(escrow_address) &gt;= ai.balance, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>);


    // 1. <b>move</b> funds from escrow <b>to</b> user account
    <b>let</b> with_cap = <a href="DiemAccount.md#0x1_DiemAccount_extract_withdraw_capability">DiemAccount::extract_withdraw_capability</a>(escrow);
    <a href="DiemAccount.md#0x1_DiemAccount_pay_from">DiemAccount::pay_from</a>&lt;<a href="GAS.md#0x1_GAS">GAS</a>&gt;(&with_cap, ai.receiver, ai.balance, x"", x"");
    <a href="DiemAccount.md#0x1_DiemAccount_restore_withdraw_capability">DiemAccount::restore_withdraw_capability</a>(with_cap);

    // 2. <b>update</b> escrow state
    // <b>update</b> balance
    <b>let</b> state = borrow_global_mut&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <b>assert</b>(state.balance &gt;= ai.balance, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_INSUFFICIENT_BALANCE">ERROR_INSUFFICIENT_BALANCE</a>);
    *&<b>mut</b> state.balance = *&<b>mut</b> state.balance - ai.balance;

    // add entry <b>to</b> unlocked <b>to</b> indicate that funds were transferred
    <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>&gt;(&<b>mut</b> state.unlocked, ai);
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_delete_transfer_account"></a>

## Function `delete_transfer_account`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_transfer_account">delete_transfer_account</a>(escrow: &signer, transfer_id: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_transfer_account">delete_transfer_account</a>(escrow: &signer, transfer_id: &vector&lt;u8&gt;)
<b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> escrow_address = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(escrow);
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



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_unlocked">delete_unlocked</a>(escrow: &signer, transfer_id: &vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_unlocked">delete_unlocked</a>(escrow: &signer, transfer_id: &vector&lt;u8&gt;)
<b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a> {
    <b>let</b> escrow_address = <a href="../../../../../../move-stdlib/docs/Signer.md#0x1_Signer_address_of">Signer::address_of</a>(escrow);
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
    <b>let</b> info = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&state.locked, index);
    *info
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_unlocked_at"></a>

## Function `get_unlocked_at`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_unlocked_at">get_unlocked_at</a>(escrow_address: address, index: u64): <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_unlocked_at">get_unlocked_at</a>(escrow_address: address, index: u64): <a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a> <b>acquires</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>  {
    <b>assert</b>(<a href="BridgeEscrow.md#0x1_BridgeEscrow_get_unlocked_length">get_unlocked_length</a>(escrow_address) &gt; index, <a href="BridgeEscrow.md#0x1_BridgeEscrow_ERROR_LOCKED_EMPTY">ERROR_LOCKED_EMPTY</a>);
    <b>let</b> state = borrow_global&lt;<a href="BridgeEscrow.md#0x1_BridgeEscrow_EscrowState">EscrowState</a>&gt;(escrow_address);
    <b>let</b> info = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_borrow">Vector::borrow</a>(&state.unlocked, index);
    *info
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
    state.balance
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

<a name="0x1_BridgeEscrow_get_sender_from_ai"></a>

## Function `get_sender_from_ai`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_from_ai">get_sender_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): address
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_sender_from_ai">get_sender_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): address {
    *&ai.sender
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_receiver_from_ai"></a>

## Function `get_receiver_from_ai`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_from_ai">get_receiver_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): address
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_receiver_from_ai">get_receiver_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): address {
    *&ai.receiver
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_balance_from_ai"></a>

## Function `get_balance_from_ai`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_balance_from_ai">get_balance_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_balance_from_ai">get_balance_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): u64 {
    *&ai.balance
}
</code></pre>



</details>

<a name="0x1_BridgeEscrow_get_transfer_id_from_ai"></a>

## Function `get_transfer_id_from_ai`



<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_transfer_id_from_ai">get_transfer_id_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">BridgeEscrow::AccountInfo</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow_get_transfer_id_from_ai">get_transfer_id_from_ai</a>(ai: &<a href="BridgeEscrow.md#0x1_BridgeEscrow_AccountInfo">AccountInfo</a>): vector&lt;u8&gt; {
    *&ai.transfer_id
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/diem/dip/blob/main/dips/dip-2.md
[ROLE]: https://github.com/diem/dip/blob/main/dips/dip-2.md#roles
[PERMISSION]: https://github.com/diem/dip/blob/main/dips/dip-2.md#permissions
