mod utils;

use bech32::{Bech32, ToBase32 as _};
use chain::{account, certificate, fee, key, transaction as tx, txbuilder, value};
use chain_core::property::Block as _;
use chain_core::property::Deserialize as _;
use chain_core::property::HasFragments as _;
use chain_core::property::Serialize;
use chain_crypto as crypto;
use chain_impl_mockchain as chain;
use crypto::bech32::Bech32 as _;
use js_sys::Uint8Array;
use rand_os::OsRng;
use std::convert::TryFrom;
use std::ops::{Add, Sub};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

/// ED25519 signing key, either normal or extended
#[wasm_bindgen]
pub struct PrivateKey(key::EitherEd25519SecretKey);

impl From<key::EitherEd25519SecretKey> for PrivateKey {
    fn from(secret_key: key::EitherEd25519SecretKey) -> PrivateKey {
        PrivateKey(secret_key)
    }
}

#[wasm_bindgen]
impl PrivateKey {
    /// Get private key from its bech32 representation
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519_sk1ahfetf02qwwg4dkq7mgp4a25lx5vh9920cr5wnxmpzz9906qvm8qwvlts0&#39;);
    /// ```
    /// For an extended 25519 key
    /// ```javascript
    /// PrivateKey.from_bech32(&#39;ed25519e_sk1gqwl4szuwwh6d0yk3nsqcc6xxc3fpvjlevgwvt60df59v8zd8f8prazt8ln3lmz096ux3xvhhvm3ca9wj2yctdh3pnw0szrma07rt5gl748fp&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PrivateKey, JsValue> {
        crypto::SecretKey::try_from_bech32_str(&bech32_str)
            .map(key::EitherEd25519SecretKey::Extended)
            .or_else(|_| {
                crypto::SecretKey::try_from_bech32_str(&bech32_str)
                    .map(key::EitherEd25519SecretKey::Normal)
            })
            .map(PrivateKey)
            .map_err(|_| JsValue::from_str("Invalid secret key"))
    }

    pub fn to_public(&self) -> PublicKey {
        self.0.to_public().into()
    }

    pub fn generate_ed25519() -> Result<PrivateKey, JsValue> {
        OsRng::new()
            .map(crypto::SecretKey::<crypto::Ed25519>::generate)
            .map(key::EitherEd25519SecretKey::Normal)
            .map(PrivateKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn generate_ed25519extended() -> Result<PrivateKey, JsValue> {
        OsRng::new()
            .map(crypto::SecretKey::<crypto::Ed25519Extended>::generate)
            .map(key::EitherEd25519SecretKey::Extended)
            .map(PrivateKey)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn to_bech32(&self) -> String {
        match self.0 {
            key::EitherEd25519SecretKey::Normal(ref secret) => secret.to_bech32_str(),
            key::EitherEd25519SecretKey::Extended(ref secret) => secret.to_bech32_str(),
        }
    }
}

/// ED25519 key used as public key
#[wasm_bindgen]
#[derive(Clone)]
pub struct PublicKey(crypto::PublicKey<crypto::Ed25519>);

impl From<crypto::PublicKey<crypto::Ed25519>> for PublicKey {
    fn from(key: crypto::PublicKey<crypto::Ed25519>) -> PublicKey {
        PublicKey(key)
    }
}

#[wasm_bindgen]
impl PublicKey {
    /// Get private key from its bech32 representation
    /// Example:
    /// ```javascript
    /// const pkey = PublicKey.from_bech32(&#39;ed25519_pk1dgaagyh470y66p899txcl3r0jaeaxu6yd7z2dxyk55qcycdml8gszkxze2&#39;);
    /// ```
    pub fn from_bech32(bech32_str: &str) -> Result<PublicKey, JsValue> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(PublicKey)
            .map_err(|_| JsValue::from_str("Malformed public key"))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }
}

#[wasm_bindgen]
pub struct PublicKeys(Vec<PublicKey>);

#[wasm_bindgen]
impl PublicKeys {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PublicKeys {
        PublicKeys(vec![])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> PublicKey {
        self.0[index].clone()
    }

    pub fn add(&mut self, key: PublicKey) {
        self.0.push(key);
    }
}

//-----------------------------//
//----------Address------------//
//-----------------------------//

/// An address of any type, this can be one of
/// * A utxo-based address without delegation (single)
/// * A utxo-based address with delegation (group)
/// * An address for an account
#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(chain_addr::Address);

#[wasm_bindgen]
impl Address {
    //XXX: Maybe this should be from_bech32?
    /// Construct Address from its bech32 representation
    /// Example
    /// ```javascript
    /// const address = Address.from_string(&#39;ca1q09u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pjyknplu&#39;);
    /// ```
    pub fn from_string(s: &str) -> Result<Address, JsValue> {
        chain_addr::AddressReadable::from_string_anyprefix(s)
            .map(|address_readable| Address(address_readable.to_address()))
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
    }

    /// Get Address bech32 (string) representation with a given prefix
    /// ```javascript
    /// let public_key = PublicKey.from_bech32(
    ///     &#39;ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8&#39;
    /// );
    /// let discriminant = AddressDiscrimination.Test;
    /// let address = Address.single_from_public_key(public_key, discriminant);
    /// address.to_string(&#39;ta&#39;)
    /// // ta1sj6gu33yw73dr60f2ehp6xemgf30r49rzc25gkrfnrfuuyf0mycgnj78ende550w5njvwzyr20q6rypdea597uu3jnwfltljddl59cseaq7yn9
    /// ```
    pub fn to_string(&self, prefix: &str) -> String {
        format!(
            "{}",
            chain_addr::AddressReadable::from_address(prefix, &self.0)
        )
    }

    /// Construct a single non-account address from a public key
    /// ```javascript
    /// let public_key = PublicKey.from_bech32(
    ///     &#39;ed25519_pk1kj8yvfrh5tg7n62kdcw3kw6zvtcafgckz4z9s6vc608pzt7exzys4s9gs8&#39;
    /// );
    /// let address = Address.single_from_public_key(public_key, AddressDiscrimination.Test);
    /// ```
    pub fn single_from_public_key(
        key: PublicKey,
        discrimination: AddressDiscrimination,
    ) -> Address {
        chain_addr::Address(discrimination.into(), chain_addr::Kind::Single(key.0)).into()
    }

    /// Construct a non-account address from a pair of public keys, delegating founds from the first to the second
    pub fn delegation_from_public_key(
        key: PublicKey,
        delegation: PublicKey,
        discrimination: AddressDiscrimination,
    ) -> Address {
        chain_addr::Address(
            discrimination.into(),
            chain_addr::Kind::Group(key.0, delegation.0),
        )
        .into()
    }

    /// Construct address of account type from a public key
    pub fn account_from_public_key(
        key: PublicKey,
        discrimination: AddressDiscrimination,
    ) -> Address {
        chain_addr::Address(discrimination.into(), chain_addr::Kind::Account(key.0)).into()
    }
}

impl From<chain_addr::Address> for Address {
    fn from(address: chain_addr::Address) -> Address {
        Address(address)
    }
}

/// Allow to differentiate between address in
/// production and testing setting, so that
/// one type of address is not used in another setting.
/// Example
/// ```javascript
/// let discriminant = AddressDiscrimination.Test;
/// let address = Address::single_from_public_key(public_key, discriminant);
/// ```
#[wasm_bindgen]
pub enum AddressDiscrimination {
    Production,
    Test,
}

impl Into<chain_addr::Discrimination> for AddressDiscrimination {
    fn into(self) -> chain_addr::Discrimination {
        match self {
            AddressDiscrimination::Production => chain_addr::Discrimination::Production,
            AddressDiscrimination::Test => chain_addr::Discrimination::Test,
        }
    }
}

//-----------------------------------//
//-------- Transaction --------------//
//-----------------------------------//

/// Type representing a unsigned transaction
#[wasm_bindgen]
pub struct Transaction(EitherTransaction);

enum EitherTransaction {
    TransactionWithoutCertificate(tx::Transaction<chain_addr::Address, tx::NoExtra>),
    TransactionWithCertificate(tx::Transaction<chain_addr::Address, certificate::Certificate>),
}

impl EitherTransaction {
    fn id(&self) -> TransactionSignDataHash {
        match &self {
            EitherTransaction::TransactionWithoutCertificate(tx) => tx.hash(),
            EitherTransaction::TransactionWithCertificate(tx) => tx.hash(),
        }
        .into()
    }

    fn inputs(&self) -> Vec<tx::Input> {
        match &self {
            EitherTransaction::TransactionWithoutCertificate(tx) => tx.inputs.clone(),
            EitherTransaction::TransactionWithCertificate(tx) => tx.inputs.clone(),
        }
        .to_vec()
    }

    fn outputs(&self) -> Vec<tx::Output<chain_addr::Address>> {
        match &self {
            EitherTransaction::TransactionWithoutCertificate(ref tx) => tx.outputs.clone(),
            EitherTransaction::TransactionWithCertificate(ref tx) => tx.outputs.clone(),
        }
        .to_vec()
    }
}

impl From<tx::Transaction<chain_addr::Address, tx::NoExtra>> for Transaction {
    fn from(tx: tx::Transaction<chain_addr::Address, tx::NoExtra>) -> Self {
        Transaction(EitherTransaction::TransactionWithoutCertificate(tx))
    }
}

impl From<tx::Transaction<chain_addr::Address, certificate::Certificate>> for Transaction {
    fn from(tx: tx::Transaction<chain_addr::Address, certificate::Certificate>) -> Self {
        Transaction(EitherTransaction::TransactionWithCertificate(tx))
    }
}

macro_rules! impl_collection {
    ($collection:ident, $type:ty) => {
        #[wasm_bindgen]
        pub struct $collection(Vec<$type>);

        #[wasm_bindgen]
        impl $collection {
            pub fn size(&self) -> usize {
                self.0.len()
            }

            pub fn get(&self, index: usize) -> $type {
                self.0[index].clone()
            }
        }

        impl From<Vec<$type>> for $collection {
            fn from(vec: Vec<$type>) -> $collection {
                $collection(vec)
            }
        }
    };
}

impl_collection!(Outputs, Output);
impl_collection!(Inputs, Input);
impl_collection!(Fragments, Fragment);

#[wasm_bindgen]
impl Transaction {
    /// Get the transaction id, needed to compute its signature
    pub fn id(&self) -> TransactionSignDataHash {
        self.0.id()
    }

    /// Get collection of the inputs in the transaction (this allocates new copies of all the values)
    pub fn inputs(&self) -> Inputs {
        self.0
            .inputs()
            .iter()
            .map(|input| Input(input.clone()))
            .collect::<Vec<Input>>()
            .into()
    }

    /// Get collection of the outputs in the transaction (this allocates new copies of all the values)
    pub fn outputs(&self) -> Outputs {
        self.0
            .outputs()
            .iter()
            .map(|output| Output(output.clone()))
            .collect::<Vec<Output>>()
            .into()
    }
}

//-----------------------------------//
//--------TransactionBuilder---------//
//-----------------------------------//

/// Builder pattern implementation for making a Transaction
///
/// Example
///
/// ```javascript
/// const txbuilder = new TransactionBuilder();
///
/// const account = Account.from_address(Address.from_string(
///   &#39;ca1qh9u0nxmnfg7af8ycuygx57p5xgzmnmgtaeer9xun7hly6mlgt3pj2xk344&#39;
/// ));
///
/// const input = Input.from_account(account, Value.from_str('1000'));
///
/// txbuilder.add_input(input);
///
/// txbuilder.add_output(
///   Address.from_string(
///     &#39;ca1q5nr5pvt9e5p009strshxndrsx5etcentslp2rwj6csm8sfk24a2w3swacn&#39;
///   ),
///   Value.from_str('500')
/// );
///
/// const feeAlgorithm = Fee.linear_fee(
///   Value.from_str('20'),
///   Value.from_str('5'),
///   Value.from_str('0')
/// );
///
/// const finalizedTx = txbuilder.finalize(
///   feeAlgorithm,
///   OutputPolicy.one(accountInputAddress)
/// );
/// ```
#[wasm_bindgen]
pub struct TransactionBuilder(EitherTransactionBuilder);

enum EitherTransactionBuilder {
    TransactionBuilderNoExtra(txbuilder::TransactionBuilder<chain_addr::Address, tx::NoExtra>),
    TransactionBuilderCertificate(
        txbuilder::TransactionBuilder<chain_addr::Address, certificate::Certificate>,
    ),
}

impl From<txbuilder::TransactionBuilder<chain_addr::Address, tx::NoExtra>> for TransactionBuilder {
    fn from(builder: txbuilder::TransactionBuilder<chain_addr::Address, tx::NoExtra>) -> Self {
        TransactionBuilder(EitherTransactionBuilder::TransactionBuilderNoExtra(builder))
    }
}

impl From<txbuilder::TransactionBuilder<chain_addr::Address, certificate::Certificate>>
    for TransactionBuilder
{
    fn from(
        builder: txbuilder::TransactionBuilder<chain_addr::Address, certificate::Certificate>,
    ) -> Self {
        TransactionBuilder(EitherTransactionBuilder::TransactionBuilderCertificate(
            builder,
        ))
    }
}

#[wasm_bindgen]
impl TransactionBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        txbuilder::TransactionBuilder::new().into()
    }

    /// Add certificate to the transaction if there isn't one already
    /// Example
    /// ```javascript
    /// const certificate = Certificate.stake_delegation(
    ///     StakePoolId.from_hex(poolId),
    ///     PublicKey.from_bech32(stakeKey)
    /// );
    ///
    /// certificate.sign(PrivateKey.from_bech32(privateKey));
    ///
    /// const txbuilder = new TransactionBuilder();
    /// txbuilder.set_certificate(certificate);
    /// ```
    #[wasm_bindgen]
    pub fn set_certificate(&mut self, certificate: Certificate) -> Result<(), JsValue> {
        let builder = match &self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref builder) => {
                builder.clone().set_certificate(certificate.0)
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(_) =>
            //Is either this or replacing the extra
            {
                return Err(JsValue::from_str("There is already one certificate"))
            }
        };
        self.0 = EitherTransactionBuilder::TransactionBuilderCertificate(builder);
        Ok(())
    }

    /// Add input to the transaction
    #[wasm_bindgen]
    pub fn add_input(&mut self, input: Input) {
        match &mut self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref mut builder) => {
                builder.add_input(&input.0)
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(ref mut builder) => {
                builder.add_input(&input.0)
            }
        }
    }

    /// Add output to the transaction
    #[wasm_bindgen]
    pub fn add_output(&mut self, address: Address, value: Value) {
        match &mut self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref mut builder) => {
                builder.add_output(address.0, value.0)
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(ref mut builder) => {
                builder.add_output(address.0, value.0)
            }
        }
    }

    /// Estimate fee with the currently added inputs, outputs and certificate based on the given algorithm
    #[wasm_bindgen]
    pub fn estimate_fee(&self, fee: &Fee) -> Result<Value, JsValue> {
        let fee_algorithm = match fee.0 {
            FeeVariant::Linear(fee_algorithm) => fee_algorithm,
        };
        match &self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref builder) => {
                builder.estimate_fee(fee_algorithm)
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(ref builder) => {
                builder.estimate_fee(fee_algorithm)
            }
        }
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
        .map(|value| value.into())
    }

    #[wasm_bindgen]
    pub fn get_balance(&self, fee: &Fee) -> Result<Balance, JsValue> {
        let fee_algorithm = match fee.0 {
            FeeVariant::Linear(fee_algorithm) => fee_algorithm,
        };
        match &self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref builder) => {
                builder.get_balance(fee_algorithm)
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(ref builder) => {
                builder.get_balance(fee_algorithm)
            }
        }
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
        .map(|balance| balance.into())
    }

    #[wasm_bindgen]
    pub fn get_balance_without_fee(&self) -> Result<Balance, JsValue> {
        match &self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(ref builder) => {
                builder.get_balance_without_fee()
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(ref builder) => {
                builder.get_balance_without_fee()
            }
        }
        .map(|balance| balance.into())
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Get the Transaction with the current inputs and outputs without computing the fees nor adding a change address
    #[wasm_bindgen]
    pub fn unchecked_finalize(self) -> Transaction {
        match self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(builder) => builder.tx.into(),
            EitherTransactionBuilder::TransactionBuilderCertificate(builder) => builder.tx.into(),
        }
    }

    /// Finalize the transaction by adding the change Address output
    /// leaving enough for paying the minimum fee computed by the given algorithm
    /// see the unchecked_finalize for the non-assisted version
    ///
    /// Example
    /// 
    /// ```javascript
    /// const feeAlgorithm = Fee.linear_fee(
    ///     Value.from_str('20'), Value.from_str('5'), Value.from_str('10')
    /// );
    ///
    /// const finalizedTx = txbuilder.finalize(
    ///   feeAlgorithm,
    ///   OutputPolicy.one(changeAddress)
    /// );
    /// ```
    #[wasm_bindgen]
    pub fn finalize(self, fee: &Fee, output_policy: OutputPolicy) -> Result<Transaction, JsValue> {
        let fee_algorithm = match fee.0 {
            FeeVariant::Linear(fee_algorithm) => fee_algorithm,
        };

        match self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(builder) => builder
                .finalize(fee_algorithm, output_policy.0)
                .map(|(_, tx)| tx.into()),
            EitherTransactionBuilder::TransactionBuilderCertificate(builder) => builder
                .finalize(fee_algorithm, output_policy.0)
                .map(|(_, tx)| tx.into()),
        }
        .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Get the current Transaction id, this will change when adding input, outputs and certificates
    #[wasm_bindgen]
    pub fn get_txid(&self) -> TransactionSignDataHash {
        match &self.0 {
            EitherTransactionBuilder::TransactionBuilderNoExtra(builder) => {
                builder.tx.hash().into()
            }
            EitherTransactionBuilder::TransactionBuilderCertificate(builder) => {
                builder.tx.hash().into()
            }
        }
    }
}

