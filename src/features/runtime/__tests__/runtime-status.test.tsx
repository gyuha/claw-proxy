import { renderHook, waitFor } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';

import { useRuntimeShellState } from '../state';

const initializeRuntimeState = vi.fn();
const getRuntimeSnapshot = vi.fn();

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
    initializeRuntimeState.mockResolvedValue({
      accountSlots: 2,
      activeProfile: 'desktop-default',
      appReady: true,
      lastHealthCheck: '1712793600',
      providerSlots: 1,
      runtimeStatus: 'ready',
    });

    const { result } = renderHook(() => useRuntimeShellState());

    await waitFor(() => expect(initializeRuntimeState).toHaveBeenCalledTimes(1));
    await waitFor(() =>
      expect(result.current.snapshot.runtimeStatus).toBe('ready'),
    );
    expect(result.current.snapshot.providerSlots).toBe(1);
  });

  it('exposes snapshot fields in a render-friendly shape', async () => {
    getRuntimeSnapshot.mockResolvedValue({
      accountSlots: 4,
      activeProfile: 'desktop-default',
      appReady: true,
      lastHealthCheck: '1712793600',
      providerSlots: 3,
      runtimeStatus: 'ready',
    });

    const { result } = renderHook(() => useRuntimeShellState());

    await result.current.refresh();

    await waitFor(() => expect(result.current.snapshot.accountSlots).toBe(4));
    expect(result.current.snapshot.activeProfile).toBe('desktop-default');
    expect(result.current.loading).toBe(false);
  });
});
