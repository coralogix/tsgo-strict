import type { Type2400 } from '../batch-12/file-2400.js';
export interface Type2401 {
  id: 2401;
  name: 'File2401';
  next: Type2400;
}

export function make2401(): Type2401 {
  return { id: 2401, name: 'File2401', next: null as unknown as Type2400 };
}
