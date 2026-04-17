import type { Type2050 } from '../batch-10/file-2050.js';
export interface Type2051 {
  id: 2051;
  name: 'File2051';
  next: Type2050;
}

export function make2051(): Type2051 {
  return { id: 2051, name: 'File2051', next: null as unknown as Type2050 };
}
