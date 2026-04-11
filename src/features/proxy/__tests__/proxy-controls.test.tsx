import { act, cleanup, renderHook, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';

import type { ProxyRuntimeSnapshot, ProxySettings } from '../models';
import { useProxyControlState } from '../state';

const applyProxySettings = vi.fn();
const getProxyRuntimeSnapshot = vi.fn();
const getProxySettings = vi.fn();
const startProxyRuntime = vi.fn();
const stopProxyRuntime = vi.fn();
const subscribeToProxyRuntimeUpdates = vi.fn();

let runtimeUpdateListener: (() => void | Promise<void>) | undefined;

function createProxySettings(
  overrides: Partial<ProxySettings> = {},
): ProxySettings {
  return {
    baseEndpoint: '/v1',
    listenHost: '127.0.0.1',
    listenPort: 8787,
    ...overrides,
  };
}

function createProxySnapshot(
  overrides: Partial<ProxyRuntimeSnapshot> = {},
): ProxyRuntimeSnapshot {
  const settings = createProxySettings(overrides.settings);

  return {
    effectiveBaseUrl: `http://${settings.listenHost}:${settings.listenPort}${settings.baseEndpoint}`,
    lastError: null,
    lastTransitionAt: '1712793600',
    settings,
    status: 'stopped',
    ...overrides,
  };
}

vi.mock('../api', () => ({
  applyProxySettings: (settings: ProxySettings) => applyProxySettings(settings),
  getProxyRuntimeSnapshot: () => getProxyRuntimeSnapshot(),
  getProxySettings: () => getProxySettings(),
  startProxyRuntime: () => startProxyRuntime(),
  stopProxyRuntime: () => stopProxyRuntime(),
  subscribeToProxyRuntimeUpdates: (
    listener: () => void | Promise<void>,
  ) => subscribeToProxyRuntimeUpdates(listener),
}));

describe('proxy control state', () => {
  afterEach(() => {
    cleanup();
  });

  beforeEach(() => {
    applyProxySettings.mockReset();
    getProxyRuntimeSnapshot.mockReset();
    getProxySettings.mockReset();
    startProxyRuntime.mockReset();
    stopProxyRuntime.mockReset();
    subscribeToProxyRuntimeUpdates.mockReset();
    runtimeUpdateListener = undefined;

    getProxyRuntimeSnapshot.mockResolvedValue(createProxySnapshot());
    getProxySettings.mockResolvedValue(createProxySettings());
    startProxyRuntime.mockResolvedValue(createProxySnapshot({ status: 'healthy' }));
    stopProxyRuntime.mockResolvedValue(createProxySnapshot({ status: 'stopped' }));
    applyProxySettings.mockResolvedValue(
      createProxySnapshot({ status: 'healthy' }),
    );
    subscribeToProxyRuntimeUpdates.mockImplementation(
      async (listener: () => void | Promise<void>) => {
        runtimeUpdateListener = listener;

        return () => {
          runtimeUpdateListener = undefined;
        };
      },
    );
  });

  it('loads current proxy settings and runtime snapshot through typed helpers on mount', async () => {
    const { result } = renderHook(() => useProxyControlState());

    await waitFor(() => expect(getProxyRuntimeSnapshot).toHaveBeenCalledTimes(1));
    await waitFor(() => expect(getProxySettings).toHaveBeenCalledTimes(1));

    expect(result.current.draft).toEqual(createProxySettings());
    expect(result.current.snapshot.status).toBe('stopped');
    expect(result.current.hostOptions).toEqual(['localhost', '127.0.0.1', '::1']);
  });

  it('submits draft settings through apply_proxy_settings and refreshes canonical state after success', async () => {
    const nextSettings = createProxySettings({
      baseEndpoint: '/desktop',
      listenHost: 'localhost',
      listenPort: 9898,
    });

    getProxyRuntimeSnapshot
      .mockResolvedValueOnce(createProxySnapshot())
      .mockResolvedValueOnce(
        createProxySnapshot({
          effectiveBaseUrl: 'http://localhost:9898/desktop',
          settings: nextSettings,
          status: 'healthy',
        }),
      );
    getProxySettings
      .mockResolvedValueOnce(createProxySettings())
      .mockResolvedValueOnce(nextSettings);

    const { result } = renderHook(() => useProxyControlState());

    await waitFor(() => expect(result.current.loading).toBe(false));

    act(() => {
      result.current.updateDraft('listenHost', 'localhost');
      result.current.updateDraft('listenPort', 9898);
      result.current.updateDraft('baseEndpoint', '/desktop');
    });

    await waitFor(() => expect(result.current.draft).toEqual(nextSettings));

    await act(async () => {
      await result.current.applySettings();
    });

    expect(applyProxySettings).toHaveBeenCalledWith(nextSettings);
    await waitFor(() =>
      expect(result.current.snapshot.effectiveBaseUrl).toBe(
        'http://localhost:9898/desktop',
      ),
    );
    expect(result.current.draft).toEqual(nextSettings);
  });

  it('refreshes from host state when runtime://proxy-updated invalidation arrives', async () => {
    getProxyRuntimeSnapshot
      .mockResolvedValueOnce(createProxySnapshot())
      .mockResolvedValueOnce(
        createProxySnapshot({
          settings: createProxySettings({ baseEndpoint: '/healthy' }),
          status: 'healthy',
        }),
      );
    getProxySettings
      .mockResolvedValueOnce(createProxySettings())
      .mockResolvedValueOnce(createProxySettings({ baseEndpoint: '/healthy' }));

    const { result } = renderHook(() => useProxyControlState());

    await waitFor(() => expect(result.current.loading).toBe(false));

    await act(async () => {
      await runtimeUpdateListener?.();
    });

    await waitFor(() => expect(getProxyRuntimeSnapshot).toHaveBeenCalledTimes(2));
    expect(result.current.snapshot.status).toBe('healthy');
    expect(result.current.snapshot.settings.baseEndpoint).toBe('/healthy');
  });
});
