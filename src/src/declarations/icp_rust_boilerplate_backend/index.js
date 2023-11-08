import { Actor, HttpAgent } from "@dfinity/agent";

// Imports and re-exports candid interface
import { idlFactory } from "./icp_rust_boilerplate_backend.did.js";
export { idlFactory } from "./icp_rust_boilerplate_backend.did.js";

/* CANISTER_ID is replaced by webpack based on the node environment
 * Note: canister environment variable will be standardized as
 * process.env.CANISTER_ID_<CANISTER_NAME_UPPERCASE>
 * beginning in dfx 0.15.0
 */
export const canisterId =
  process.env.CANISTER_ID_ICP_RUST_BOILERPLATE_BACKEND ||
  process.env.ICP_Rust_Boilerplate_Backend_Canister_ID;

// Cached actor instance
let cachedActor = null;

export const createActor = (canisterId, options = {}) => {
  if (cachedActor) {
    return cachedActor; // Return the cached actor if it exists
  }

  const agent = options.agent || new HttpAgent({ ...options.agentOptions });

  if (options.agent && options.agentOptions) {
    console.warn(
      "Detected both agent and agentOptions passed to createActor. Ignoring agentOptions and proceeding with the provided agent."
    );
  }

  // Fetch root key for certificate validation during development
  if (process.env.DFX_NETWORK !== "ic") {
    try {
      await agent.fetchRootKey();
    } catch (err) {
      throw new Error("Unable to fetch root key. Make sure your local replica is running.");
    }
  }

  // Check if the canisterId is valid
  if (!Actor.isValidCanisterId(canisterId)) {
    throw new Error("Invalid canisterId. Please provide a valid canister ID.");
  }

  // Creates an actor with the candid interface and the HttpAgent
  cachedActor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
    ...options.actorOptions,
  });

  return cachedActor;
};

export const icp_rust_boilerplate_backend = createActor(canisterId);
