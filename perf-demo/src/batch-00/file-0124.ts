import type { Type123 } from '../batch-00/file-0123.js';
export interface Type124 {
  id: 124;
  name: 'File124';
  next: Type123;
}

export function make124(): Type124 {
  return { id: 124, name: 'File124', next: null as unknown as Type123 };
}