/// Helper to add change addresses when finalizing a transaction, there are currently two options
/// * forget: use all the excess money as fee
/// * one: send all the excess money to the given address
#[wasm_bindgen]
pub struct OutputPolicy(txbuilder::OutputPolicy);

impl From<txbuilder::OutputPolicy> for OutputPolicy {
    fn from(output_policy: txbuilder::OutputPolicy) -> OutputPolicy {
        OutputPolicy(output_policy)
    }
}

#[wasm_bindgen]
impl OutputPolicy {
    /// don't do anything with the excess money in transaction
    pub fn forget() -> OutputPolicy {
        txbuilder::OutputPolicy::Forget.into()
    }

    /// use the given address as the only change address
    pub fn one(address: Address) -> OutputPolicy {
        txbuilder::OutputPolicy::One(address.0).into()
    }
}

/// Builder pattern implementation for signing a Transaction (adding witnesses)
/// Example (for an account as input)
///
/// ```javascript
/// //finalizedTx could be the result of the finalize method on a TransactionBuilder object
/// const finalizer = new TransactionFinalizer(finalizedTx);
///
/// const witness = Witness.for_account(
///   Hash.from_hex(genesisHashString),
///   finalizer.get_txid(),
///   inputAccountPrivateKey,
///   SpendingCounter.zero()
/// );
///
/// finalizer.set_witness(0, witness);
///
/// const signedTx = finalizer.build();
/// ```
#[wasm_bindgen]
pub struct TransactionFinalizer(txbuilder::TransactionFinalizer);

