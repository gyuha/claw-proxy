import {
  getRuntimeSnapshot as getRuntimeSnapshotFromHost,
  initializeRuntimeState as initializeRuntimeStateFromHost,
  pingRuntime,
  type RuntimePingResponse,
} from '../../lib/ipc/runtime';
import type { RuntimeSnapshot } from './models';

export async function getRuntimeSnapshot(): Promise<RuntimeSnapshot> {
  return getRuntimeSnapshotFromHost();
}

export async function initializeRuntimeState(): Promise<RuntimeSnapshot> {
  return initializeRuntimeStateFromHost();
}

export async function pingRuntimeStatus(): Promise<RuntimePingResponse> {
  return pingRuntime();
}
