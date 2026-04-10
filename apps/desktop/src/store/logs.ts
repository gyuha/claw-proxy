import { create } from 'zustand';
import type { LogEntry } from '../../../../packages/shared-types/src';

const MAX_LOGS = 500;

interface LogStore {
  logs: LogEntry[];
  addLog: (entry: LogEntry) => void;
  clear: () => void;
}

export const useLogStore = create<LogStore>((set) => ({
  logs: [],
  addLog: (entry) => set((state) => ({
    logs: [entry, ...state.logs].slice(0, MAX_LOGS),
  })),
  clear: () => set({ logs: [] }),
}));
