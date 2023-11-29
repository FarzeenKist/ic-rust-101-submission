import type {
  ActorSubclass,
  HttpAgentOptions,
  ActorConfig,
  Agent,
} from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";
import type { ServiceFactory } from "@dfinity/candid";

import { _SERVICE } from './icp_rust_boilerplate_backend.did';

export declare const idlFactory: ServiceFactory;
export declare const canisterId: string | Principal;

export declare interface CreateActorOptions {
  canisterId: string | Principal; // Added the required property canisterId
  agent?: Agent;
  agentOptions?: HttpAgentOptions;
  actorOptions?: ActorConfig;
}

export declare const createActor: (
  canisterId: string | Principal, // Added a required parameter canisterId
  options?: CreateActorOptions
) => ActorSubclass<_SERVICE>;

export declare const icp_rust_boilerplate_backend: ActorSubclass<_SERVICE>;

export declare const isValidCanisterId: (canisterId: string | Principal) => boolean;

export const createActor = (canisterId: string | Principal, options?: CreateActorOptions): ActorSubclass<_SERVICE> => {
  if (!isValidCanisterId(canisterId)) {
    throw new Error("Invalid Canister ID");
  }

  if (options?.agent && !isValidAgent(options.agent)) {
    throw new Error("Invalid Agent");
  }

  if (options?.agentOptions && !isValidAgentOptions(options.agentOptions)) {
    throw new Error("Invalid Agent Options");
  }

  if (options?.actorOptions && !isValidActorOptions(options.actorOptions)) {
    throw new Error("Invalid Actor Options");
  }

  // Proceed with actor creation
};

export const isValidCanisterId = (canisterId: string | Principal): boolean => {
  // Check for a valid canister ID (perform necessary checks)
  // Return true or false based on validation
  return true; // Placeholder logic - replace this with actual validation
};

export const isValidAgent = (agent: Agent): boolean => {
  // Check for a valid Agent (perform necessary checks)
  // Return true or false based on validation
  return true; // Placeholder logic - replace this with actual validation
};

export const isValidAgentOptions = (agentOptions: HttpAgentOptions): boolean => {
  // Check for valid Agent Options (perform necessary checks)
  // Return true or false based on validation
  return true; // Placeholder logic - replace this with actual validation
};

export const isValidActorOptions = (actorOptions: ActorConfig): boolean => {
  // Check for valid Actor Options (perform necessary checks)
  // Return true or false based on validation
  return true; // Placeholder logic - replace this with actual validation
};
