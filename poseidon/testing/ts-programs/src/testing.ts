import {
  Account,
  AssociatedTokenAccount,
  Mint,
  Pubkey,
  Signer,
  TokenProgram,
  u64,
  u8,
  SystemProgram,
  String,
  MetadataProgram,
  CreateMetadataV3,
  DataV2,
  None,
  Vec,
  UncheckedAccount,
  Metadata,
  u16,
  Collection,
  Uses,
  UseMethod,
  Creator,
} from "@solanaturbine/poseidon";
import { none, some } from "@solanaturbine/poseidon/index";

export default class TestingProgram {
  static PROGRAM_ID = new Pubkey(
    "CYEX79QtyVPz99LtKWamTvmMmGpMkKGH3mBF2tpRGmEs"
  );

  createTokenMint(
    key: Pubkey,
    payer: Signer,
    mint_account: Mint,
    metadata_account: Metadata, 
    token_decimals:u8,
    token_name: String<20>,
    token_symbol: String<15>,
    token_uri: String<100>,
    seller_fee_basis_points: u16,
    // remaining: u64,
    // total: u64
  ) {

    // let coded = "COD3D"
    // SolanaProgram.msg("HELLO THIS IS ME SPEAKING");
    // SolanaProgram.msg(`this is the new string ${coded}`);
    //`hello this message is ${coded}`
    //msg!("hello this message is COD3D")
    //let code:String = "COD3D"
    // SolanaProgram.msg("hello this message is {}", coded)
    // you would have to iterate through the string to find the opening bracket and the closing bracket then place the string variable there

    //order of accounts in the methods 

    /// CHECK: Validate address by deriving pda
    mint_account.derive(token_decimals, payer.key, payer.key).init(payer)
    metadata_account.derive(["metadata", mint_account.key]);
    MetadataProgram.createMetadataAccountsV3(
      new CreateMetadataV3(
        payer,
        payer,
        payer,
        mint_account // mint account
      ),
      new DataV2(
        token_name,
        token_symbol,
        token_uri,
        seller_fee_basis_points,
        none(),
        none(),
        none()
      ),
      // best thing to do , create an empty type for the metadataprogram 
      // new DataV2(
      //   token_name,
      //   token_symbol,
      //   token_uri,
      //   seller_fee_basis_points,
      //   // some(new Vec<Creator,10>()),
      //   none(),
      //   some(new Collection(true, key)),
      //   some(new Uses(UseMethod.Burn, remaining, total))
      //   // some(new Uses(UseMethod.Burn, new u64(5), new u64(10)))
      //   // none(),
      // ),
      true,
      false,
      none()
    );
  }
}

// TokenProgram.burn(
//   mint_account,
//   mint_account,
//   mint_account,
//   new u64(9921928)
// )

// new DataV2(
//   token_name,
//   token_symbol,
//   token_uri,
//   seller_fee_basis_points,
//   some(new Vec<Creator,10>()),
//   some(new Collection(true, key)),
//   some(new Uses(UseMethod.Burn, new u64(5), new u64(10)))
// ),

// some(new Collection{verified:true, key:key})

// export interface AddressData extends Account {
//   payer: Pubkey;
//   name: String<20>;
//   address: String<20>;
//   accountBump: u8;
// }
