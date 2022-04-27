
<a name="0x1_BridgeScripts"></a>

# Module `0x1::BridgeScripts`



-  [Function `bridge_create_escrow`](#0x1_BridgeScripts_bridge_create_escrow)
-  [Function `bridge_deposit`](#0x1_BridgeScripts_bridge_deposit)
-  [Function `bridge_deposit_funds`](#0x1_BridgeScripts_bridge_deposit_funds)
-  [Function `bridge_withdraw`](#0x1_BridgeScripts_bridge_withdraw)
-  [Function `bridge_withdraw_funds`](#0x1_BridgeScripts_bridge_withdraw_funds)
-  [Function `bridge_close_transfer`](#0x1_BridgeScripts_bridge_close_transfer)


<pre><code><b>use</b> <a href="BridgeEscrow.md#0x1_BridgeEscrow">0x1::BridgeEscrow</a>;
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



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit">bridge_deposit</a>(sender: signer, escrow: address, receiver_other: vector&lt;u8&gt;, value: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit">bridge_deposit</a>(
    sender: signer,
    escrow: address,
    receiver_other: vector&lt;u8&gt;,
    value: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_create_transfer_account">BridgeEscrow::create_transfer_account</a>(escrow, &sender, receiver_other, value, transfer_id);
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_deposit_funds"></a>

## Function `bridge_deposit_funds`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit_funds">bridge_deposit_funds</a>(sender: signer, escrow: address, value: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_deposit_funds">bridge_deposit_funds</a>(
    sender: signer,
    escrow: address,
    value: u64,
) {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_deposit_funds">BridgeEscrow::deposit_funds</a>(escrow, &sender, value);
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_withdraw"></a>

## Function `bridge_withdraw`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw">bridge_withdraw</a>(sender: signer, escrow: address, sender_other: vector&lt;u8&gt;, receiver: address, balance: u64, transfer_id: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw">bridge_withdraw</a>(
    sender: signer,
    escrow: address,
    sender_other: vector&lt;u8&gt;,
    receiver: address,
    balance: u64,
    transfer_id: vector&lt;u8&gt;,
) {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_from_escrow">BridgeEscrow::withdraw_from_escrow</a>(&sender, escrow,
        sender_other,
        receiver, // receiver
        balance, // balance
        transfer_id, // transfer_id
    );
}
</code></pre>



</details>

<a name="0x1_BridgeScripts_bridge_withdraw_funds"></a>

## Function `bridge_withdraw_funds`



<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw_funds">bridge_withdraw_funds</a>(sender: signer, escrow: address, receiver: address, balance: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>script</b>) <b>fun</b> <a href="ol_bridge.md#0x1_BridgeScripts_bridge_withdraw_funds">bridge_withdraw_funds</a>(
    sender: signer,
    escrow: address,
    receiver: address,
    balance: u64,
) {
    <a href="BridgeEscrow.md#0x1_BridgeEscrow_withdraw_funds">BridgeEscrow::withdraw_funds</a>(&sender, escrow,
        receiver, // receiver
        balance, // balance
    );
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
