import { cleanup, fireEvent, render } from '@testing-library/react';
import { afterEach, describe, expect, it, vi } from 'vitest';

import AppShell from '../../../components/shell/AppShell';
import type { ProxyRuntimeSnapshot, ProxySettings } from '../models';
import type { ProxyControlState } from '../state';
import type { RuntimeSnapshot } from '../../runtime/models';

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

function createProxyControlState(
  overrides: Partial<ProxyControlState> = {},
): ProxyControlState {
  return {
    applySettings: vi.fn().mockResolvedValue(undefined),
    draft: createProxySettings(),
    error: null,
    hostOptions: ['localhost', '127.0.0.1', '::1'],
    isApplying: false,
    loading: false,
    refresh: vi.fn().mockResolvedValue(undefined),
    snapshot: createProxySnapshot(),
    startProxy: vi.fn().mockResolvedValue(undefined),
    stopProxy: vi.fn().mockResolvedValue(undefined),
    updateDraft: vi.fn(),
    ...overrides,
  };
}

describe('proxy shell status surface', () => {
  afterEach(() => {
    cleanup();
  });

  it('renders Start Proxy, Stop Proxy, and Apply Settings controls inside the shell', () => {
    const proxy = createProxyControlState({
      snapshot: createProxySnapshot({
        lastError: 'Previous apply failed.',
        status: 'misconfigured',
      }),
    });
    const { getByLabelText, getByRole, getByText } = render(
      <AppShell
        error={null}
        loading={false}
        proxy={proxy}
        snapshot={createRuntimeSnapshot()}
      />,
    );

    expect(getByRole('button', { name: 'Start Proxy' })).toBeInTheDocument();
    expect(getByRole('button', { name: 'Stop Proxy' })).toBeInTheDocument();
    expect(getByRole('button', { name: 'Apply Settings' })).toBeInTheDocument();
    expect(getByLabelText('Listen Host')).toHaveValue('127.0.0.1');
    expect(getByLabelText('Listen Port')).toHaveValue(8787);
    expect(getByLabelText('Base Endpoint')).toHaveValue('/v1');
    expect(getByText('Effective Local URL')).toBeInTheDocument();

    fireEvent.click(getByRole('button', { name: 'Start Proxy' }));
    fireEvent.click(getByRole('button', { name: 'Stop Proxy' }));
    fireEvent.click(getByRole('button', { name: 'Apply Settings' }));

    expect(proxy.startProxy).toHaveBeenCalledTimes(1);
    expect(proxy.stopProxy).toHaveBeenCalledTimes(1);
    expect(proxy.applySettings).toHaveBeenCalledTimes(1);
  });

  it('renders Healthy, Stopped, and Misconfigured proxy states with host-owned error feedback', () => {
    const proxy = createProxyControlState({
      snapshot: createProxySnapshot({
        effectiveBaseUrl: 'http://127.0.0.1:8787/v1',
        lastError: 'Listen port already in use.',
        status: 'misconfigured',
      }),
    });
    const { getByText } = render(
      <AppShell
        error={null}
        loading={false}
        proxy={proxy}
        snapshot={createRuntimeSnapshot()}
      />,
    );

    expect(getByText('Misconfigured')).toBeInTheDocument();
    expect(getByText('Listen port already in use.')).toBeInTheDocument();
    expect(getByText('Active Local URL')).toBeInTheDocument();
  });

  it('disables lifecycle and apply controls while proxy state is loading', () => {
    const proxy = createProxyControlState({ loading: true });
    const { getByRole } = render(
      <AppShell
        error={null}
        loading={false}
        proxy={proxy}
        snapshot={createRuntimeSnapshot()}
      />,
    );

    expect(getByRole('button', { name: 'Start Proxy' })).toBeDisabled();
    expect(getByRole('button', { name: 'Stop Proxy' })).toBeDisabled();
    expect(getByRole('button', { name: 'Apply Settings' })).toBeDisabled();
  });
});
