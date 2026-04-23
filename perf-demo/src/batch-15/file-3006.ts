import type { Type3005 } from '../batch-15/file-3005.js';
export interface Type3006 {
  id: 3006;
  name: 'File3006';
  next: Type3005;
}

export function make3006(): Type3006 {
  return { id: 3006, name: 'File3006', next: null as unknown as Type3005 };
}
