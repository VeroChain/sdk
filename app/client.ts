import * as anchor from "@coral-xyz/anchor"

export const provider = anchor.AnchorProvider.env()
anchor.setProvider(provider)

export const program = anchor.workspace.Verochain
