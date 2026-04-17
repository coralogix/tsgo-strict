import type { Type2250 } from '../batch-11/file-2250.js';
export interface Type2251 {
  id: 2251;
  name: 'File2251';
  next: Type2250;
}

export function make2251(): Type2251 {
  return { id: 2251, name: 'File2251', next: null as unknown as Type2250 };
}
