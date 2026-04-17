import type { Type128 } from '../batch-00/file-0128.js';
export interface Type129 {
  id: 129;
  name: 'File129';
  next: Type128;
}

export function make129(): Type129 {
  return { id: 129, name: 'File129', next: null as unknown as Type128 };
}
