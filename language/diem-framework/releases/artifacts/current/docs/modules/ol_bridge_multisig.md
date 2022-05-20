
<a name="0x1_BridgeMultisigScripts"></a>

# Module `0x1::BridgeMultisigScripts`



-  [Constants](#@Constants_0)
-  [Function `bridge_multisig_create_escrow`](#0x1_BridgeMultisigScripts_bridge_multisig_create_escrow)
-  [Function `bridge_multisig_deposit`](#0x1_BridgeMultisigScripts_bridge_multisig_deposit)
-  [Function `bridge_multisig_deposit_funds`](#0x1_BridgeMultisigScripts_bridge_multisig_deposit_funds)
-  [Function `bridge_multisig_withdraw`](#0x1_BridgeMultisigScripts_bridge_multisig_withdraw)
-  [Function `bridge_multisig_close_transfer`](#0x1_BridgeMultisigScripts_bridge_multisig_close_transfer)


<pre><code><b>use</b> <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig">0x1::BridgeEscrowMultisig</a>;
<b>use</b> <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x1_BridgeMultisigScripts_ZERO_ADDRESS"></a>



<pre><code><b>const</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>: address = 0;
</code></pre>



<a name="0x1_BridgeMultisigScripts_bridge_multisig_create_escrow"></a>

## Function `bridge_multisig_create_escrow`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_create_escrow">bridge_multisig_create_escrow</a>(sender: signer, executor1: address, executor2: address, executor3: address, executor4: address, executor5: address, min_votes: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_create_escrow">bridge_multisig_create_escrow</a>(
    sender: signer,
    executor1: address,
    executor2: address,
    executor3: address,
    executor4: address,
    executor5: address,
    min_votes:u64
) {
    <b>let</b> executors = <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_empty">Vector::empty</a>&lt;address&gt;();
    <b>if</b> (executor1 != <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>) {
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> executors, executor1)
    };
    <b>if</b> (executor2 != <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>) {
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> executors, executor2)
    };
    <b>if</b> (executor3 != <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>) {
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> executors, executor3)
    };
    <b>if</b> (executor4 != <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>) {
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> executors, executor4)
    };
    <b>if</b> (executor5 != <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_ZERO_ADDRESS">ZERO_ADDRESS</a>) {
        <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_push_back">Vector::push_back</a>(&<b>mut</b> executors, executor5)
    };
    <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_initialize_escrow">BridgeEscrowMultisig::initialize_escrow</a>(&sender, executors, min_votes);
}
</code></pre>



</details>

<a name="0x1_BridgeMultisigScripts_bridge_multisig_deposit"></a>

## Function `bridge_multisig_deposit`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_deposit">bridge_multisig_deposit</a>(sender: signer, escrow: address, receiver_other: vector&lt;u8&gt;, value: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_deposit">bridge_multisig_deposit</a>(
    sender: signer,
    escrow: address,
    receiver_other: vector&lt;u8&gt;,
    value: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_create_transfer_account">BridgeEscrowMultisig::create_transfer_account</a>(escrow, &sender, receiver_other, value, transfer_id);
}
</code></pre>



</details>

<a name="0x1_BridgeMultisigScripts_bridge_multisig_deposit_funds"></a>

## Function `bridge_multisig_deposit_funds`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_deposit_funds">bridge_multisig_deposit_funds</a>(sender: signer, escrow: address, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_deposit_funds">bridge_multisig_deposit_funds</a>(
    sender: signer,
    escrow: address,
    value: u64,
) {
    <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_deposit_funds">BridgeEscrowMultisig::deposit_funds</a>(escrow, &sender, value);
}
</code></pre>



</details>

<a name="0x1_BridgeMultisigScripts_bridge_multisig_withdraw"></a>

## Function `bridge_multisig_withdraw`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_withdraw">bridge_multisig_withdraw</a>(sender: signer, escrow: address, sender_other: vector&lt;u8&gt;, receiver: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_withdraw">bridge_multisig_withdraw</a>(
    sender: signer,
    escrow: address,
    sender_other: vector&lt;u8&gt;,
    receiver: address,
    balance: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_withdraw_from_escrow">BridgeEscrowMultisig::withdraw_from_escrow</a>(&sender, escrow,
        sender_other,
        receiver, // receiver
        balance, // balance
        transfer_id, // transfer_id
    );
}
</code></pre>



</details>

<a name="0x1_BridgeMultisigScripts_bridge_multisig_close_transfer"></a>

## Function `bridge_multisig_close_transfer`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_close_transfer">bridge_multisig_close_transfer</a>(sender: signer, escrow: address, transfer_id: vector&lt;u8&gt;, close_other: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge_multisig.md#0x1_BridgeMultisigScripts_bridge_multisig_close_transfer">bridge_multisig_close_transfer</a>(
    sender: signer,
    escrow: address,
    transfer_id: vector&lt;u8&gt;,
    close_other: bool,
) {
    <b>if</b> (!close_other) {
        <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_delete_transfer_account">BridgeEscrowMultisig::delete_transfer_account</a>( & sender, escrow, &transfer_id);
    } <b>else</b> {
        <a href="BridgeEscrowMultisig.md#0x1_BridgeEscrowMultisig_delete_unlocked">BridgeEscrowMultisig::delete_unlocked</a>( & sender, escrow, &transfer_id);
    }
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/diem/dip/blob/main/dips/dip-2.md
[ROLE]: https://github.com/diem/dip/blob/main/dips/dip-2.md#roles
[PERMISSION]: https://github.com/diem/dip/blob/main/dips/dip-2.md#permissions
