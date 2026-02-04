import { program } from "./client"

export async function createPetition(title: string) {
  return program.methods.createPetition(title).rpc()
}