impl From<txbuilder::TransactionFinalizer> for TransactionFinalizer {
    fn from(finalizer: txbuilder::TransactionFinalizer) -> TransactionFinalizer {
        TransactionFinalizer(finalizer)
    }
}

#[wasm_bindgen]
impl TransactionFinalizer {
    #[wasm_bindgen(constructor)]
    pub fn new(transaction: Transaction) -> Self {
        TransactionFinalizer(match transaction.0 {
            EitherTransaction::TransactionWithCertificate(tx) => {
                txbuilder::TransactionFinalizer::new_cert(tx)
            }
            EitherTransaction::TransactionWithoutCertificate(tx) => {
                txbuilder::TransactionFinalizer::new_trans(tx)
            }
        })
    }

    /// Set the witness for the corresponding index, the index corresponds to the order in which the inputs were added to the transaction
    pub fn set_witness(&mut self, index: usize, witness: Witness) -> Result<(), JsValue> {
        self.0
            .set_witness(index, witness.0)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    pub fn get_txid(&self) -> TransactionSignDataHash {
        self.0.get_txid().into()
    }

    pub fn build(self) -> Result<GeneratedTransaction, JsValue> {
        self.0
            .build()
            .map(GeneratedTransaction)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
    }
}

/// Type for representing a Transaction with Witnesses (signatures)
#[wasm_bindgen]
pub struct GeneratedTransaction(txbuilder::GeneratedTransaction);

impl From<txbuilder::GeneratedTransaction> for GeneratedTransaction {
    fn from(generated_transaction: txbuilder::GeneratedTransaction) -> GeneratedTransaction {
        GeneratedTransaction(generated_transaction)
    }
}

#[wasm_bindgen]
impl GeneratedTransaction {
    pub fn id(&self) -> TransactionSignDataHash {
        match &self.0 {
            chain::txbuilder::GeneratedTransaction::Type1(auth) => auth.transaction.hash(),
            chain::txbuilder::GeneratedTransaction::Type2(auth) => auth.transaction.hash(),
        }
        .into()
    }

