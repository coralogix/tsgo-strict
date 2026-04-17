import type { Type2128 } from '../batch-10/file-2128.js';
export interface Type2129 {
  id: 2129;
  name: 'File2129';
  next: Type2128;
}

export function make2129(): Type2129 {
  return { id: 2129, name: 'File2129', next: null as unknown as Type2128 };
}
