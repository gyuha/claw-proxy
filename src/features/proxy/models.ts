export const proxyRuntimeStatusValues = [
  'starting',
  'healthy',
  'stopped',
  'misconfigured',
] as const;

export type ProxyRuntimeStatus = (typeof proxyRuntimeStatusValues)[number];
export const loopbackHostOptions = ['localhost', '127.0.0.1', '::1'] as const;
export type ProxyListenHost = (typeof loopbackHostOptions)[number];

export interface SerializedProxySettings {
  base_endpoint: string;
  listen_host: string;
  listen_port: number;
}

export interface ProxySettings {
  baseEndpoint: string;
  listenHost: string;
  listenPort: number;
}

export interface SerializedProxyRuntimeSnapshot {
  effective_base_url: string;
  last_error: string | null;
  last_transition_at: string | null;
  settings: SerializedProxySettings;
  status: ProxyRuntimeStatus;
}

export interface ProxyRuntimeSnapshot {
  effectiveBaseUrl: string;
  lastError: string | null;
  lastTransitionAt: string | null;
  settings: ProxySettings;
  status: ProxyRuntimeStatus;
}

const defaultProxySettings = {
  baseEndpoint: '/v1',
  listenHost: '127.0.0.1',
  listenPort: 8787,
} satisfies ProxySettings;

export function createInitialProxySettings(): ProxySettings {
  return { ...defaultProxySettings };
}

export function createInitialProxyRuntimeSnapshot(): ProxyRuntimeSnapshot {
  return {
    effectiveBaseUrl: buildProxyEffectiveBaseUrl(defaultProxySettings),
    lastError: null,
    lastTransitionAt: null,
    settings: createInitialProxySettings(),
    status: 'stopped',
  };
}

export function normalizeProxySettings(
  settings: SerializedProxySettings,
): ProxySettings {
  return {
    baseEndpoint: normalizeBaseEndpoint(settings.base_endpoint),
    listenHost: settings.listen_host,
    listenPort: settings.listen_port,
  };
}

export function normalizeProxyRuntimeSnapshot(
  snapshot: SerializedProxyRuntimeSnapshot,
): ProxyRuntimeSnapshot {
  const settings = normalizeProxySettings(snapshot.settings);

  return {
    effectiveBaseUrl:
      snapshot.effective_base_url || buildProxyEffectiveBaseUrl(settings),
    lastError: snapshot.last_error,
    lastTransitionAt: snapshot.last_transition_at,
    settings,
    status: snapshot.status,
  };
}

export function buildProxyEffectiveBaseUrl(settings: ProxySettings): string {
  const authority =
    settings.listenHost === '::1' ? `[${settings.listenHost}]` : settings.listenHost;

  return `http://${authority}:${settings.listenPort}${settings.baseEndpoint}`;
}

function normalizeBaseEndpoint(baseEndpoint: string): string {
  const trimmed = baseEndpoint.trim();
  if (!trimmed) {
    return '/';
  }

  return trimmed.startsWith('/') ? trimmed : `/${trimmed}`;
}
