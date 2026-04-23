import type { Type1801 } from '../batch-09/file-1801.js';
export interface Type1802 {
  id: 1802;
  name: 'File1802';
  next: Type1801;
}

export function make1802(): Type1802 {
  return { id: 1802, name: 'File1802', next: null as unknown as Type1801 };
}
