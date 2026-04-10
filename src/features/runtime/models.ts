export const runtimeStatusValues = [
  'starting',
  'ready',
  'stopped',
  'error',
] as const;

export type RuntimeStatus = (typeof runtimeStatusValues)[number];

export interface SerializedRuntimeSnapshot {
  account_slots: number;
  active_profile: string | null;
  app_ready: boolean;
  last_health_check: string | null;
  provider_slots: number;
  runtime_status: RuntimeStatus;
}

export interface RuntimeSnapshot {
  accountSlots: number;
  activeProfile: string | null;
  appReady: boolean;
  lastHealthCheck: string | null;
  providerSlots: number;
  runtimeStatus: RuntimeStatus;
}

export function createInitialRuntimeSnapshot(): RuntimeSnapshot {
  return {
    accountSlots: 0,
    activeProfile: null,
    appReady: false,
    lastHealthCheck: null,
    providerSlots: 0,
    runtimeStatus: 'starting',
  };
}

export function normalizeRuntimeSnapshot(
  snapshot: SerializedRuntimeSnapshot,
): RuntimeSnapshot {
  return {
    accountSlots: snapshot.account_slots,
    activeProfile: snapshot.active_profile,
    appReady: snapshot.app_ready,
    lastHealthCheck: snapshot.last_health_check,
    providerSlots: snapshot.provider_slots,
    runtimeStatus: snapshot.runtime_status,
  };
}
