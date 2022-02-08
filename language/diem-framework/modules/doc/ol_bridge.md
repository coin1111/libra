
<a name="0x1_BridgeScripts"></a>

# Module `0x1::BridgeScripts`



-  [Function `bridge_create_escrow`](#0x1_BridgeScripts_bridge_create_escrow)
-  [Function `bridge_deposit`](#0x1_BridgeScripts_bridge_deposit)
-  [Function `bridge_withdraw`](#0x1_BridgeScripts_bridge_withdraw)
-  [Function `bridge_close_transfer`](#0x1_BridgeScripts_bridge_close_transfer)


<pre><code><b>use</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow">0x1::BridgeEscrow</a>;
<b>use</b> <a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector">0x1::Vector</a>;
</code></pre>



<a name="0x1_BridgeScripts_bridge_create_escrow"></a>

## Function `bridge_create_escrow`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_create_escrow">bridge_create_escrow</a>(sender: signer)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_create_escrow">bridge_create_escrow</a>(
    sender: signer,
) {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_initialize_escrow">BridgeEscrow::initialize_escrow</a>(&sender);
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_deposit"></a>

## Function `bridge_deposit`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit">bridge_deposit</a>(sender: signer, escrow: address, receiver: address, receiver_other: vector&lt;u8&gt;, value: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit">bridge_deposit</a>(
    sender: signer,
    escrow: address,
    receiver: address,
    receiver_other: vector&lt;u8&gt;,
    value: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <b>if</b> (<a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&receiver_other) == 0) {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_this">BridgeEscrow::create_transfer_account_this</a>(escrow, &sender, receiver, value, transfer_id);
    } <b>else</b> {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account_other">BridgeEscrow::create_transfer_account_other</a>(escrow, &sender, receiver_other, value, transfer_id);
    }
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_withdraw"></a>

## Function `bridge_withdraw`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw">bridge_withdraw</a>(sender: signer, escrow: address, sender_this: address, sender_other: vector&lt;u8&gt;, receiver: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw">bridge_withdraw</a>(
    sender: signer,
    escrow: address,
    sender_this: address,
    sender_other: vector&lt;u8&gt;,
    receiver: address,
    balance: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <b>if</b> (<a href="../../../../../../move-stdlib/docs/Vector.md#0x1_Vector_length">Vector::length</a>(&sender_other) == 0) {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_this">BridgeEscrow::withdraw_from_escrow_this</a>(&sender, escrow,
            sender_this,
            receiver, // receiver
            balance, // balance
            transfer_id, // transfer_id
        );
    } <b>else</b> {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow_other">BridgeEscrow::withdraw_from_escrow_other</a>(&sender, escrow,
            sender_other,
            receiver, // receiver
            balance, // balance
            transfer_id, // transfer_id
        );
    }
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_close_transfer"></a>

## Function `bridge_close_transfer`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_close_transfer">bridge_close_transfer</a>(sender: signer, escrow: address, transfer_id: vector&lt;u8&gt;, close_other: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_close_transfer">bridge_close_transfer</a>(
    sender: signer,
    escrow: address,
    transfer_id: vector&lt;u8&gt;,
    close_other: bool,
) {
    <b>if</b> (!close_other) {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_transfer_account">BridgeEscrow::delete_transfer_account</a>( & sender, escrow, &transfer_id);
    } <b>else</b> {
        <a href="BridgeEscrow.md#0x1_BridgeEscrow_delete_unlocked">BridgeEscrow::delete_unlocked</a>( & sender, escrow, &transfer_id);
    }
}
</code></pre>



</details>


[//]: # ("File containing references which can be used from documentation")
[ACCESS_CONTROL]: https://github.com/diem/dip/blob/main/dips/dip-2.md
[ROLE]: https://github.com/diem/dip/blob/main/dips/dip-2.md#roles
[PERMISSION]: https://github.com/diem/dip/blob/main/dips/dip-2.md#permissions
