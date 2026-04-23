import type { Type3004 } from '../batch-15/file-3004.js';
export interface Type3005 {
  id: 3005;
  name: 'File3005';
  next: Type3004;
}

export function make3005(): Type3005 {
  return { id: 3005, name: 'File3005', next: null as unknown as Type3004 };
}
