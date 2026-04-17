import type { Type3600 } from '../batch-18/file-3600.js';
export interface Type3601 {
  id: 3601;
  name: 'File3601';
  next: Type3600;
}

export function make3601(): Type3601 {
  return { id: 3601, name: 'File3601', next: null as unknown as Type3600 };
}
