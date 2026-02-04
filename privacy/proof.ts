export function generatePrivateProof(pubkey: string) {
  return `vero_proof_${pubkey}_${Date.now()}`
}