    /// Get a copy of the inner Transaction, discarding the signatures
    pub fn transaction(&self) -> Transaction {
        match &self.0 {
            chain::txbuilder::GeneratedTransaction::Type1(auth) => auth.transaction.clone().into(),
            chain::txbuilder::GeneratedTransaction::Type2(auth) => auth.transaction.clone().into(),
        }
    }
}

/// Type for representing the hash of a Transaction, necessary for signing it
#[wasm_bindgen]
pub struct TransactionSignDataHash(tx::TransactionSignDataHash);

#[wasm_bindgen]
impl TransactionSignDataHash {
    pub fn from_bytes(bytes: &[u8]) -> Result<TransactionSignDataHash, JsValue> {
        tx::TransactionSignDataHash::try_from(bytes)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(|digest| digest.into())
    }

    pub fn from_hex(input: &str) -> Result<TransactionSignDataHash, JsValue> {
        tx::TransactionSignDataHash::from_str(input)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
            .map(TransactionSignDataHash)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.as_ref().to_vec()
    }
}

impl From<tx::TransactionSignDataHash> for TransactionSignDataHash {
    fn from(txid: tx::TransactionSignDataHash) -> TransactionSignDataHash {
        TransactionSignDataHash(txid)
    }
}

/// Type for representing a generic Hash
#[wasm_bindgen]
pub struct Hash(key::Hash);

impl From<key::Hash> for Hash {
    fn from(hash: key::Hash) -> Hash {
        Hash(hash)
    }
}

#[wasm_bindgen]
impl Hash {
    pub fn from_bytes(bytes: &[u8]) -> Hash {
        key::Hash::hash_bytes(bytes).into()
    }

