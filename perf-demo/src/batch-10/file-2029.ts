import type { Type2028 } from '../batch-10/file-2028.js';
export interface Type2029 {
  id: 2029;
  name: 'File2029';
  next: Type2028;
}

export function make2029(): Type2029 {
  return { id: 2029, name: 'File2029', next: null as unknown as Type2028 };
}
