import type { Type2401 } from '../batch-12/file-2401.js';
export interface Type2402 {
  id: 2402;
  name: 'File2402';
  next: Type2401;
}

export function make2402(): Type2402 {
  return { id: 2402, name: 'File2402', next: null as unknown as Type2401 };
}
