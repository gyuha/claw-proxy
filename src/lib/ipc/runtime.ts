import { invoke } from '@tauri-apps/api/core';

import {
  normalizeRuntimeSnapshot,
  type RuntimeSnapshot,
  type RuntimeStatus,
  type SerializedRuntimeSnapshot,
} from '../../features/runtime/models';

const runtimeCommandNames = {
  getRuntimeSnapshot: 'get_runtime_snapshot',
  initializeRuntimeState: 'initialize_runtime_state',
  pingRuntime: 'ping_runtime',
} as const;

export interface RuntimePingResponse {
  reachable: boolean;
  runtimeStatus: RuntimeStatus;
  status: string;
}

interface SerializedRuntimePingResponse {
  reachable: boolean;
  runtime_status: RuntimeStatus;
  status: string;
}

function normalizeRuntimePing(
  response: SerializedRuntimePingResponse,
): RuntimePingResponse {
  return {
    reachable: response.reachable,
    runtimeStatus: response.runtime_status,
    status: response.status,
  };
}

export async function getRuntimeSnapshot(): Promise<RuntimeSnapshot> {
  const snapshot = await invoke<SerializedRuntimeSnapshot>(
    runtimeCommandNames.getRuntimeSnapshot,
  );

  return normalizeRuntimeSnapshot(snapshot);
}

export async function initializeRuntimeState(): Promise<RuntimeSnapshot> {
  const snapshot = await invoke<SerializedRuntimeSnapshot>(
    runtimeCommandNames.initializeRuntimeState,
  );

  return normalizeRuntimeSnapshot(snapshot);
}

export async function pingRuntime(): Promise<RuntimePingResponse> {
  const response = await invoke<SerializedRuntimePingResponse>(
    runtimeCommandNames.pingRuntime,
  );

  return normalizeRuntimePing(response);
}

export { runtimeCommandNames };