    pub fn from_hex(hex_string: &str) -> Result<Hash, JsValue> {
        key::Hash::from_str(hex_string)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(Hash)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.serialize_as_vec().unwrap()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Input(tx::Input);

impl From<tx::Input> for Input {
    fn from(input: tx::Input) -> Input {
        Input(input)
    }
}

/// Generalized input which have a specific input value, and
/// either contains an account reference or a TransactionSignDataHash+index
///
/// This uniquely refer to a specific source of value.
#[wasm_bindgen]
impl Input {
    pub fn from_utxo(utxo_pointer: &UtxoPointer) -> Self {
        Input(tx::Input::from_utxo(utxo_pointer.0))
    }

    pub fn from_account(account: &Account, v: Value) -> Self {
        Input(tx::Input::from_account(account.0.clone(), v.0))
    }

    /// Get the kind of Input, this can be either "Account" or "Utxo"
    pub fn get_type(&self) -> String {
        match self.0.get_type() {
            tx::InputType::Account => "Account".to_string(),
            tx::InputType::Utxo => "Utxo".to_string(),
        }
    }

    pub fn value(&self) -> Value {
        self.0.value.into()
    }

    /// Get the inner UtxoPointer if the Input type is Utxo
    pub fn get_utxo_pointer(&self) -> Result<UtxoPointer, JsValue> {
        match self.0.to_enum() {
            tx::InputEnum::UtxoInput(utxo_pointer) => Ok(utxo_pointer.into()),
            _ => Err(JsValue::from_str("Input is not from utxo")),
        }
    }

    /// Get the source Account if the Input type is Account
    pub fn get_account(&self) -> Result<Account, JsValue> {
        match self.0.to_enum() {
            tx::InputEnum::AccountInput(account, _) => Ok(account.into()),
            _ => Err(JsValue::from_str("Input is not from account")),
        }
    }
}

/// Unspent transaction pointer. This is composed of:
/// * the transaction identifier where the unspent output is (a FragmentId)
/// * the output index within the pointed transaction's outputs
/// * the value we expect to read from this output, this setting is added in order to protect undesired withdrawal
/// and to set the actual fee in the transaction.
#[wasm_bindgen]
#[derive(Debug)]
pub struct UtxoPointer(tx::UtxoPointer);

impl From<tx::UtxoPointer> for UtxoPointer {
    fn from(ptr: tx::UtxoPointer) -> UtxoPointer {
        UtxoPointer(ptr)
    }
}

#[wasm_bindgen]
impl UtxoPointer {
    pub fn new(fragment_id: FragmentId, output_index: u8, value: Value) -> UtxoPointer {
        UtxoPointer(tx::UtxoPointer {
            transaction_id: fragment_id.0,
            output_index,
            value: value.0,
        })
    }
}

/// This is either an single account or a multisig account depending on the witness type
#[wasm_bindgen]
#[derive(Debug)]
pub struct Account(tx::AccountIdentifier);

impl From<tx::AccountIdentifier> for Account {
    fn from(account_identifier: tx::AccountIdentifier) -> Account {
        Account(account_identifier)
    }
}

#[wasm_bindgen]
impl Account {
    pub fn from_address(address: &Address) -> Result<Account, JsValue> {
        if let chain_addr::Kind::Account(key) = address.0.kind() {
            Ok(Account(tx::AccountIdentifier::from_single_account(
                key.clone().into(),
            )))
        } else {
            Err(JsValue::from_str("Address is not account"))
        }
    }

    pub fn to_address(&self) -> Address {
        let kind = match self.0.to_single_account() {
            Some(key) => chain_addr::Kind::Account(key.into()),
            None => panic!(),
        };
        let discriminant = chain_addr::Discrimination::Production;
        chain_addr::Address(discriminant, kind).into()
    }

    pub fn from_public_key(key: PublicKey) -> Account {
        Account(tx::AccountIdentifier::from_single_account(key.0.into()))
    }
}

/// Type for representing a Transaction Output, composed of an Address and a Value
#[wasm_bindgen]
#[derive(Clone)]
pub struct Output(tx::Output<chain_addr::Address>);

impl From<tx::Output<chain_addr::Address>> for Output {
    fn from(output: tx::Output<chain_addr::Address>) -> Output {
        Output(output)
    }
}

#[wasm_bindgen]
impl Output {
    pub fn address(&self) -> Address {
        self.0.address.clone().into()
    }

    pub fn value(&self) -> Value {
        self.0.value.into()
    }
}

/// Type used for representing certain amount of lovelaces.
/// It wraps an unsigned 64 bits number.
/// Strings are used for passing to and from javascript,
/// as the native javascript Number type can't hold the entire u64 range
/// and BigInt is not yet implemented in all the browsers
#[wasm_bindgen]
#[derive(Debug, Eq, PartialEq)]
pub struct Value(value::Value);

impl AsRef<u64> for Value {
    fn as_ref(&self) -> &u64 {
        &self.0.as_ref()
    }
}

impl From<u64> for Value {
    fn from(number: u64) -> Value {
        value::Value(number).into()
    }
}

#[wasm_bindgen]
impl Value {
    /// Parse the given string into a rust u64 numeric type.
    pub fn from_str(s: &str) -> Result<Value, JsValue> {
        s.parse::<u64>()
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(|number| number.into())
    }

    /// Return the wrapped u64 formatted as a string.
    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }

    pub fn checked_add(&self, other: &Value) -> Result<Value, JsValue> {
        self.0
            .add(other.0)
            .map_err(|e| JsValue::from_str(&format!("{}", &format!("{}", e))))
            .map(Value)
    }

    pub fn checked_sub(&self, other: &Value) -> Result<Value, JsValue> {
        self.0
            .sub(other.0)
            .map_err(|e| JsValue::from_str(&format!("{}", &format!("{}", e))))
            .map(Value)
    }
}

impl From<value::Value> for Value {
    fn from(value: value::Value) -> Value {
        Value(value)
    }
}

#[wasm_bindgen]
pub struct U128(u128);

impl From<u128> for U128 {
    fn from(number: u128) -> U128 {
        U128(number)
    }
}

#[wasm_bindgen]
impl U128 {
    pub fn from_be_bytes(bytes: Uint8Array) -> Result<U128, JsValue> {
        if bytes.length() == std::mem::size_of::<u128>() as u32 {
            let mut slice = [0u8; 16];
            bytes.copy_to(&mut slice);
            Ok(u128::from_be_bytes(slice).into())
        } else {
            Err(JsValue::from_str(&format!(
                "Invalid array length. Found {}, expected: 16",
                bytes.length()
            )))
        }
    }

