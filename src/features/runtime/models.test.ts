import { describe, expect, it } from 'vitest';

import {
  createInitialRuntimeSnapshot,
  runtimeStatusValues,
  type RuntimeSnapshot,
} from './models';

describe('runtime models', () => {
  it('exposes stable shell fields for the runtime snapshot', () => {
    expect(runtimeStatusValues).toEqual(['starting', 'ready', 'stopped', 'error']);

    expect(createInitialRuntimeSnapshot()).toEqual({
      accountSlots: 0,
      activeProfile: null,
      appReady: false,
      lastHealthCheck: null,
      providerSlots: 0,
      runtimeStatus: 'starting',
    });
  });

  it('keeps the consumer contract in camelCase', () => {
    const snapshot = {
      accountSlots: 3,
      activeProfile: 'desktop-default',
      appReady: true,
      lastHealthCheck: '1712793600',
      providerSlots: 2,
      runtimeStatus: 'ready',
    } satisfies RuntimeSnapshot;

    expect(Object.keys(snapshot)).toEqual([
      'accountSlots',
      'activeProfile',
      'appReady',
      'lastHealthCheck',
      'providerSlots',
      'runtimeStatus',
    ]);
  });
});
