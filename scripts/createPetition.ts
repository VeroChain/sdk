import { createPetition } from "../app/petitions"

async function main() {
  await createPetition("On-chain Privacy Rights")
  console.log("Petition created on Vero Chain (Solana)")
}

main()
