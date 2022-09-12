elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(PartialEq, TypeAbi,TopEncode,TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct EscrowFormat<M: ManagedTypeApi> {
    pub token_send: TokenIdentifier<M>,
    pub amount_send: BigUint<M>,
    pub token_receive: TokenIdentifier<M>,
    pub amount_receive: BigUint<M>,
}
  
#[derive(TypeAbi,TopEncode,TopDecode, NestedEncode, NestedDecode, Clone)]
pub struct EscrowWalletFormat<M: ManagedTypeApi> {
    pub wallet: ManagedAddress<M>,
    pub data: EscrowFormat<M>
}
    
 