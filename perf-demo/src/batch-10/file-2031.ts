import type { Type2030 } from '../batch-10/file-2030.js';
export interface Type2031 {
  id: 2031;
  name: 'File2031';
  next: Type2030;
}

export function make2031(): Type2031 {
  return { id: 2031, name: 'File2031', next: null as unknown as Type2030 };
}