    pub fn from_le_bytes(bytes: Uint8Array) -> Result<U128, JsValue> {
        if bytes.length() == std::mem::size_of::<u128>() as u32 {
            let mut slice = [0u8; 16];
            bytes.copy_to(&mut slice);
            Ok(u128::from_le_bytes(slice).into())
        } else {
            Err(JsValue::from_str(&format!(
                "Invalid array length. Found {}, expected: 16",
                bytes.length()
            )))
        }
    }

    pub fn from_str(s: &str) -> Result<U128, JsValue> {
        s.parse::<u128>()
            .map_err(|e| JsValue::from_str(&format! {"{:?}", e}))
            .map(U128)
    }

    pub fn to_str(&self) -> String {
        format!("{}", self.0)
    }
}

#[wasm_bindgen]
pub struct Certificate(certificate::Certificate);

#[wasm_bindgen]
impl Certificate {
    /// Create a stake delegation certificate from account (stake key) to pool_id
    pub fn stake_delegation(pool_id: StakePoolId, account: PublicKey) -> Certificate {
        let content = certificate::StakeDelegation {
            stake_key_id: tx::AccountIdentifier::from_single_account(account.0.into()),
            pool_id: pool_id.0,
        };
        certificate::Certificate {
            content: certificate::CertificateContent::StakeDelegation(content),
            signatures: vec![],
        }
        .into()
    }

    pub fn stake_pool_registration(pool_info: StakePoolInfo) -> Certificate {
        certificate::Certificate {
            content: certificate::CertificateContent::StakePoolRegistration(pool_info.0),
            signatures: vec![],
        }
        .into()
    }

    /// Add signature to certificate
    pub fn sign(&mut self, private_key: PrivateKey) {
        let signature = match &self.0.content {
            certificate::CertificateContent::StakeDelegation(s) => {
                s.make_certificate(&private_key.0)
            }
            certificate::CertificateContent::StakePoolRegistration(s) => {
                s.make_certificate(&private_key.0)
            }
            certificate::CertificateContent::StakePoolRetirement(s) => {
                s.make_certificate(&private_key.0)
            }
        };
        self.0.signatures.push(signature);
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.0
            .serialize_as_vec()
            .map_err(|error| JsValue::from_str(&format!("{}", error)))
    }

    pub fn to_bech32(&self) -> Result<String, JsValue> {
        Bech32::new("cert".to_string(), self.as_bytes()?.to_base32())
            .map(|bech32| bech32.to_string())
            .map_err(|error| JsValue::from_str(&format!("{}", error)))
    }
}

impl From<certificate::Certificate> for Certificate {
    fn from(certificate: certificate::Certificate) -> Certificate {
        Certificate(certificate)
    }
}

#[wasm_bindgen]
pub struct StakePoolInfo(chain::stake::StakePoolInfo);

impl From<chain::stake::StakePoolInfo> for StakePoolInfo {
    fn from(info: chain::stake::StakePoolInfo) -> StakePoolInfo {
        StakePoolInfo(info)
    }
}

#[wasm_bindgen]
impl StakePoolInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(
        serial: U128,
        owners: PublicKeys,
        kes_public_key: KesPublicKey,
        vrf_public_key: VrfPublicKey,
    ) -> StakePoolInfo {
        chain::stake::StakePoolInfo {
            serial: serial.0,
            owners: owners
                .0
                .into_iter()
                .map(|key| account::Identifier::from(key.0))
                .collect(),
            initial_key: chain::leadership::genesis::GenesisPraosLeader {
                kes_public_key: kes_public_key.0,
                vrf_public_key: vrf_public_key.0,
            },
        }
        .into()
    }

    pub fn id(&self) -> StakePoolId {
        self.0.to_id().into()
    }
}

#[wasm_bindgen]
pub struct StakePoolId(chain::stake::StakePoolId);

impl From<chain::stake::StakePoolId> for StakePoolId {
    fn from(pool_id: chain::stake::StakePoolId) -> StakePoolId {
        StakePoolId(pool_id)
    }
}

#[wasm_bindgen]
impl StakePoolId {
    pub fn from_hex(hex_string: &str) -> Result<StakePoolId, JsValue> {
        key::Hash::from_str(hex_string)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
            .map(|hash| StakePoolId(hash.into()))
    }

