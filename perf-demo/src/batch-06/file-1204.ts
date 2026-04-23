import type { Type1203 } from '../batch-06/file-1203.js';
export interface Type1204 {
  id: 1204;
  name: 'File1204';
  next: Type1203;
}

export function make1204(): Type1204 {
  return { id: 1204, name: 'File1204', next: null as unknown as Type1203 };
}
