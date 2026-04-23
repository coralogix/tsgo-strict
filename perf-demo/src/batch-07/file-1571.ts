import type { Type1570 } from '../batch-07/file-1570.js';
export interface Type1571 {
  id: 1571;
  name: 'File1571';
  next: Type1570;
}

export function make1571(): Type1571 {
  return { id: 1571, name: 'File1571', next: null as unknown as Type1570 };
}
