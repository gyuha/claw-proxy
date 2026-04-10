export interface ServerStatus {
  status: 'running' | 'stopped';
  uptime_secs: number;
  proxy_port: number;
  admin_port: number;
}

export interface ProviderInfo {
  name: string;
  available: boolean;
}

export interface LogEntry {
  timestamp: string;
  request_id: string;
  model: string;
  provider: string;
  source_format: 'openai' | 'anthropic';
  status_code: number;
  latency_ms: number;
}
