import { describe, expect, it } from 'vitest';

import {
  createInitialProxyRuntimeSnapshot,
  normalizeProxyRuntimeSnapshot,
  proxyRuntimeStatusValues,
  type ProxyRuntimeSnapshot,
} from './models';

describe('proxy models', () => {
  it('exposes stable camelCase fields for the proxy runtime snapshot', () => {
    expect(proxyRuntimeStatusValues).toEqual([
      'starting',
      'healthy',
      'stopped',
      'misconfigured',
    ]);

    expect(createInitialProxyRuntimeSnapshot()).toEqual({
      effectiveBaseUrl: 'http://127.0.0.1:8787/v1',
      lastError: null,
      lastTransitionAt: null,
      settings: {
        baseEndpoint: '/v1',
        listenHost: '127.0.0.1',
        listenPort: 8787,
      },
      status: 'stopped',
    });
  });

  it('normalizes serialized proxy snapshots for UI consumers', () => {
    const snapshot = normalizeProxyRuntimeSnapshot({
      effective_base_url: 'http://localhost:8787/v1',
      last_error: 'bind failed',
      last_transition_at: '1712793600',
      settings: {
        base_endpoint: 'v1',
        listen_host: 'localhost',
        listen_port: 8787,
      },
      status: 'misconfigured',
    });

    expect(snapshot).toEqual({
      effectiveBaseUrl: 'http://localhost:8787/v1',
      lastError: 'bind failed',
      lastTransitionAt: '1712793600',
      settings: {
        baseEndpoint: '/v1',
        listenHost: 'localhost',
        listenPort: 8787,
      },
      status: 'misconfigured',
    } satisfies ProxyRuntimeSnapshot);
  });
});
