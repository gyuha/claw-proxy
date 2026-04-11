import { invoke } from '@tauri-apps/api/core';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';

import {
  normalizeProxyRuntimeSnapshot,
  normalizeProxySettings,
  type ProxyRuntimeSnapshot,
  type ProxySettings,
  type SerializedProxyRuntimeSnapshot,
  type SerializedProxySettings,
} from '../../features/proxy/models';

const proxyCommandNames = {
  applyProxySettings: 'apply_proxy_settings',
  getProxyRuntimeSnapshot: 'get_proxy_runtime_snapshot',
  getProxySettings: 'get_proxy_settings',
  startProxyRuntime: 'start_proxy_runtime',
  stopProxyRuntime: 'stop_proxy_runtime',
} as const;

export const proxyRuntimeUpdatedEvent = 'runtime://proxy-updated';

export interface ProxyCommandError {
  code: string;
  message: string;
}

function normalizeProxyCommandError(error: unknown): ProxyCommandError {
  if (
    typeof error === 'object' &&
    error !== null &&
    'message' in error &&
    typeof error.message === 'string'
  ) {
    const code =
      'code' in error && typeof error.code === 'string'
        ? error.code
        : 'proxy_command_failed';

    return {
      code,
      message: error.message,
    };
  }

  if (error instanceof Error) {
    return {
      code: 'proxy_command_failed',
      message: error.message,
    };
  }

  return {
    code: 'proxy_command_failed',
    message: 'Unable to reach the proxy command surface.',
  };
}

function serializeProxySettings(settings: ProxySettings): SerializedProxySettings {
  return {
    base_endpoint: settings.baseEndpoint,
    listen_host: settings.listenHost,
    listen_port: settings.listenPort,
  };
}

async function invokeProxyCommand<T>(
  command: (typeof proxyCommandNames)[keyof typeof proxyCommandNames],
  args?: Record<string, unknown>,
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw normalizeProxyCommandError(error);
  }
}

export async function getProxyRuntimeSnapshot(): Promise<ProxyRuntimeSnapshot> {
  const snapshot = await invokeProxyCommand<SerializedProxyRuntimeSnapshot>(
    proxyCommandNames.getProxyRuntimeSnapshot,
  );

  return normalizeProxyRuntimeSnapshot(snapshot);
}

export async function getProxySettings(): Promise<ProxySettings> {
  const settings = await invokeProxyCommand<SerializedProxySettings>(
    proxyCommandNames.getProxySettings,
  );

  return normalizeProxySettings(settings);
}

export async function startProxyRuntime(): Promise<ProxyRuntimeSnapshot> {
  const snapshot = await invokeProxyCommand<SerializedProxyRuntimeSnapshot>(
    proxyCommandNames.startProxyRuntime,
  );

  return normalizeProxyRuntimeSnapshot(snapshot);
}

export async function stopProxyRuntime(): Promise<ProxyRuntimeSnapshot> {
  const snapshot = await invokeProxyCommand<SerializedProxyRuntimeSnapshot>(
    proxyCommandNames.stopProxyRuntime,
  );

  return normalizeProxyRuntimeSnapshot(snapshot);
}

export async function applyProxySettings(
  settings: ProxySettings,
): Promise<ProxyRuntimeSnapshot> {
  const snapshot = await invokeProxyCommand<SerializedProxyRuntimeSnapshot>(
    proxyCommandNames.applyProxySettings,
    {
      settings: serializeProxySettings(settings),
    },
  );

  return normalizeProxyRuntimeSnapshot(snapshot);
}

export async function subscribeToProxyRuntimeUpdates(
  listener: () => void | Promise<void>,
): Promise<UnlistenFn> {
  return listen(proxyRuntimeUpdatedEvent, () => {
    void listener();
  });
}

export { proxyCommandNames };
