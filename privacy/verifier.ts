export function verifyPrivateProof(proof: string): boolean {
  return proof.startsWith("vero_proof_")
}
