import type { Type801 } from '../batch-04/file-0801.js';
export interface Type802 {
  id: 802;
  name: 'File802';
  next: Type801;
}

export function make802(): Type802 {
  return { id: 802, name: 'File802', next: null as unknown as Type801 };
}