    pub fn to_string(&self) -> String {
        format!("{}", self.0).to_string()
    }
}

#[wasm_bindgen]
pub struct KesPublicKey(crypto::PublicKey<crypto::SumEd25519_12>);

impl From<crypto::PublicKey<crypto::SumEd25519_12>> for KesPublicKey {
    fn from(kes: crypto::PublicKey<crypto::SumEd25519_12>) -> KesPublicKey {
        KesPublicKey(kes)
    }
}

#[wasm_bindgen]
impl KesPublicKey {
    pub fn from_bech32(bech32_str: &str) -> Result<KesPublicKey, JsValue> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(KesPublicKey)
            .map_err(|_| JsValue::from_str("Malformed kes public key"))
    }
}

#[wasm_bindgen]
pub struct VrfPublicKey(crypto::PublicKey<crypto::Curve25519_2HashDH>);

impl From<crypto::PublicKey<crypto::Curve25519_2HashDH>> for VrfPublicKey {
    fn from(vrf: crypto::PublicKey<crypto::Curve25519_2HashDH>) -> VrfPublicKey {
        VrfPublicKey(vrf)
    }
}

#[wasm_bindgen]
impl VrfPublicKey {
    pub fn from_bech32(bech32_str: &str) -> Result<VrfPublicKey, JsValue> {
        crypto::PublicKey::try_from_bech32_str(&bech32_str)
            .map(VrfPublicKey)
            .map_err(|_| JsValue::from_str("Malformed vrf public key"))
    }
}

/// Amount of the balance in the transaction.
#[wasm_bindgen]
pub struct Balance(tx::Balance);

impl From<tx::Balance> for Balance {
    fn from(balance: tx::Balance) -> Balance {
        Balance(balance)
    }
}

#[wasm_bindgen]
impl Balance {
    //Not sure is this is the best way
    pub fn get_sign(&self) -> JsValue {
        JsValue::from_str(match self.0 {
            tx::Balance::Positive(_) => "positive",
            tx::Balance::Negative(_) => "negative",
            tx::Balance::Zero => "zero",
        })
    }

    /// Get value without taking into account if the balance is positive or negative
    pub fn get_value(&self) -> Value {
        match self.0 {
            tx::Balance::Positive(v) => Value(v),
            tx::Balance::Negative(v) => Value(v),
            tx::Balance::Zero => Value(value::Value(0)),
        }
    }
}

/// Algorithm used to compute transaction fees
/// Currently the only implementation if the Linear one
#[wasm_bindgen]
pub struct Fee(FeeVariant);

use fee::FeeAlgorithm;

#[wasm_bindgen]
impl Fee {
    /// Linear algorithm, this is formed by: `coefficient * (#inputs + #outputs) + constant + certificate * #certificate
    pub fn linear_fee(constant: Value, coefficient: Value, certificate: Value) -> Fee {
        Fee(FeeVariant::Linear(fee::LinearFee::new(
            *constant.0.as_ref(),
            *coefficient.0.as_ref(),
            *certificate.0.as_ref(),
        )))
    }

    /// Compute the fee if possible (it can fail in case the values are out of range)
    pub fn calculate(&self, tx: Transaction) -> Option<Value> {
        use EitherTransaction::TransactionWithCertificate;
        use EitherTransaction::TransactionWithoutCertificate;
        match (&self.0, tx.0) {
            (FeeVariant::Linear(algorithm), TransactionWithCertificate(ref tx)) => {
                algorithm.calculate(tx)
            }
            (FeeVariant::Linear(algorithm), TransactionWithoutCertificate(ref tx)) => {
                algorithm.calculate(tx)
            }
        }
        .map(Value)
    }
}

pub enum FeeVariant {
    Linear(fee::LinearFee),
}

/// Structure that proofs that certain user agrees with
/// some data. This structure is used to sign `Transaction`
/// and get `SignedTransaction` out.
///
/// It's important that witness works with opaque structures
/// and may not know the contents of the internal transaction.
#[wasm_bindgen]
pub struct Witness(tx::Witness);

#[wasm_bindgen]
impl Witness {
    /// Generate Witness for an utxo-based transaction Input
    pub fn for_utxo(
        genesis_hash: Hash,
        transaction_id: TransactionSignDataHash,
        secret_key: PrivateKey,
    ) -> Witness {
        Witness(tx::Witness::new_utxo(
            &genesis_hash.0,
            &transaction_id.0,
            &secret_key.0,
        ))
    }

    /// Generate Witness for an account based transaction Input
    /// the account-spending-counter should be incremented on each transaction from this account
    pub fn for_account(
        genesis_hash: Hash,
        transaction_id: TransactionSignDataHash,
        secret_key: PrivateKey,
        account_spending_counter: SpendingCounter,
    ) -> Witness {
        Witness(tx::Witness::new_account(
            &genesis_hash.0,
            &transaction_id.0,
            &account_spending_counter.0,
            &secret_key.0,
        ))
    }

    /// Get string representation
    pub fn to_bech32(&self) -> Result<String, JsValue> {
        let bytes = self
            .0
            .serialize_as_vec()
            .map_err(|error| JsValue::from_str(&format!("{}", error)))?;

        Bech32::new("witness".to_string(), bytes.to_base32())
            .map(|bech32| bech32.to_string())
            .map_err(|error| JsValue::from_str(&format!("{}", error)))
    }
}

#[wasm_bindgen]
pub struct SpendingCounter(account::SpendingCounter);

