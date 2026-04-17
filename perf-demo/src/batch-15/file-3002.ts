import type { Type3001 } from '../batch-15/file-3001.js';
export interface Type3002 {
  id: 3002;
  name: 'File3002';
  next: Type3001;
}

export function make3002(): Type3002 {
  return { id: 3002, name: 'File3002', next: null as unknown as Type3001 };
}
