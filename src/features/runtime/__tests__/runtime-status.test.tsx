import { renderHook, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useRuntimeShellState } from '../state';
import type { RuntimeSnapshot } from '../models';

const initializeRuntimeState = vi.fn();
const getRuntimeSnapshot = vi.fn();

function createRuntimeSnapshot(
  overrides: Partial<RuntimeSnapshot> = {},
): RuntimeSnapshot {
  return {
    accountSlots: 2,
    activeProfile: 'desktop-default',
    appReady: true,
    lastHealthCheck: '1712793600',
    providerSlots: 1,
    runtimeStatus: 'ready',
    ...overrides,
  };
}

vi.mock('../api', () => ({
  getRuntimeSnapshot: () => getRuntimeSnapshot(),
  initializeRuntimeState: () => initializeRuntimeState(),
}));

describe('runtime shell state', () => {
  beforeEach(() => {
    initializeRuntimeState.mockReset();
    getRuntimeSnapshot.mockReset();
  });

  it('initializes the shell through typed runtime helpers', async () => {
    initializeRuntimeState.mockResolvedValue(createRuntimeSnapshot());

    const { result } = renderHook(() => useRuntimeShellState());

    await waitFor(() => expect(initializeRuntimeState).toHaveBeenCalledTimes(1));
    await waitFor(() =>
      expect(result.current.snapshot.runtimeStatus).toBe('ready'),
    );
    expect(result.current.snapshot.providerSlots).toBe(1);
  });

  it('exposes snapshot fields in a render-friendly shape', async () => {
    getRuntimeSnapshot.mockResolvedValue(
      createRuntimeSnapshot({
        accountSlots: 4,
        providerSlots: 3,
      }),
    );

    const { result } = renderHook(() => useRuntimeShellState());

    await result.current.refresh();

    await waitFor(() => expect(result.current.snapshot.accountSlots).toBe(4));
    expect(result.current.snapshot.activeProfile).toBe('desktop-default');
    expect(result.current.loading).toBe(false);
  });
});