impl From<account::SpendingCounter> for SpendingCounter {
    fn from(spending_counter: account::SpendingCounter) -> SpendingCounter {
        SpendingCounter(spending_counter)
    }
}

/// Spending counter associated to an account.
///
/// every time the owner is spending from an account,
/// the counter is incremented. A matching counter
/// needs to be used in the spending phase to make
/// sure we have non-replayability of a transaction.
#[wasm_bindgen]
impl SpendingCounter {
    pub fn zero() -> Self {
        account::SpendingCounter::zero().into()
    }

    pub fn from_u32(counter: u32) -> Self {
        account::SpendingCounter::from(counter).into()
    }
}

/// All possible messages recordable in the Block content
#[wasm_bindgen]
#[derive(Clone)]
pub struct Fragment(chain::fragment::Fragment);

impl From<chain::fragment::Fragment> for Fragment {
    fn from(msg: chain::fragment::Fragment) -> Fragment {
        Fragment(msg)
    }
}

#[wasm_bindgen]
impl Fragment {
    pub fn from_generated_transaction(tx: GeneratedTransaction) -> Fragment {
        let msg = match tx.0 {
            chain::txbuilder::GeneratedTransaction::Type1(auth) => {
                chain::fragment::Fragment::Transaction(auth)
            }
            chain::txbuilder::GeneratedTransaction::Type2(auth) => {
                chain::fragment::Fragment::Certificate(auth)
            }
        };
        Fragment(msg)
    }

    /// Get a Transaction if the Fragment represents one
    pub fn get_transaction(self) -> Result<GeneratedTransaction, JsValue> {
        match self.0 {
            chain::fragment::Fragment::Transaction(auth) => {
                Ok(txbuilder::GeneratedTransaction::Type1(auth).into())
            }
            _ => Err(JsValue::from_str("Invalid message type")),
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>, JsValue> {
        self.0
            .serialize_as_vec()
            .map_err(|error| JsValue::from_str(&format!("{}", error)))
    }

    pub fn is_initial(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::Initial(_) => true,
            _ => false,
        }
    }

    pub fn is_transaction(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::Transaction(_) => true,
            _ => false,
        }
    }

    pub fn is_certificate(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::Certificate(_) => true,
            _ => false,
        }
    }

    pub fn is_old_utxo_declaration(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::OldUtxoDeclaration(_) => true,
            _ => false,
        }
    }

    pub fn is_update_proposal(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::UpdateProposal(_) => true,
            _ => false,
        }
    }

    pub fn is_update_vote(&self) -> bool {
        match self.0 {
            chain::fragment::Fragment::UpdateVote(_) => true,
            _ => false,
        }
    }
}

/// `Block` is an element of the blockchain it contains multiple
/// transaction and a reference to the parent block. Alongside
/// with the position of that block in the chain.
#[wasm_bindgen]
pub struct Block(chain::block::Block);

impl From<chain::block::Block> for Block {
    fn from(block: chain::block::Block) -> Block {
        Block(block)
    }
}

#[wasm_bindgen]
impl Block {
    /// Deserialize a block from a byte array
    pub fn from_bytes(bytes: Uint8Array) -> Result<Block, JsValue> {
        let mut slice: Box<[u8]> = vec![0; bytes.length() as usize].into_boxed_slice();
        bytes.copy_to(&mut *slice);
        chain::block::Block::deserialize(&*slice)
            .map_err(|e| JsValue::from_str(&format!("{}", e)))
            .map(Block)
    }

    pub fn id(&self) -> BlockId {
        self.0.id().into()
    }

    pub fn parent_id(&self) -> BlockId {
        self.0.parent_id().into()
    }

    ///This involves copying all the messages
    pub fn fragments(&self) -> Fragments {
        self.0
            .fragments()
            .map(|m| Fragment::from(m.clone()))
            .collect::<Vec<Fragment>>()
            .into()
    }
}

#[wasm_bindgen]
pub struct BlockId(chain::block::BlockId);

impl From<chain::block::BlockId> for BlockId {
    fn from(block_id: chain::block::BlockId) -> BlockId {
        BlockId(block_id)
    }
}

#[wasm_bindgen]
impl BlockId {
    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.serialize_as_vec().unwrap()
    }
}

#[wasm_bindgen]
pub struct FragmentId(chain::fragment::FragmentId);

impl From<chain::fragment::FragmentId> for FragmentId {
    fn from(fragment_id: chain::fragment::FragmentId) -> FragmentId {
        FragmentId(fragment_id)
    }
}

#[wasm_bindgen]
impl FragmentId {
    pub fn from_bytes(bytes: &[u8]) -> FragmentId {
        chain::fragment::FragmentId::hash_bytes(bytes).into()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.0.serialize_as_vec().unwrap()
    }
}

//this is useful for debugging, I'm not sure it is a good idea to have it here
//also, the 'hex' module in chain_crypto is private, so I cannot use that

#[wasm_bindgen]
pub fn uint8array_to_hex(input: JsValue) -> Result<String, JsValue> {
    //For some reason JSON.stringify serializes Uint8Array as objects instead of arrays
    let input_array: std::collections::BTreeMap<usize, u8> = input
        .into_serde()
        .map_err(|e| JsValue::from_str(&format!("{}", e)))?;

    let mut v = Vec::with_capacity(input_array.len() * 2);

    const ALPHABET: &'static [u8] = b"0123456789abcdef";
    for &byte in input_array.values() {
        v.push(ALPHABET[(byte >> 4) as usize]);
        v.push(ALPHABET[(byte & 0xf) as usize]);
    }

    String::from_utf8(v).map_err(|e| JsValue::from_str(&format!("{}", e)))
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
