import type { Type210 } from '../batch-01/file-0210.js';
export interface Type211 {
  id: 211;
  name: 'File211';
  next: Type210;
}

export function make211(): Type211 {
  return { id: 211, name: 'File211', next: null as unknown as Type210 };
}
