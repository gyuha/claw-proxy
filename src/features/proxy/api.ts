import {
  applyProxySettings as applyProxySettingsFromHost,
  getProxyRuntimeSnapshot as getProxyRuntimeSnapshotFromHost,
  getProxySettings as getProxySettingsFromHost,
  startProxyRuntime as startProxyRuntimeFromHost,
  stopProxyRuntime as stopProxyRuntimeFromHost,
  subscribeToProxyRuntimeUpdates as subscribeToProxyRuntimeUpdatesFromHost,
} from '../../lib/ipc/proxy';
import type { ProxyRuntimeSnapshot, ProxySettings } from './models';

export async function getProxyRuntimeSnapshot(): Promise<ProxyRuntimeSnapshot> {
  return getProxyRuntimeSnapshotFromHost();
}

export async function getProxySettings(): Promise<ProxySettings> {
  return getProxySettingsFromHost();
}

export async function startProxyRuntime(): Promise<ProxyRuntimeSnapshot> {
  return startProxyRuntimeFromHost();
}

export async function stopProxyRuntime(): Promise<ProxyRuntimeSnapshot> {
  return stopProxyRuntimeFromHost();
}

export async function applyProxySettings(
  settings: ProxySettings,
): Promise<ProxyRuntimeSnapshot> {
  return applyProxySettingsFromHost(settings);
}

export async function subscribeToProxyRuntimeUpdates(
  listener: () => void | Promise<void>,
): Promise<() => void> {
  return subscribeToProxyRuntimeUpdatesFromHost(listener);
}
